// nixos.rs
use anyhow::Result;
use std::process::{Command, Stdio};

// pub fn build(hostname: &str, target_host: &str) -> Result<()> {
//     execute_switch(hostname, target_host)
// }

pub fn switch(hostname: &str, target_host: &str) -> Result<()> {
    execute_switch(hostname, target_host)?;
    // TODO deploy age key only if flag is present
    // deploy_age_key(target_host)?;
    Ok(())
}

pub fn execute_switch(hostname: &str, target_host: &str) -> Result<()> {
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
        .status()?;
    Ok(())
}

// Handles remote NixOS commands with age key deployment
// pub fn execute_with_age_key_deploy(hostname: &str, target_host: &str) -> io::Result<()> {
//     // First, execute the configuration switch
//     if let Err(e) = execute_switch(hostname, target_host) {
//         eprintln!("Failed to switch configuration: {}", e);
//         return Err(e);
//     }

//     // If the switch is successful, deploy the age key
//     if let Err(e) = deploy_age_key(target_host) {
//         eprintln!("Failed to deploy age key: {}", e);
//         return Err(e);
//     }

//     Ok(())
// }

// /// Deploys the age key to the remote machine
// fn deploy_age_key(target_host: &str) -> io::Result<()> {
//     // TODO: make these locations configurable
//     let key_path = "~/.config/sops/age/keys.txt";
//     let remote_dir = "/etc/sops/age";
//     let remote_key_path = "/tmp/age-key.txt";

//     println!("Deploying age key to {}...", target_host);

//     // Create the remote directory if it doesn't exist
//     Command::new("ssh")
//         .arg(target_host)
//         .arg("sudo")
//         .arg("mkdir")
//         .arg("-p")
//         .arg(remote_dir)
//         .status()
//         .map(|_| ())?;

//     // Copy the age key file to the remote machine
//     Command::new("scp")
//         .arg(key_path)
//         .arg(format!("{}:{}", target_host, remote_key_path))
//         .status()
//         .map(|_| ())?;

//     // Move the key file to the final location and set permissions
//     Command::new("ssh")
//         .arg(target_host)
//         .arg("sudo")
//         .arg("mv")
//         .arg(remote_key_path)
//         .arg(format!("{}/keys.txt", remote_dir))
//         .arg("&&")
//         .arg("sudo")
//         .arg("chmod")
//         .arg("600")
//         .arg(format!("{}/keys.txt", remote_dir))
//         .status()
//         .map(|_| ())?;

//     println!("✅ Age key deployed to {}", target_host);
//     Ok(())
// }
