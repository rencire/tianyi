use anyhow::Result;
use std::process::{Command, Stdio};

fn command_binary() -> String {
    std::env::var("NIXOS_ANYWHERE_BIN").unwrap_or_else(|_| String::from("nixos-anywhere"))
}

fn build_install_command_for_binary(
    binary: &str,
    hostname: &str,
    target_host: &str,
    passthrough_args: &[String],
    extra_files_dir: Option<&str>,
) -> Command {
    let mut command = Command::new(binary);

    command
        .arg("--flake")
        .arg(hostname)
        .arg("--target-host")
        .arg(target_host);

    if let Some(extra_files_dir) = extra_files_dir {
        command.arg("--extra-files").arg(extra_files_dir);
    }

    command.args(passthrough_args);
    command
}

fn build_install_command(
    hostname: &str,
    target_host: &str,
    passthrough_args: &[String],
    extra_files_dir: Option<&str>,
) -> Command {
    build_install_command_for_binary(
        &command_binary(),
        hostname,
        target_host,
        passthrough_args,
        extra_files_dir,
    )
}

fn build_raw_command_for_binary(binary: &str, args: &[String]) -> Command {
    let mut command = Command::new(binary);
    command.args(args);
    command
}

fn build_raw_command(args: &[String]) -> Command {
    build_raw_command_for_binary(&command_binary(), args)
}

pub fn execute_provision(
    hostname: &str,
    target_host: &str,
    passthrough_args: &[String],
    extra_files_dir: Option<&str>,
) -> Result<()> {
    let status = build_install_command(hostname, target_host, passthrough_args, extra_files_dir)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()?;
    if !status.success() {
        anyhow::bail!("nixos-anywhere exited with status {}", status);
    }

    Ok(())
}

pub fn run_raw(args: &[String]) -> Result<()> {
    let status = build_raw_command(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()?;
    if !status.success() {
        anyhow::bail!("nixos-anywhere exited with status {}", status);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{
        build_install_command, build_install_command_for_binary, build_raw_command,
        build_raw_command_for_binary,
    };
    use crate::cli::Cli;
    use clap::Parser;
    use std::process::Command;

    fn command_args(command: &Command) -> Vec<String> {
        command
            .get_args()
            .map(|arg| arg.to_string_lossy().into_owned())
            .collect()
    }

    #[test]
    fn provision_accepts_passthrough_args() {
        let result = Cli::try_parse_from([
            "tianyi",
            "provision",
            ".#host",
            "root@example",
            "--host-keys-dir",
            "/tmp/keys",
            "--phases",
            "disko,install,reboot",
            "--debug",
        ]);

        assert!(result.is_ok());
    }

    #[test]
    fn builds_minimal_install_command() {
        let passthrough_args = Vec::new();
        let command = build_install_command(".#host", "root@example", &passthrough_args, None);

        assert_eq!(command.get_program().to_string_lossy(), "nixos-anywhere");
        assert_eq!(
            command_args(&command),
            vec!["--flake", ".#host", "--target-host", "root@example"]
        );
    }

    #[test]
    fn builds_install_command_with_all_supported_options() {
        let passthrough_args = vec![
            String::from("-i"),
            String::from("/tmp/id"),
            String::from("--generate-hardware-config"),
            String::from("nixos-facter"),
            String::from("/tmp/facter.json"),
            String::from("--phases"),
            String::from("disko,install,reboot"),
            String::from("--disko-mode"),
            String::from("format"),
            String::from("--kexec"),
            String::from("/tmp/kexec"),
            String::from("--debug"),
            String::from("--option"),
            String::from("accept-flake-config"),
            String::from("true"),
        ];
        let command = build_install_command(
            ".#host",
            "root@example",
            &passthrough_args,
            Some("/tmp/extra-files"),
        );

        assert_eq!(
            command_args(&command),
            vec![
                "--flake",
                ".#host",
                "--target-host",
                "root@example",
                "--extra-files",
                "/tmp/extra-files",
                "-i",
                "/tmp/id",
                "--generate-hardware-config",
                "nixos-facter",
                "/tmp/facter.json",
                "--phases",
                "disko,install,reboot",
                "--disko-mode",
                "format",
                "--kexec",
                "/tmp/kexec",
                "--debug",
                "--option",
                "accept-flake-config",
                "true",
            ]
        );
    }

    #[test]
    fn honors_configured_nixos_anywhere_binary() {
        let passthrough_args = Vec::new();
        let command = build_install_command_for_binary(
            "/nix/store/abc/bin/nixos-anywhere",
            ".#host",
            "root@example",
            &passthrough_args,
            None,
        );

        assert_eq!(
            command.get_program().to_string_lossy(),
            "/nix/store/abc/bin/nixos-anywhere"
        );
    }

    #[test]
    fn builds_raw_nixos_anywhere_command() {
        let args = vec![
            String::from("--phases"),
            String::from("disko,install"),
            String::from("--debug"),
        ];
        let command = build_raw_command_for_binary("nixos-anywhere", &args);

        assert_eq!(command.get_program().to_string_lossy(), "nixos-anywhere");
        assert_eq!(command_args(&command), args);
    }

    #[test]
    fn preserves_all_raw_nixos_anywhere_args_exactly() {
        let cases = [
            vec![
                String::from("--phases"),
                String::from("disko,install,reboot"),
                String::from("--debug"),
                String::from("--print-build-logs"),
            ],
            vec![
                String::from("--kexec"),
                String::from("/tmp/kexec"),
                String::from("--post-kexec-ssh-port"),
                String::from("2222"),
                String::from("--env-password"),
            ],
            vec![
                String::from("--disk-encryption-keys"),
                String::from("root"),
                String::from("/tmp/key"),
                String::from("--no-reboot"),
            ],
            vec![
                String::from("--target-host"),
                String::from("root@example"),
                String::from("--flake"),
                String::from(".#host"),
                String::from("-i"),
                String::from("/tmp/id"),
            ],
        ];

        for args in cases {
            let command = build_raw_command(&args);
            assert_eq!(command_args(&command), args);
        }
    }

    #[test]
    fn appends_host_key_extra_files_before_passthrough_args() {
        let passthrough_args = vec![
            String::from("-i"),
            String::from("/tmp/id"),
            String::from("--phases"),
            String::from("disko,install"),
            String::from("--post-kexec-ssh-port"),
            String::from("2222"),
            String::from("--debug"),
        ];
        let command = build_install_command(
            ".#host",
            "root@example",
            &passthrough_args,
            Some("/tmp/extra-files"),
        );

        assert_eq!(
            command_args(&command),
            vec![
                "--flake",
                ".#host",
                "--target-host",
                "root@example",
                "--extra-files",
                "/tmp/extra-files",
                "-i",
                "/tmp/id",
                "--phases",
                "disko,install",
                "--post-kexec-ssh-port",
                "2222",
                "--debug",
            ]
        );
    }
}
