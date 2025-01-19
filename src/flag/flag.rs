use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

/// A little thread safe flag that can be raised and checked.
#[derive(Debug, Default)]
pub struct Flag(Arc<AtomicBool>);

impl Flag {
    /// Returns a closure that once called will raise the flag.
    /// Can be sent to another thread!
    pub fn get_raise(&self) -> impl Fn() + Send + 'static {
        let state = self.0.clone();
        move || {
            state.store(true, Ordering::Relaxed);
        }
    }

    pub fn is_raised(&self) -> bool {
        self.0.load(Ordering::Relaxed)
    }
}
