use std::{thread, time::Duration};

pub fn delay(delay: Duration, action: impl FnOnce() + Send + 'static) {
    thread::spawn(move || {
        thread::sleep(delay);
        action();
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::flag::Flag;

    #[test]
    fn test_delay() {
        let start = std::time::Instant::now();
        let was_called = Flag::default();
        delay(Duration::from_millis(100), was_called.get_raise());
        was_called.wait_until_raised();
    }
}
