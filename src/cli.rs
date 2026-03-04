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
    #[command(
        long_about = "Provision NixOS onto a remote machine using nixos-anywhere.\n\nSupported calling style (nh-style):\n  <flake_ref> -H <host_name> --target-host <target_host> [nixos-anywhere args...]\n  (or --hostname instead of -H)\n\nTianyi maps:\n  <flake_ref> + -H <host_name> -> --flake <flake_ref>#<host_name>\n  --target-host <target_host>  -> --target-host <target_host>\n\n`target_host` can be a normal SSH target or an SSH config host alias from ~/.ssh/config.\n\nTianyi-specific option:\n  --host-keys-dir <DIR>\n    Copies ssh_host_ed25519_key and ssh_host_ed25519_key.pub into a temporary\n    /etc/ssh tree and passes that temp directory via --extra-files.\n\nAll other flags are passed through unchanged to nixos-anywhere.\nYou do not need `--` before passthrough flags.",
        after_long_help = "Examples:\n  tianyi provision . -H my-host --target-host root@192.168.1.50 \\\n    --host-keys-dir ./secrets/ssh-host-keys/my-host \\\n    -i ~/.ssh/id_ed25519 --phases disko,install,reboot\n\n  tianyi provision . -H vishnu --target-host vishnu-deploy \\\n    -i ~/.ssh/id_ed25519 --disko-mode mount --debug\n\nAuth note:\n  Passing -i can still prompt for a key passphrase.\n  Without -i, SSH may use agent-loaded identities (ssh-agent/gpg-agent)."
    )]
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
    /// `<flake_ref> -H <host_name> --target-host <target_host> [--host-keys-dir DIR] [nixos-anywhere args...]`
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    pub args: Vec<String>,
}

#[derive(Args, Debug)]
pub struct AnywhereArgs {
    #[arg(required = true, trailing_var_arg = true, allow_hyphen_values = true)]
    pub args: Vec<String>,
}
