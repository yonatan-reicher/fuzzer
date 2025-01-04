use std::time::Duration;
use std::path::PathBuf;
use fuzzer::{Fuzzer, Runner,DefaultRunner};

// Mock implementation of Fuzzer for the test
struct MockFuzzer;

impl Fuzzer for MockFuzzer {
    fn generate_input(&self) -> String {
        "Hello, world!\n".to_string()
    }
}

#[test]
fn test_runner_with_echo() {
    // Get the manifest directory
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");

    // Construct the full path to the echo.py script
    let mut executable_path = PathBuf::from(manifest_dir);
    executable_path.push("resources/test/echo.py");

    // Verify the file exists
    assert!(
        executable_path.exists(),
        "Executable file does not exist: {:?}",
        executable_path
    );

    let executable = executable_path.to_str().expect("Invalid executable path").to_string();
    let fuzzer = MockFuzzer;
    let runner = DefaultRunner::new(executable, Duration::from_secs(5), fuzzer);

    // Run the `run_with_input` directly for testing
    let input = "Hello, world!\n";
    let result = runner.run_with_input(input).expect("Failed to run the program");

    // Validate results
    println!("{}", result.exit_code);
    assert_eq!(result.exit_code, 0, "Program did not exit with code 0");
    assert_eq!(result.stdout.trim(), input.trim(), "Unexpected output");
    assert!(result.stderr.is_empty(), "Program wrote to stderr");
}
