use anyhow::Result;
use std::process::Command;

pub fn execute_install(
    hostname: &str,
    target_host: &str,
    identity: &str,
    temp_path: &str,
    facter_json_path: &str,
) -> Result<()> {
    // TODO: debug why we need to enter passphrase for identity ssh key twice
    // when using this rust app, whereas when running nixos-anywhere directly
    //  in terminal, we only need to enter it once?
    // TODO: we should pin down this version of nixos-anywhere
    Command::new("nix")
        .arg("run")
        .arg("github:nix-community/nixos-anywhere")
        .arg("--")
        .arg("--flake")
        .arg(hostname)
        .arg("--target-host")
        .arg(target_host)
        .arg("-i")
        .arg(identity)
        .arg("--extra-files")
        .arg(temp_path)
        .arg("--generate-hardware-config")
        .arg("nixos-facter")
        .arg(facter_json_path)
        .status()?;

    // Command::new("nix")
    //     .arg("run")
    //     .arg("github:nix-community/nixos-anywhere")
    //     .arg("--")
    //     .arg("--flake")
    //     .arg(".#vm0")
    //     .arg("--target-host")
    //     .arg("root@installer.local")
    //     .arg("-i")
    //     .arg("/Users/ren/.ssh/homelab_installer")
    //     .arg("--extra-files")
    //     .arg("./tmp")
    //     .arg("--generate-hardware-config")
    //     .arg("nixos-facter")
    //     .arg("./nix/nixos/vm0/facter.json")
    //     .status()?;

    // TempDir automatically cleans up when dropped
    Ok(())
}

#[cfg(test)]
mod tests {
    // use super::*;
    // use std::fs;
    // use tempfile::TempDir;

    // #[test]
    // fn test_setup_with_nixos_anywhere_integration() -> Result<()> {
    //     let temp_keys = TempDir::new()?;
    //     let keys_dir = temp_keys.path();

    //     fs::write(keys_dir.join("ssh_host_ed25519_key"), "private_key_content")?;
    //     fs::write(
    //         keys_dir.join("ssh_host_ed25519_key.pub"),
    //         "public_key_content",
    //     )?;

    //     let private_key = TempDir::new()?;
    //     fs::write(private_key.path().join("id_ed25519"), "test_private_key")?;

    //     // This will fail at the nixos-anywhere step (command not found),
    //     // but proves the preparation worked
    //     let result = execute_install(
    //         ".#test-host",
    //         "test@localhost",
    //         keys_dir.to_str().unwrap(),
    //         // private_key.path().join("id_ed25519").to_str().unwrap(),
    //         // "./path/to/facter/json/parent/dir",
    //     );

    //     // We expect an error (nixos-anywhere not found), not a file setup error
    //     assert!(result.is_err());
    //     Ok(())
    // }
}
