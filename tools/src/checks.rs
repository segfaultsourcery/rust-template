use tools::{run, BadExit};

fn main() -> Result<(), BadExit> {
    // Run all tests.
    run!("cargo", "test", "--all")
}
