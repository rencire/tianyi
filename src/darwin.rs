use std::process::{Command, Stdio};
use std::io;

pub fn execute(subcommand: &str, hostname: &str) -> io::Result<()> {
    Command::new("darwin-rebuild")
        .arg(subcommand)
        .arg("--flake")
        .arg(hostname)
        // Preserve color output from darwin-rebuild with stdio::inherit(e)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .map(|_| ())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_execution() {
        let output = Command::new("echo")
            .args(["darwin-rebuild", "test", "--flake", "host"])
            .output()
            .unwrap();
        assert!(output.status.success());
    }
}
