use std::{
    sync::{Arc, Condvar, Mutex},
    time::Duration,
};

/// A little thread safe flag that can be raised and checked, and waited on!
#[derive(Debug, Default)]
pub struct WaitableFlag(Arc<(Mutex<bool>, Condvar)>);

impl WaitableFlag {
    /// Returns a closure that once called will raise the flag.
    /// Can be sent to another thread!
    pub fn get_raise(&self) -> impl Fn() + Send + 'static {
        let state = self.0.clone();
        move || {
            *state.0.lock().unwrap() = true;
            state.1.notify_all();
        }
    }

    pub fn is_raised(&self) -> bool {
        *self.0 .0.lock().unwrap()
    }

    pub fn wait(&self) -> Wait {
        Wait(self.0.clone())
    }
}

#[derive(Debug)]
pub struct Wait(Arc<(Mutex<bool>, Condvar)>);

impl Wait {
    pub fn until_raised(&self) {
        let _lock = self
            .0
             .1
            .wait_while(self.0 .0.lock().unwrap(), |raised| !*raised)
            .unwrap();
    }

    pub fn until_raised_timeout(&self, timeout: Duration) -> std::sync::WaitTimeoutResult {
        let (_lock, timeout_result) = self
            .0
             .1
            .wait_timeout_while(self.0 .0.lock().unwrap(), timeout, |raised| !*raised)
            .unwrap();
        timeout_result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::delay::delay;
    use std::time::Duration;

    #[test]
    fn test_waitable_flag_gets_notified() {
        let flag = WaitableFlag::default();
        assert!(!flag.is_raised());
        delay(Duration::from_millis(10), flag.get_raise());
        flag.wait().until_raised_timeout(Duration::from_millis(20));
        assert!(flag.is_raised());
    }

    #[test]
    fn test_waitable_flag_doesnt_get_notified() {
        let flag = WaitableFlag::default();
        assert!(!flag.is_raised());
        flag.wait().until_raised_timeout(Duration::from_millis(10));
        assert!(!flag.is_raised());
    }
}
