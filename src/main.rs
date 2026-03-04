mod cli;
mod commands;
mod darwin;
mod nixos;
mod nixos_anywhere;

use crate::cli::Cli;
use clap::Parser;

fn main() {
    let parsed = Cli::parse();
    if let Err(e) = commands::execute(parsed) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn verify_cli() {
        Cli::command().debug_assert();
    }

    #[test]
    fn test_hostname_required() {
        let cases = ["build", "switch", "activate"];

        for cmd in cases {
            let result = Cli::try_parse_from(&["tianyi", cmd]);
            assert!(
                result.is_err(),
                "{} should require hostname but didn't",
                cmd
            );
        }
    }

    #[test]
    fn test_valid_commands() {
        let cases = [
            ("build", "myhost"),
            ("switch", "myhost"),
            ("activate", "myhost"),
        ];

        for (cmd, hostname) in cases {
            let result = Cli::try_parse_from(&["tianyi", cmd, hostname]);
            assert!(
                result.is_ok(),
                "{} with hostname should be valid but wasn't",
                cmd
            );
        }
    }

    #[test]
    fn test_install_without_identity_is_valid() {
        let result = Cli::try_parse_from([
            "tianyi",
            "install",
            ".#my-host",
            "installer@example",
            "./host-keys",
            "./facter.json",
        ]);
        assert!(result.is_ok(), "install without identity should be valid");
    }

    #[test]
    fn test_install_with_identity_is_valid() {
        let result = Cli::try_parse_from([
            "tianyi",
            "install",
            ".#my-host",
            "installer@example",
            "--identity",
            "~/.ssh/installer_ed25519",
            "./host-keys",
            "./facter.json",
        ]);
        assert!(result.is_ok(), "install with identity should be valid");
    }
}
