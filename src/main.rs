mod cli;
mod commands;
mod nh;
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
    fn test_wrapper_commands_require_args() {
        let cases = ["os", "darwin", "home", "search", "clean", "anywhere"];

        for cmd in cases {
            let result = Cli::try_parse_from(["tianyi", cmd]);
            assert!(
                result.is_err(),
                "{} should require downstream args but didn't",
                cmd
            );
        }
    }

    #[test]
    fn test_valid_commands() {
        let cases = [
            vec!["tianyi", "os", "switch", ".#myhost"],
            vec!["tianyi", "darwin", "switch", ".#myhost"],
            vec!["tianyi", "home", "switch", ".#home"],
            vec!["tianyi", "search", "ripgrep"],
            vec!["tianyi", "clean", "all"],
            vec![
                "tianyi",
                "provision",
                ".#myhost",
                "root@example",
                "--host-keys-dir",
                "./keys/host",
                "-i",
                "~/.ssh/id_ed25519",
                "--phases",
                "disko,install,reboot",
            ],
            vec![
                "tianyi",
                "anywhere",
                "--debug",
                "--phases",
                "disko,install,reboot",
            ],
        ];

        for args in cases {
            let result = Cli::try_parse_from(args);
            assert!(result.is_ok(), "command should parse but didn't");
        }
    }

    #[test]
    fn test_install_command_removed() {
        let result = Cli::try_parse_from([
            "tianyi",
            "install",
            ".#myhost",
            "root@example",
            "--phases",
            "disko,install,reboot",
        ]);

        assert!(result.is_err(), "install alias should not parse");
    }
}
