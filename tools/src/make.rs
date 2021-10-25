use tools::{run, BadExit};

fn main() -> Result<(), BadExit> {
    // Build the release version.
    run!("cargo", "build", "--release", "--bin", "cli")
}
