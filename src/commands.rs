use crate::cli::{Cli, ProvisionArgs};
use crate::{nh, nixos_anywhere};
use anyhow::{anyhow, Result};
use std::fs;
use std::path::Path;
use tempfile::TempDir;

struct DeployContext {
    temp_dir: TempDir,
}

struct ParsedProvisionArgs {
    hostname: String,
    target_host: String,
    host_keys_dir: Option<String>,
    passthrough_args: Vec<String>,
}

fn parse_provision_args(args: ProvisionArgs) -> Result<ParsedProvisionArgs> {
    if args.args.is_empty() {
        return Err(anyhow!(
            "provision requires at least a flake reference argument"
        ));
    }

    let flake_ref = args.args[0].clone();
    let mut host_name = None;
    let mut target_host = None;
    let mut host_keys_dir = None;
    let mut passthrough_args = Vec::new();

    let mut index = 1;
    while index < args.args.len() {
        match args.args[index].as_str() {
            "--host-keys-dir" => {
                if index + 1 >= args.args.len() {
                    return Err(anyhow!("--host-keys-dir requires a value"));
                }
                host_keys_dir = Some(args.args[index + 1].clone());
                index += 2;
            }
            "-H" | "--hostname" => {
                if index + 1 >= args.args.len() {
                    return Err(anyhow!("{} requires a value", args.args[index]));
                }
                host_name = Some(args.args[index + 1].clone());
                index += 2;
            }
            "--target-host" => {
                if index + 1 >= args.args.len() {
                    return Err(anyhow!("--target-host requires a value"));
                }
                target_host = Some(args.args[index + 1].clone());
                index += 2;
            }
            _ => {
                passthrough_args.push(args.args[index].clone());
                index += 1;
            }
        }
    }

    let host_name = host_name.ok_or_else(|| {
        anyhow!("provision requires -H/--hostname <HOST_NAME> (nh-style only)")
    })?;
    let target_host = target_host.ok_or_else(|| {
        anyhow!("provision requires --target-host <HOST> (nh-style only)")
    })?;

    if flake_ref.contains('#') {
        return Err(anyhow!(
            "provision expects <flake_ref> without '#'; -H/--hostname selects the host output"
        ));
    }
    let hostname = format!("{}#{}", flake_ref, host_name);

    Ok(ParsedProvisionArgs {
        hostname,
        target_host,
        host_keys_dir,
        passthrough_args,
    })
}

fn provision_command(args: ProvisionArgs) -> Result<()> {
    let parsed = parse_provision_args(args)?;

    let deploy_context = match parsed.host_keys_dir.as_deref() {
        Some(host_keys_dir) => Some(prepare_deploy_context(host_keys_dir)?),
        None => None,
    };
    let extra_files_dir = deploy_context
        .as_ref()
        .and_then(|context| context.temp_dir.path().to_str());

    nixos_anywhere::execute_provision(
        &parsed.hostname,
        &parsed.target_host,
        &parsed.passthrough_args,
        extra_files_dir,
    )?;
    Ok(())
}

pub fn execute(cli: Cli) -> Result<()> {
    match cli {
        Cli::Os(args) => nh::run("os", &args.args),
        Cli::Darwin(args) => nh::run("darwin", &args.args),
        Cli::Home(args) => nh::run("home", &args.args),
        Cli::Search(args) => nh::run("search", &args.args),
        Cli::Clean(args) => nh::run("clean", &args.args),
        Cli::Provision(args) => provision_command(args),
        Cli::Anywhere(args) => nixos_anywhere::run_raw(&args.args),
    }
}

