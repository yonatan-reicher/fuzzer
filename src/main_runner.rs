use crate::fuzzer::Fuzzer;
use crate::runner::{ProgramResult, Runner};
use crate::stoppable_loop::{LoopAction, StoppableLoop};
use shared_child::SharedChild;
use std::thread;
use std::{
    io::{self, Write},
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};

pub struct MainRunner<F: Fuzzer> {
    executable: PathBuf,
    timeout: Duration,
    fuzzer: F,
}

impl<T: Fuzzer> MainRunner<T> {
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
fn spawn_with_stdin(executable: &Path, input: &[u8]) -> io::Result<SharedChild> {
    // TODO: Can we limit Command::new to only absolute paths?
    let child = SharedChild::spawn(
        std::process::Command::new(executable)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null()),
    )?;
    child
        .take_stdin()
        .as_mut()
        .ok_or(io::Error::new(
            io::ErrorKind::Other,
            "could not open stdin pipe",
        ))?
        .write_all(input)?;
    Ok(child)
}

fn delay(delay: Duration, action: impl FnOnce() + Send + 'static) {
    thread::spawn(move || {
        thread::sleep(delay);
        action();
    });
}

struct RunnerLoopAction<'path, 'fuzzer, Fuzzer> {
    fuzzer: &'fuzzer mut Fuzzer,
    executable: &'path Path,
}

impl<'p, 'f, F: Fuzzer> LoopAction for RunnerLoopAction<'p, 'f, F> {
    type Stop = Arc<SharedChild>;
    type Wait = (Arc<SharedChild>, String);

    type Output = String;

    fn stop(child: &Self::Stop) {
        child.kill().expect("could not kill child process");
    }

    fn wait(child_and_input: Self::Wait) -> Option<Self::Output> {
        let (child, input) = child_and_input;
        let exit_status = child.wait().expect("could not wait for child process");
        if exit_status.success() {
            None
        } else {
            Some(input)
        }
    }

    fn start(&mut self) -> (Self::Stop, Self::Wait) {
        let input = self.fuzzer.generate_input();
        let child = spawn_with_stdin(self.executable, input.as_bytes())
            .expect("could not spawn child process");
        let child = Arc::new(child);
        (child.clone(), (child, input))
    }
}

impl<T: Fuzzer + Send> Runner for MainRunner<T> {
    fn run(&mut self) {
        let mut searcher = StoppableLoop::new(RunnerLoopAction {
            fuzzer: &mut self.fuzzer,
            executable: &self.executable,
        });

        let stop = searcher.get_stop();
        delay(self.timeout, stop);

        let input_found = searcher.run();
        match input_found {
            Some(result) => println!("Execution succeeded: {}", result),
            None => println!("Execution timed out"),
        }
    }

    fn run_with_input(&mut self, _input: &str) -> Result<ProgramResult, String> {
        todo!()
    }
}
