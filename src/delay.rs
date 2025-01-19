use std::sync::{Arc, Condvar, Mutex, WaitTimeoutResult};
use std::thread;
use std::time::Duration;

pub fn delay(delay: Duration, action: impl FnOnce() + Send + 'static) {
    thread::spawn(move || {
        thread::sleep(delay);
        action();
    });
}

pub fn cancelable_delay(delay: Duration, action: impl FnOnce() + Send + 'static) -> impl FnOnce() {
    use crate::flag::WaitableFlag;

    let was_cancelled = WaitableFlag::default();
    let wait = was_cancelled.wait();
    thread::spawn(move || {
        let timeout_result = wait.until_raised_timeout(delay);
        if timeout_result.timed_out() {
            action();
        }
    });
    was_cancelled.get_raise()
}

#[derive(Debug)]
pub struct Delayer<A: Action> {
    state: Arc<State<A>>,
}

#[derive(Debug)]
struct State<A: Action> {
    request: Mutex<Option<Request<A>>>,
    request_arrived: Condvar,
    cancel: Mutex<bool>,
    cancel_set_true: Condvar,
}

#[derive(Debug)]
struct Request<A: Action> {
    delay: Duration,
    action: A,
}

trait Action: FnOnce() + Send + 'static {}
impl<T> Action for T where T: FnOnce() + Send + 'static {}

impl<A: Action> Delayer<A> {
    fn wait_and_pop_request(state: &State<A>) -> Request<A> {
        let request = state.request.lock().unwrap();
        let mut request = state.request_arrived.wait_while(request, |request| request.is_none()).unwrap();
        request.take().expect("This must have a value based on the condition above")
    }

    fn wait_timeout_or_cancel(state: &State<A>, delay: Duration) -> WaitTimeoutResult {
        let cancel = state.cancel.lock().unwrap();
        let (mut cancel, timeout_result) = state.cancel_set_true.wait_timeout_while(cancel, delay, |cancel| !*cancel).unwrap();
        *cancel = false;
        timeout_result
    }

    fn start_worker(state: Arc<State<A>>) {
        thread::spawn(move || {
            loop {
                let request = Self::wait_and_pop_request(&state);
                let timeout_result = Self::wait_timeout_or_cancel(&state, request.delay);
                if timeout_result.timed_out() {
                    (request.action)();
                }
            }
        });
    }

    pub fn new() -> Self {
        let state = Arc::new(State {
            request: Mutex::new(None),
            request_arrived: Condvar::new(),
            cancel: Mutex::new(false),
            cancel_set_true: Condvar::new(),
        });
        Self::start_worker(state.clone());
        Self { state }
    }

    /// Sets an action to be performed after a delay, cancelling the previous
    /// action if it was not yet performed.
    pub fn cancel_and_set(&self, delay: Duration, action: A) {
        self.cancel();
        self.set(delay, action);
    }

    pub fn cancel(&self) {
        let mut cancel = self.state.cancel.lock().unwrap();
        *cancel = true;
        self.state.cancel_set_true.notify_all();
    }

    pub fn set(&self, delay: Duration, action: A) {
        assert!(delay > Duration::from_secs(0));
        self.state.request.lock().unwrap().replace(Request { delay, action });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::flag::WaitableFlag;

    #[test]
    fn test_delay() {
        let duration = Duration::from_millis(100);
        let start = std::time::Instant::now();
        let was_called = WaitableFlag::default();
        delay(duration, was_called.get_raise());
        was_called.wait().until_raised();
        assert!(start.elapsed() >= duration);
    }

    #[test]
    fn test_delayer() {
        let duration = Duration::from_millis(100);
        let start = std::time::Instant::now();
        let was_called = WaitableFlag::default();
        let delayer = Delayer::new();
        delayer.set(duration, was_called.get_raise());
        was_called.wait().until_raised();
        assert!(start.elapsed() >= duration);
    }
}
