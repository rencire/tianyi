use anyhow::Result;
use std::process::{Command, Stdio};

// pub fn build(hostname: &str) -> Result<()> {
//     execute_darwin("build", hostname)
// }

pub fn switch(hostname: &str) -> Result<()> {
    execute_darwin("switch", hostname)
}

// pub fn activate(hostname: &str) -> Result<()> {
//     execute_darwin("activate", hostname)
// }
// TODO change this interface to accept CLI enums
fn execute_darwin(subcommand: &str, hostname: &str) -> Result<()> {
    Command::new("darwin-rebuild")
        .arg(subcommand)
        .arg("--flake")
        .arg(hostname)
        // Preserve color output from darwin-rebuild with stdio::inherit(e)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()?;
    // If above line doesn't throw an error, return OK
    Ok(())
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
