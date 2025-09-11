use clap::Parser;
mod darwin;

#[derive(Parser)]
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
