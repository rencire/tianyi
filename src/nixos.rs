use std::io;
use std::process::{Command, Stdio};

/// Handles remote NixOS commands (only `switch` for now)
pub fn execute(hostname: &str, target_host: &str) -> io::Result<()> {
    Command::new("nix")
        .args([
            "run",
            "nixpkgs#nixos-rebuild",
            "--",
            "--fast",
            "--flake",
            hostname,
            "--target-host",
            target_host,
            "--use-remote-sudo",
            "switch",
        ])
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .map(|_| ())
}
