use crate::cli::Cli;
use crate::{darwin, nixos, nixos_anywhere};
use anyhow::Result;
use std::fs;
use std::path::Path;
use tempfile::TempDir;

struct DeployContext {
    temp_dir: TempDir,
}

fn install_command(
    hostname: &str,
    target_host: &str,
    identity: &str,
    host_keys_dir: &str,
    facter_json_path: &str,
) -> Result<()> {
    let context = prepare_deploy_context(host_keys_dir)?;
    let temp_path = context.temp_dir.path().to_str().unwrap();
    // nixos_anywhere::execute_install(hostname, target_host, identity)?;
    nixos_anywhere::execute_install(hostname, target_host, identity, temp_path, facter_json_path)?;
    Ok(())
}

pub fn execute(cli: Cli) -> Result<()> {
    match cli {
        Cli::Build { hostname } => build_command(hostname),
        Cli::Switch {
            hostname,
            target_host,
        } => switch_command(hostname, target_host),
        Cli::Activate { hostname } => activate_command(hostname),
        Cli::Install {
            hostname,
            target_host,
            identity,
            host_keys_dir,
            facter_json_path,
        } => install_command(
            &hostname,
            &target_host,
            &identity,
            &host_keys_dir,
            &facter_json_path,
        ),
    }
}

fn build_command(_hostname: String) -> Result<()> {
    todo!("implement build for darwin and nixos");
}

fn switch_command(hostname: String, target_host: Option<String>) -> Result<()> {
    match target_host {
        // Note: for now, assume we always want to deploy nixos system for remote deployments.
        // TODO this 'switch' should really be deploy
        Some(target) => {
            // Remote switch (NixOS)
            nixos::switch(&hostname, &target)
        }
        None => {
            // TODO determine if on nixos or darwin. if so, call the function from
            // the appropriate platform.
            // Local switch (Darwin)
            // For now, we assume local switch is always darwin
            darwin::switch(&hostname)
        }
    }
}

fn activate_command(_hostname: String) -> Result<()> {
    todo!("implement build for darwin and nixos");
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
    use std::fs;

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
}
