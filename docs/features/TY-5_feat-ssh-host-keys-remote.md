# TY-5_feat-ssh-host-keys-remote.md
## Metadata
- **ID**: TY-5
- **Status**: Ready
- **Depends**: TY-4
- **Owner**: @rencire
- **Created**: 2025-09-23

## User Story
> *As a user,
> I want to easily deploy configurations to remote nixos machines with ssh host key,
> So I can decrypt secrets in my nix configuration and use them for services like wifi.

## Specs
### Technical Requirements
- [] Add support for copying ssh host keys to remote machine via remote deployments with `switch` command

e.g.
```
tianyi switch .#hostA user@hostA —ssh-host-keys ./ssh_host_keys/hostA/
```

We can base it off of this script:
```bash
#!/usr/bin/env bash

# Create a temporary directory
temp=$(mktemp -d)

# Function to cleanup temporary directory on exit
cleanup() {
  rm -rf "$temp"
}
trap cleanup EXIT

# Create the directory where sshd expects to find the host keys
install -d -m755 "$temp/etc/ssh"

# Copy keys to the temporary directory
pass ssh_host_ed25519_key > "$temp/etc/ssh/"

# Set the correct permissions so sshd will accept the key
chmod 600 "$temp/etc/ssh/ssh_host_ed25519_key"

# Install NixOS to the host system with our secrets
nixos-anywhere --extra-files "$temp" --flake '.#your-host' --target-host root@yourip -i $ssh_host_private_key_file
```
