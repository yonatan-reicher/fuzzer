use std::time::Duration;
use crate::fuzzer::Fuzzer;

#[derive(Debug)]
pub struct ProgramResult {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
}

pub trait Runner {
    fn run(&self);
    fn run_with_input(&self, input: &str) -> Result<ProgramResult, String>;
}

#[allow(dead_code)]
pub struct DefaultRunner<F: Fuzzer> {
    executable: String,
    timeout: Duration,
    fuzzer: F,
}

impl<T: Fuzzer> DefaultRunner<T> {
    pub fn new(executable: String, timeout: Duration, fuzzer: T) -> Self {
        Self {
            executable,
            timeout,
            fuzzer,
        }
    }
}

impl<T: Fuzzer> Runner for DefaultRunner<T> {
    fn run(&self) {
        let input = self.fuzzer.generate_input();
        match self.run_with_input(&input) {
            Ok(result) => println!("Execution succeeded: {:?}", result),
            Err(e) => eprintln!("Execution failed: {:?}", e),
        }
    }

    fn run_with_input(&self, input: &str) -> Result<ProgramResult, String> {
        // Placeholder for running the executable with input
        Ok(ProgramResult {
            stdout: input.to_string(),
            stderr: "".to_string(),
            exit_code: 0,
        })
    }
}
