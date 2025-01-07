use std::{env, time::Duration};
use std::path::PathBuf;
use std::process;
use fuzzer::{DefaultRunner, MainFuzzer, Runner};


#[derive(Debug)]
enum FuzzingMode {
    Strings,
    Urls,
}

impl FuzzingMode {
    fn from_arg(arg: &str) -> Result<Self, String> {
        match arg {
            "--strings" => Ok(FuzzingMode::Strings),
            "--urls" => Ok(FuzzingMode::Urls),
            _ => Err(format!("Invalid option: {}. Use --strings or --urls.", arg)),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} (--strings | --urls) <executable>", args[0]);
        process::exit(1);
    }

    let mode_arg = &args[1];
    let executable = &args[2];

    let mode = match FuzzingMode::from_arg(mode_arg) {
        Ok(mode) => mode,
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1);
        }
    };

    let executable = PathBuf::from(executable);
    if !executable.exists() {
        eprintln!("Executable not found: {:?}", executable);
        process::exit(1);
    }

    println!("Fuzzing mode: {:?}", mode);
    println!("Target executable: {:?}", executable);

    let mut runner = DefaultRunner::new(
        executable,
        // TODO: Get from CLI?
        Duration::from_secs(5),
        MainFuzzer::default(),
    );
    runner.run();
    // TODO: Report Metrics
}
