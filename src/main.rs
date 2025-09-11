use clap::Parser;
mod darwin;

#[derive(Parser)]
#[command(version, about = "Tianyi rebuild manager")]
struct Cli {
    #[command(subcommand)]
    command: Subcommand,
}

/// Supported subcommands matching SPEC.md
#[derive(clap::Subcommand)]
enum Subcommand {
    /// Build system configuration
    Build,
    /// Build and activate configuration
    Switch,
    /// Activate existing build
    Activate,
}

fn main() {
    let subcommand = match Cli::parse().command {
        Subcommand::Build => "build",
        Subcommand::Switch => "switch",
        Subcommand::Activate => "activate",
    };

    if let Err(e) = darwin::execute(subcommand) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
