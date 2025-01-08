use std::{io::{self, Write}, path::{Path, PathBuf}, sync::Mutex, time::Duration};
use crate::fuzzer::Fuzzer;
use std::thread;

#[derive(Debug)]
pub struct ProgramResult {
    // TODO: We should use `Vec<u8>` instead to support non-UTF-8 IO.
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
}

pub trait Runner {
    fn run(&mut self);
    fn run_with_input(&mut self, input: &str) -> Result<ProgramResult, String>;
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

/// Spawn a process with the given executable path and write the input to its
/// stdin.
fn spawn_with_stdin(executable: &Path, input: &[u8]) -> io::Result<std::process::Child> {
    // TODO: Can we limit Command::new to only absolute paths?
    let mut child = std::process::Command::new(executable)
        .stdin(std::process::Stdio::piped())
        .spawn()?;
    child.stdin.as_mut()
        .ok_or(io::Error::new(io::ErrorKind::Other, "could not open stdin pipe"))?
        .write_all(input)?;
    Ok(child)
}

impl<T: Fuzzer> Runner for DefaultRunner<T> {
    fn run(&mut self) {
        // TODO: Here we want to run over and over again, right? And return some metrics?
        
        let mut stop = false;
        let mut last_child: Mutex<Option<std::process::Child>> = None.into();
        let t = thread::spawn(move || {
            while !stop {
                let input = self.fuzzer.generate_input();
                // TODO: Switch to '?' instead of unwrap
                last_child;
                child.wait();
            }
        });

        thread::sleep(self.timeout);

        while !stop { }
        match self.run_with_input(&input) {
            Ok(result) => println!("Execution succeeded: {:?}", result),
            Err(e) => eprintln!("Execution failed: {:?}", e),
        }
    }

    fn run_with_input(&mut self, input: &str) -> Result<ProgramResult, String> {

        Ok(ProgramResult {
            stdout: "default output".to_string(),
            stderr: "default error".to_string(),
            exit_code: child.wait_timeout
        })
    }
}
