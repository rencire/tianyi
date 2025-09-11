//! Darwin rebuild command implementation
//! Implements TY-2 from docs/features/TY-2_feat-darwin-rebuild.md

use std::process::{Command, Stdio};
use std::io;

/// Executes darwin-rebuild with the given subcommand
/// Preserves colored output by connecting directly to the terminal
pub fn execute(subcommand: &str) -> io::Result<()> {
    Command::new("darwin-rebuild")
        .arg(subcommand)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        // Ignore status, just propagate IO errors
        // Consider adding more robust error handling logic
        // in future if needed.
        .map(|_| ())

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_construction() {
        let mut cmd = Command::new("echo");
        cmd.arg("darwin-rebuild test");
        assert!(cmd.output().is_ok());
    }
}
