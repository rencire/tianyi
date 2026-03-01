use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about = "Tianyi rebuild manager")]
pub enum Cli {
    /// Build system configuration
    Build { hostname: String },
    /// Build and activate configuration
    Switch {
        /// Flake reference (e.g., ".#my-host" or "myhost")
        hostname: String,
        /// (Optional) Remote target for deployment. Can be:
        /// - An SSH target (e.g., "user@host" or "host")
        /// - An SSH config host alias (e.g., "my-server" from ~/.ssh/config)
        /// If omitted, we will use darwin-rebuild for the hostname on local machine.
        #[arg(required = false)]
        target_host: Option<String>,
    },

    /// Activate existing build
    Activate { hostname: String },

    /// Install nixos onto a new remote machine
    Install {
        hostname: String,
        target_host: String,
        /// Path to SSH private key for nixos-anywhere to use for authentication to perform installation.
        identity: String,
        /// Path to directory containing SSH host keys we will copy over to target machine.
        host_keys_dir: String,
        /// Path to local filesystem where we want to store the facter.json generated from nixos-facter.
        facter_json_path: String,
    },
}
