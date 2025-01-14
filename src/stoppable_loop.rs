use std::sync::{Arc, Mutex};

/// Controls a loop that can be stopped from the outside, even if it is
/// in the middle of an iteration.
#[derive(Debug)]
pub struct StoppableLoop<A: LoopAction> {
    state: Arc<Mutex<State<A::Stop>>>,
    action: A,
}

#[derive(Debug, Default)]
enum State<Stoppable> {
    #[default]
    NotStarted,
    Running(Stoppable),
    Stopped,
}

pub trait LoopAction {
    type Stop: Send + Sync + 'static;
    type Wait;
    type Output;

    fn stop(stop: &Self::Stop);

    fn wait(wait: Self::Wait) -> Option<Self::Output>;

    fn start(&mut self) -> (Self::Stop, Self::Wait);
}

impl<S> State<S> {
    fn is_stopped(&self) -> bool {
        matches!(self, State::Stopped)
    }
}

impl<A: LoopAction> StoppableLoop<A> {
    pub fn new(action: A) -> Self {
        Self {
            state: Default::default(),
            action,
        }
    }

    /// Returns a closure that once called will stop the current loop, if it is
    /// running.
    pub fn get_stop(&self) -> impl FnOnce() + Send + 'static {
        // Clone the Arc to allow sending it to another thread.
        let state = self.state.clone();
        move || {
            let mut state = state.lock().unwrap();
            if let State::Running(stop) = &*state {
                A::stop(stop);
            }
            *state = State::Stopped;
        }
    }

    fn start_action(&mut self) -> Option<A::Wait> {
        let mut lock = self.state.lock().unwrap();
        let state = &mut *lock;
        if state.is_stopped() {
            return None;
        }
        let (stop, wait) = self.action.start();
        *state = State::Running(stop);
        Some(wait)
    }

    pub fn run(&mut self) -> Option<A::Output> {
        loop {
            let Some(wait) = self.start_action() else {
                return None;
            };

            let output = A::wait(wait);
            if self.state.lock().unwrap().is_stopped() {
                return None;
            }
            if let Some(output) = output {
                return Some(output);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct EasyTestAction;

    impl LoopAction for EasyTestAction {
        type Stop = ();
        type Wait = ();
        type Output = i32;

        fn start(&mut self) -> (Self::Stop, Self::Wait) {
            ((), ())
        }

        fn stop(_: &Self::Stop) {}

        fn wait(_: Self::Wait) -> Option<Self::Output> {
            Some(42)
        }
    }

    #[test]
    fn stoppable_loop_returns_on_success() {
        let mut loop_ = StoppableLoop::new(EasyTestAction);
        assert_eq!(loop_.run(), Some(42));
    }

    use std::sync::{Condvar, Mutex};
    struct StuckTestAction;

    impl LoopAction for StuckTestAction {
        type Stop = Arc<(Mutex<bool>, Condvar)>;
        type Wait = Arc<(Mutex<bool>, Condvar)>;
        type Output = ();

        fn start(&mut self) -> (Self::Stop, Self::Wait) {
            let pair = Arc::new((Mutex::new(false), Condvar::new()));
            (pair.clone(), pair)
        }

        fn stop(stop: &Self::Stop) {
            let (lock, cvar) = &**stop;
            let mut stopped = lock.lock().unwrap();
            *stopped = true;
            cvar.notify_all();
        }

        fn wait(wait: Self::Wait) -> Option<Self::Output> {
            let (lock, cvar) = &*wait;
            let mut stopped = lock.lock().unwrap();
            // Get stuck until the loop is stopped.
            while !*stopped {
                stopped = cvar.wait(stopped).unwrap();
            }
            None
        }
    }

    #[test]
    fn test_stoppable_loop_stop() {
        let mut loop_ = StoppableLoop::new(StuckTestAction);
        std::thread::sleep(std::time::Duration::from_millis(10));
        let stop = loop_.get_stop();
        stop();
        assert_eq!(loop_.run(), None);
    }
}
