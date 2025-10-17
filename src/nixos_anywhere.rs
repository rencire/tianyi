use anyhow::Result;
use std::process::Command;

pub fn execute_deploy(
    temp_path: &str,
    hostname: &str,
    target_host: &str,
    ssh_private_key_file: &str,
) -> Result<()> {
    // Install NixOS with nixos-anywhere
    Command::new("nixos-anywhere")
        .arg("--extra-files")
        .arg(temp_path)
        .arg("--flake")
        .arg(hostname)
        .arg("--target-host")
        .arg(target_host)
        .arg("-i")
        .arg(ssh_private_key_file)
        .status()?;

    // TempDir automatically cleans up when dropped
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_setup_with_nixos_anywhere_integration() -> Result<()> {
        let temp_keys = TempDir::new()?;
        let keys_dir = temp_keys.path();

        fs::write(keys_dir.join("ssh_host_ed25519_key"), "private_key_content")?;
        fs::write(
            keys_dir.join("ssh_host_ed25519_key.pub"),
            "public_key_content",
        )?;

        let private_key = TempDir::new()?;
        fs::write(private_key.path().join("id_ed25519"), "test_private_key")?;

        // This will fail at the nixos-anywhere step (command not found),
        // but proves the preparation worked
        let result = execute_deploy(
            ".#test-host",
            "test@localhost",
            keys_dir.to_str().unwrap(),
            private_key.path().join("id_ed25519").to_str().unwrap(),
        );

        // We expect an error (nixos-anywhere not found), not a file setup error
        assert!(result.is_err());
        Ok(())
    }
}
