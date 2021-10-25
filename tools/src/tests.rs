//! Common tests for the entire project.
//! This can include checking for formatting errors,
//! licence compatibility, etc.

use super::{run, BadExit};

#[test]
fn test_formatting() {
    assert!(
        run!("cargo", "fmt", "--all", "--", "--check").is_ok(),
        "The code is not properly formatted."
    );
}
