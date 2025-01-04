pub mod fuzzer;
pub mod runner;

// Re-export commonly used types and functions
pub use fuzzer::Fuzzer;
pub use runner::{Runner, ProgramResult, DefaultRunner};
