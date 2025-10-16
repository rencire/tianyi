use crate::cli::Cli;
use crate::{darwin, nixos};
use anyhow::Result;

pub fn execute(cli: Cli) -> Result<()> {
    match cli {
        Cli::Build { hostname } => build_command(hostname),
        Cli::Switch {
            hostname,
            target_host,
        } => switch_command(hostname, target_host),
        Cli::Activate { hostname } => activate_command(hostname),
    }
}

fn build_command(_hostname: String) -> Result<()> {
    todo!("implement build for darwin and nixos");
}

fn switch_command(hostname: String, target_host: Option<String>) -> Result<()> {
    match target_host {
        // Note: for now, assume we always want to deploy nixos system for remote deployments.
        Some(target) => {
            // Remote switch (NixOS)
            nixos::switch(&hostname, &target)
        }
        None => {
            // TODO determine if on nixos or darwin. if so, call the function from
            // the appropriate platform.
            // Local switch (Darwin)
            // For now, we assume local switch is always darwin
            darwin::switch(&hostname)
        }
    }
}

fn activate_command(_hostname: String) -> Result<()> {
    todo!("implement build for darwin and nixos");
}
