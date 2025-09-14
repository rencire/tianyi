use clap::Parser;
mod commands;
mod darwin;
mod nixos;

#[derive(Parser, Debug)]
#[command(version, about = "Tianyi rebuild manager")]
enum Cli {
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
}

fn main() {
    let (subcommand, hostname, target_host) = match Cli::parse() {
        Cli::Build { hostname } => ("build", hostname, None),
        Cli::Switch {
            hostname,
            target_host,
        } => ("switch", hostname, target_host),
        Cli::Activate { hostname } => ("activate", hostname, None),
    };

    if let Err(e) = commands::execute(subcommand, &hostname, target_host.as_deref()) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn verify_cli() {
        Cli::command().debug_assert();
    }

    #[test]
    fn test_hostname_required() {
        let cases = ["build", "switch", "activate"];

        for cmd in cases {
            let result = Cli::try_parse_from(&["tianyi", cmd]);
            assert!(
                result.is_err(),
                "{} should require hostname but didn't",
                cmd
            );
        }
    }

    #[test]
    fn test_valid_commands() {
        let cases = [
            ("build", "myhost"),
            ("switch", "myhost"),
            ("activate", "myhost"),
        ];

        for (cmd, hostname) in cases {
            let result = Cli::try_parse_from(&["tianyi", cmd, hostname]);
            assert!(
                result.is_ok(),
                "{} with hostname should be valid but wasn't",
                cmd
            );
        }
    }
}
