mod cli;
mod commands;
mod nh;
mod nixos_anywhere;

use anyhow::Result;
use clap::Parser;
pub use cli::Cli;
use std::ffi::OsString;

#[cfg(test)]
use clap::CommandFactory;

pub fn run(cli: Cli) -> Result<()> {
    commands::execute(cli)
}

pub fn run_args<I, S>(args: I) -> Result<()>
where
    I: IntoIterator<Item = S>,
    S: Into<OsString>,
{
    let cli = Cli::try_parse_from(
        std::iter::once(OsString::from("tianyi")).chain(args.into_iter().map(Into::into)),
    )?;
    run(cli)
}

#[cfg(test)]
mod tests {
    use super::*;

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
                ".",
                "-H",
                "myhost",
                "--target-host",
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

    #[test]
    fn test_install_help_command_removed() {
        let result = Cli::try_parse_from(["tianyi", "install", "--help"]);
        assert!(
            result.is_err(),
            "install help alias should not parse because install command is removed"
        );
    }
}