// See: https://github.com/nix-community/nixos-anywhere/blob/main/docs/howtos/extra-files.md
fn prepare_deploy_context(host_keys_dir: &str) -> Result<DeployContext> {
    // Create a temporary directory
    let temp_dir = TempDir::new()?;
    let temp_path = temp_dir.path();

    // Create the directory where sshd expects to find the host keys
    let ssh_dir = temp_path.join("etc/ssh");
    fs::create_dir_all(&ssh_dir)?;

    // Copy keys to the temporary directory
    let host_key_src = Path::new(host_keys_dir).join("ssh_host_ed25519_key");
    let host_key_pub_src = Path::new(host_keys_dir).join("ssh_host_ed25519_key.pub");

    let host_key_dst = ssh_dir.join("ssh_host_ed25519_key");
    let host_key_pub_dst = ssh_dir.join("ssh_host_ed25519_key.pub");

    // TODO check that these files exist for above paths

    fs::copy(&host_key_src, &host_key_dst)?;
    fs::copy(&host_key_pub_src, &host_key_pub_dst)?;

    // Set the correct permissions so sshd will accept the key
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&host_key_dst, fs::Permissions::from_mode(0o600))?;
        fs::set_permissions(&host_key_pub_dst, fs::Permissions::from_mode(0o644))?;
    }

    #[cfg(not(unix))]
    {
        eprintln!("Warning: Cannot set Unix file permissions on this platform");
    }

    Ok(DeployContext { temp_dir })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::ProvisionArgs;
    use std::fs;

    #[test]
    fn test_parse_provision_args_with_host_keys_dir() -> Result<()> {
        let parsed = parse_provision_args(ProvisionArgs {
            args: vec![
                String::from("."),
                String::from("-H"),
                String::from("host"),
                String::from("--target-host"),
                String::from("root@example"),
                String::from("--host-keys-dir"),
                String::from("/tmp/keys"),
                String::from("-i"),
                String::from("/tmp/id"),
                String::from("--phases"),
                String::from("disko,install,reboot"),
            ],
        })?;

        assert_eq!(parsed.hostname, ".#host");
        assert_eq!(parsed.target_host, "root@example");
        assert_eq!(parsed.host_keys_dir, Some(String::from("/tmp/keys")));
        assert_eq!(
            parsed.passthrough_args,
            vec![
                String::from("-i"),
                String::from("/tmp/id"),
                String::from("--phases"),
                String::from("disko,install,reboot"),
            ]
        );
        Ok(())
    }

    #[test]
    fn test_parse_provision_args_with_nh_style_host_name_and_target() -> Result<()> {
        let parsed = parse_provision_args(ProvisionArgs {
            args: vec![
                String::from("."),
                String::from("-H"),
                String::from("vishnu"),
                String::from("--target-host"),
                String::from("vishnu-deploy"),
                String::from("-i"),
                String::from("/tmp/id"),
            ],
        })?;

        assert_eq!(parsed.hostname, ".#vishnu");
        assert_eq!(parsed.target_host, "vishnu-deploy");
        assert_eq!(
            parsed.passthrough_args,
            vec![String::from("-i"), String::from("/tmp/id"),]
        );
        Ok(())
    }

    #[test]
    fn test_parse_provision_args_with_long_hostname_alias() -> Result<()> {
        let parsed = parse_provision_args(ProvisionArgs {
            args: vec![
                String::from("."),
                String::from("--hostname"),
                String::from("vishnu"),
                String::from("--target-host"),
                String::from("vishnu-deploy"),
            ],
        })?;

        assert_eq!(parsed.hostname, ".#vishnu");
        assert_eq!(parsed.target_host, "vishnu-deploy");
        Ok(())
    }

    #[test]
    fn test_parse_provision_args_without_host_keys_dir() -> Result<()> {
        let parsed = parse_provision_args(ProvisionArgs {
            args: vec![
                String::from("."),
                String::from("-H"),
                String::from("host"),
                String::from("--target-host"),
                String::from("root@example"),
                String::from("--debug"),
            ],
        })?;

        assert_eq!(parsed.host_keys_dir, None);
        assert_eq!(parsed.passthrough_args, vec![String::from("--debug")]);
        Ok(())
    }

    #[test]
    fn test_prepare_deploy_context_success() -> Result<()> {
        let temp_keys = TempDir::new()?;
        let keys_dir = temp_keys.path();

        // Create dummy SSH key files
        fs::write(keys_dir.join("ssh_host_ed25519_key"), "private_key_content")?;
        fs::write(
            keys_dir.join("ssh_host_ed25519_key.pub"),
            "public_key_content",
        )?;

        let context = prepare_deploy_context(keys_dir.to_str().unwrap())?;

        let ssh_dir = context.temp_dir.path().join("etc/ssh");
        // Verify the ssh_dir was created
        assert!(ssh_dir.exists());
        assert!(ssh_dir.join("ssh_host_ed25519_key").exists());
        assert!(ssh_dir.join("ssh_host_ed25519_key.pub").exists());

        Ok(())
    }

    #[test]
    fn test_prepare_deploy_context_missing_source_keys() -> Result<()> {
        let result = prepare_deploy_context("/nonexistent/path");

        // Should fail because source keys don't exist
        assert!(result.is_err());
        Ok(())
    }

    #[test]
    #[cfg(unix)]
    fn test_prepare_deploy_context_permissions() -> Result<()> {
        use std::os::unix::fs::PermissionsExt;

        let temp_keys = TempDir::new()?;
        let keys_dir = temp_keys.path();

        fs::write(keys_dir.join("ssh_host_ed25519_key"), "private_key_content")?;
        fs::write(
            keys_dir.join("ssh_host_ed25519_key.pub"),
            "public_key_content",
        )?;

        let context = prepare_deploy_context(keys_dir.to_str().unwrap())?;
        let ssh_dir = context.temp_dir.path().join("etc/ssh");

        // Check private key permissions are 0o600
        let private_key_perms = fs::metadata(ssh_dir.join("ssh_host_ed25519_key"))?
            .permissions()
            .mode()
            & 0o777;
        assert_eq!(private_key_perms, 0o600);

        // Check public key permissions are 0o644
        let public_key_perms = fs::metadata(ssh_dir.join("ssh_host_ed25519_key.pub"))?
            .permissions()
            .mode()
            & 0o777;
        assert_eq!(public_key_perms, 0o644);

        Ok(())
    }

    #[test]
    fn test_parse_provision_args_missing_required_args() {
        let result = parse_provision_args(ProvisionArgs {
            args: vec![String::from(".#host")],
        });
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_provision_args_errors_for_flake_ref_with_hash() {
        let result = parse_provision_args(ProvisionArgs {
            args: vec![
                String::from(".#host"),
                String::from("-H"),
                String::from("vishnu"),
                String::from("--target-host"),
                String::from("vishnu-deploy"),
            ],
        });
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_provision_args_rejects_direct_style_target_host() {
        let result = parse_provision_args(ProvisionArgs {
            args: vec![
                String::from("."),
                String::from("root@example"),
                String::from("--debug"),
            ],
        });
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_provision_args_requires_target_host_flag() {
        let result = parse_provision_args(ProvisionArgs {
            args: vec![
                String::from("."),
                String::from("-H"),
                String::from("host"),
                String::from("--debug"),
            ],
        });
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_provision_args_missing_host_keys_dir_value() {
        let result = parse_provision_args(ProvisionArgs {
            args: vec![
                String::from("."),
                String::from("-H"),
                String::from("host"),
                String::from("--target-host"),
                String::from("root@example"),
                String::from("--host-keys-dir"),
            ],
        });
        assert!(result.is_err());
    }
}
