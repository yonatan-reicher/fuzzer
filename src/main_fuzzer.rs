use crate::Fuzzer;

#[derive(Debug, Default, Clone)]
pub struct MainFuzzer {
    /// TODO: This is just a placeholder implementation 😌
    state: u64,
}

impl Fuzzer for MainFuzzer {
    fn generate_input(&mut self) -> String {
        // TODO: Should we handle what happens when we run for a very long
        // time, so much so that we overflow the `u64` type?
        self.state += 1;
        format!("{}", self.state)
    }
}

