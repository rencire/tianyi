use clap::{Args, Parser};

#[derive(Parser, Debug)]
#[command(version, about = "Tianyi wrapper for nh and nixos-anywhere")]
pub enum Cli {
    /// Forward to `nh os`
    Os(NhArgs),
    /// Forward to `nh darwin`
    Darwin(NhArgs),
    /// Forward to `nh home`
    Home(NhArgs),
    /// Forward to `nh search`
    Search(NhArgs),
    /// Forward to `nh clean`
    Clean(NhArgs),
    /// Provision NixOS onto a remote machine with optional Tianyi helpers
    #[command(alias = "install")]
    Provision(ProvisionArgs),
    /// Forward any arguments directly to `nixos-anywhere`
    Anywhere(AnywhereArgs),
}

#[derive(Args, Debug)]
pub struct NhArgs {
    #[arg(required = true, trailing_var_arg = true, allow_hyphen_values = true)]
    pub args: Vec<String>,
}

#[derive(Args, Debug)]
pub struct ProvisionArgs {
    /// Raw provision args:
    /// `<hostname> <target_host> [--host-keys-dir DIR] [nixos-anywhere args...]`
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    pub args: Vec<String>,
}

#[derive(Args, Debug)]
pub struct AnywhereArgs {
    #[arg(required = true, trailing_var_arg = true, allow_hyphen_values = true)]
    pub args: Vec<String>,
}
