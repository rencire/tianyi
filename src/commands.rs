use crate::darwin; // Imports from darwin.rs
use crate::nixos;
use std::io;

/// Public API: Routes to darwin (local) or nixos (remote)
pub fn execute(command: &str, hostname: &str, target_host: Option<&str>) -> io::Result<()> {
    match (command, target_host) {
        // For now, all switch commands with a hostname assume remote nixos "switch",
        // since we do not need to deploy to remote darwin machines.
        (_, Some(target)) => nixos::execute(command, hostname, target), // Remote
        _ => darwin::execute(command, hostname),                        // Local
    }
}
