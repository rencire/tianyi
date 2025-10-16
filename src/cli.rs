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
    // TODO add Deploy command enum here
}
