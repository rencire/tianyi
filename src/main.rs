use clap::Parser;
mod darwin;

#[derive(Parser, Debug)]
#[command(version, about = "Tianyi rebuild manager")]
enum Cli {
    /// Build system configuration
    Build { hostname: String },
    /// Build and activate configuration
    Switch { hostname: String },
    /// Activate existing build
    Activate { hostname: String },
}

fn main() {
    let (subcommand, hostname) = match Cli::parse() {
        Cli::Build { hostname } => ("build", hostname),
        Cli::Switch { hostname } => ("switch", hostname),
        Cli::Activate { hostname } => ("activate", hostname),
    };

    if let Err(e) = darwin::execute(subcommand, &hostname) {
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
