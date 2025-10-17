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

    /// Deploy build to new machine
    Deploy {
        hostname: String,
        target_host: String,
        /// Path to directory containing SSH host keys
        #[arg(long, short = 'k')]
        ssh_host_key_files: String,
        /// Path to SSH private key for authentication
        #[arg(long, short = 'i')]
        ssh_private_key_file: String,
    },
}
