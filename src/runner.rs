use crate::fuzzer::Fuzzer;
use std::{path::PathBuf, time::Duration};

#[derive(Debug)]
pub struct ProgramResult {
    pub stdout: Vec<u8>,
    pub stderr: String,
    pub exit_code: i32,
}

pub trait Runner {
    fn run(&mut self);
    // TODO: Remove this?
    fn run_with_input(&mut self, input: &[u8]) -> Result<ProgramResult, String>;
}

#[allow(dead_code)]
pub struct DefaultRunner<F: Fuzzer> {
    executable: PathBuf,
    timeout: Duration,
    fuzzer: F,
}

impl<T: Fuzzer> DefaultRunner<T> {
    pub fn new(executable: PathBuf, timeout: Duration, fuzzer: T) -> Self {
        Self {
            executable,
            timeout,
            fuzzer,
        }
    }
}

impl<T: Fuzzer> Runner for DefaultRunner<T> {
    fn run(&mut self) {
        let input = self.fuzzer.generate_input();
        match self.run_with_input(&input) {
            Ok(result) => println!("Execution succeeded: {:?}", result),
            Err(e) => eprintln!("Execution failed: {:?}", e),
        }
    }

    fn run_with_input(&mut self, input: &[u8]) -> Result<ProgramResult, String> {
        // Placeholder for running the executable with input
        Ok(ProgramResult {
            stdout: input.to_vec(),
            stderr: "".to_string(),
            exit_code: 0,
        })
    }
}
