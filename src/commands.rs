use crate::darwin; // Imports from darwin.rs
use crate::nixos;
use std::io;

/// Public API: Routes to darwin (local) or nixos (remote)
pub fn execute(command: &str, hostname: &str, target_host: Option<&str>) -> io::Result<()> {
    match (command, target_host) {
        ("switch", Some(target)) => nixos::execute(hostname, target), // Remote
        _ => darwin::execute(command, hostname),                      // Local
    }
}
