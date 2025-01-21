use fuzzer::{FuzzingMode, MainFuzzer, MainRunner, Runner};
use std::path::PathBuf;
use std::process;
use std::{env, time::Duration};

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} (--strings | --urls) <executable>", args[0]);
        process::exit(1);
    }

    let mode_arg = &args[1];
    let executable = &args[2];

    let mode = FuzzingMode::from_arg(mode_arg)?;

    let executable = PathBuf::from(executable);
    if !executable.exists() {
        eprintln!("Executable not found: {:?}", executable);
        process::exit(1);
    }

    println!("Fuzzing mode: {:?}", mode);
    println!("Target executable: {:?}", executable);

    let fuzzer = MainFuzzer::new(mode);
    

    let mut runner = MainRunner::new(
        executable,
        // TODO: Get from CLI?
        Duration::from_secs(5),
        MainFuzzer::default(),
    );
    runner.run();
    // TODO: Report Metrics

    Ok(())
}
