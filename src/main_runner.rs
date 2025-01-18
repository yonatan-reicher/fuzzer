use crate::fuzzer::Fuzzer;
use crate::runner::{ProgramResult, Runner};
use crate::stoppable_loop::{LoopAction, StoppableLoop};
use shared_child::SharedChild;
use std::thread;
use std::{
    fmt::{self, Display, Formatter},
    io::{self, Write},
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};
use crate::flag::Flag;

/// This runner takes a fuzzer, a total timeout, and an executable path. It
/// runs the executable with the input generated by the fuzzer until the total
/// timeout is reached.
///
/// Every execution has it's own individual timeout of `SINGLE_EXECUTION_TIMEOUT_SECS`.
pub struct MainRunner<F: Fuzzer> {
    executable: PathBuf,
    timeout: Duration,
    fuzzer: F,
    single_execution_timeout: Duration,
}

const SINGLE_EXECUTION_TIMEOUT_SECS: f32 = 1.5;

impl<T: Fuzzer> MainRunner<T> {
    pub fn new(executable: PathBuf, timeout: Duration, fuzzer: T) -> Self {
        Self {
            executable,
            timeout,
            fuzzer,
            single_execution_timeout: Duration::from_secs_f32(SINGLE_EXECUTION_TIMEOUT_SECS),
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
    single_execution_timeout: Duration,
}

fn stop(child: &Arc<SharedChild>) {
    child.kill().expect("could not kill child process");
}

fn wait<F>(
    runner: &mut RunnerLoopAction<F>,
    child: Arc<SharedChild>,
    input: Vec<u8>,
) -> Option<Vec<u8>> {
    match wait_with_timeout(child.clone(), runner.single_execution_timeout) {
        WaitWithTimeoutResult::Timeout => None,
        WaitWithTimeoutResult::Finished(exit_status) => {
            if exit_status.success() {
                None
            } else {
                // Only return the input if the execution failed, and not
                // because of a timeout!
                Some(input)
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum WaitWithTimeoutResult {
    Finished(std::process::ExitStatus),
    Timeout,
}

fn wait_with_timeout(child: Arc<SharedChild>, timeout: Duration) -> WaitWithTimeoutResult {
    let was_killed = Flag::default();
    kill_after_timeout(child.clone(), timeout, was_killed.get_raise());
    let exit_status = child.wait().expect("could not wait for child process");
    if was_killed.is_raised() {
        WaitWithTimeoutResult::Timeout
    } else {
        WaitWithTimeoutResult::Finished(exit_status)
    }
}

fn kill_after_timeout(
    child: Arc<SharedChild>,
    duration: Duration,
    on_killed: impl FnOnce() + Send + 'static,
) {
    delay(duration, move || {
        let exit_status = child.try_wait().expect("could not wait for child process");
        let is_alive = exit_status.is_none();
        if is_alive {
            child.kill().expect("could not kill child process");
            on_killed();
        }
    });
}

impl<'p, 'f, F: Fuzzer> LoopAction for RunnerLoopAction<'p, 'f, F> {
    type Stop = Arc<SharedChild>;
    type Wait = (Arc<SharedChild>, Vec<u8>);

    type Output = Vec<u8>;

    fn stop(child: &Self::Stop) {
        stop(child);
    }

    fn wait(&mut self, (child, input): Self::Wait) -> Option<Self::Output> {
        wait(self, child, input)
    }

    fn start(&mut self) -> (Self::Stop, Self::Wait) {
        let input = self.fuzzer.generate_input();
        let child =
            spawn_with_stdin(self.executable, &input).expect("could not spawn child process");
        let child = Arc::new(child);
        (child.clone(), (child, input))
    }
}

struct InputFoundPrinter(Vec<u8>);

impl Display for InputFoundPrinter {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let bytes = &self.0;
        if let Ok(string) = std::str::from_utf8(bytes) {
            write!(f, "{}", string)
        } else {
            write!(f, "{:?}", bytes)
        }
    }
}

impl<T: Fuzzer + Send> Runner for MainRunner<T> {
    fn run(&mut self) {
        let mut searcher = StoppableLoop::new(RunnerLoopAction {
            fuzzer: &mut self.fuzzer,
            executable: &self.executable,
            single_execution_timeout: self.single_execution_timeout,
        });

        let stop = searcher.get_stop();
        delay(self.timeout, stop);

        let input_found = searcher.run();
        match input_found {
            Some(result) => println!(
                "Execution succeeded. Output: '{}'",
                InputFoundPrinter(result)
            ),
            None => println!("Execution timed out"),
        }
    }

    fn run_with_input(&mut self, _input: &[u8]) -> Result<ProgramResult, String> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kill_after_timeout_kills_after_timeout() {
        let start = std::time::Instant::now();
        let child = Arc::new(SharedChild::spawn(std::process::Command::new("sleep").arg("10000")).unwrap());
        let was_killed = Flag::default();
        kill_after_timeout(child.clone(), Duration::from_millis(100), was_killed.get_raise());
        let exit_status = child.wait().unwrap();
        let elapsed = start.elapsed();
        let was_killed = was_killed.is_raised();
        println!("Elapsed: {:?}", elapsed);
        let max_error = 30;
        assert!(elapsed < Duration::from_millis(100 + max_error));
        assert!(!exit_status.success());
        assert!(was_killed);
    }

    #[test]
    fn test_kill_after_timeout_kills_doesnt_kill_before_timeout() {
        let duration = Duration::from_millis(100);
        let start = std::time::Instant::now();
        let child = Arc::new(SharedChild::spawn(&mut std::process::Command::new("echo")).unwrap());
        let was_killed = Flag::default();
        kill_after_timeout(child.clone(), duration, was_killed.get_raise());
        let exit_status = child.wait().unwrap();
        let elapsed = start.elapsed();
        let was_killed = was_killed.is_raised();
        println!("Elapsed: {:?}", elapsed);
        assert!(elapsed < duration);
        assert!(exit_status.success());
        assert!(!was_killed);
    }
}
