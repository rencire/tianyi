use anyhow::{bail, Result};
use std::process::{Command, Stdio};

fn build_command(command_group: &str, args: &[String]) -> Command {
    let mut command = Command::new("nh");
    command.arg(command_group).args(args);
    command
}

pub fn run(command_group: &str, args: &[String]) -> Result<()> {
    let status = build_command(command_group, args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()?;

    if status.success() {
        Ok(())
    } else {
        bail!("nh {} exited with status {}", command_group, status)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn command_args(command: &Command) -> Vec<String> {
        command
            .get_args()
            .map(|arg| arg.to_string_lossy().into_owned())
            .collect()
    }

    #[test]
    fn builds_nh_os_command() {
        let args = vec![
            String::from("switch"),
            String::from(".#host"),
            String::from("--"),
            String::from("--ask"),
        ];
        let command = build_command("os", &args);

        assert_eq!(command.get_program().to_string_lossy(), "nh");
        assert_eq!(
            command_args(&command),
            vec!["os", "switch", ".#host", "--", "--ask"]
        );
    }

    #[test]
    fn builds_nh_non_os_commands() {
        let cases = [
            ("darwin", vec![String::from("build"), String::from(".#mac")]),
            ("home", vec![String::from("switch"), String::from(".#home")]),
            ("search", vec![String::from("hello")]),
            ("clean", vec![String::from("all")]),
        ];

        for (group, args) in cases {
            let command = build_command(group, &args);
            let mut expected = vec![group.to_string()];
            expected.extend(args);

            assert_eq!(command.get_program().to_string_lossy(), "nh");
            assert_eq!(command_args(&command), expected);
        }
    }

    #[test]
    fn builds_nh_with_global_flags() {
        let args = vec![
            String::from("--verbose"),
            String::from("switch"),
            String::from(".#host"),
        ];
        let command = build_command("os", &args);

        assert_eq!(command.get_program().to_string_lossy(), "nh");
        assert_eq!(
            command_args(&command),
            vec!["os", "--verbose", "switch", ".#host"]
        );
    }

    #[test]
    fn preserves_all_nh_args_exactly() {
        let cases = [
            (
                "os",
                vec![
                    String::from("--quiet"),
                    String::from("build"),
                    String::from(".#host"),
                    String::from("--"),
                    String::from("--show-trace"),
                ],
            ),
            (
                "darwin",
                vec![
                    String::from("--elevation-program"),
                    String::from("sudo"),
                    String::from("repl"),
                    String::from(".#mac"),
                ],
            ),
            (
                "home",
                vec![
                    String::from("switch"),
                    String::from(".#home"),
                    String::from("--"),
                    String::from("--dry-run"),
                ],
            ),
            (
                "search",
                vec![
                    String::from("--json"),
                    String::from("--limit"),
                    String::from("5"),
                    String::from("--channel"),
                    String::from("nixos-unstable"),
                    String::from("--platforms"),
                    String::from("ripgrep"),
                ],
            ),
            (
                "clean",
                vec![
                    String::from("profile"),
                    String::from("/nix/var/nix/profiles/system"),
                ],
            ),
        ];

        for (group, args) in cases {
            let command = build_command(group, &args);
            let mut expected = vec![group.to_string()];
            expected.extend(args);
            assert_eq!(command_args(&command), expected);
        }
    }
}
