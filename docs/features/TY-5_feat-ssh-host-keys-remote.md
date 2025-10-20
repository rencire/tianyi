# TY-5_feat-ssh-host-keys-remote.md
## Metadata
- **ID**: TY-5
- **Status**: Done
- **Depends**: TY-5
- **Owner**: @rencire
- **Created**: 2025-09-23

## User Story
> *As a user,
> I want to easily install configurations to remote nixos machines with ssh host key,
> So I can decrypt secrets in my nix configuration and use them for services like wifi.

## Specs
### Technical Requirements
- [] Add support for copying ssh host keys to remote machine via remote deployments with `nixos-anywhere` command

e.g.
```
tianyi install \
  .#hostA \
  user@hostA \
  ~/.ssh/my_ssh_private_key \
  ./ssh_host_keys/hostA/ \
  ./path/to/facter.json

```

We can base it off of this script:
```bash
#!/usr/bin/env bash

hostname=$1
target_host=$2
ssh_host_key_files=$3
ssh_private_key_file=$4

# Create a temporary directory
temp=$(mktemp -d)

# Create the directory where sshd expects to find the host keys
install -d -m755 "$temp/etc/ssh"

# Copy keys to the temporary directory
# Use hard-coded key names for now
cp $ssh_host_key_files/ssh_host_ed25519_key "$temp/etc/ssh/"
cp $ssh_host_key_files/ssh_host_ed25519_key.pub "$temp/etc/ssh/"

# Set the correct permissions so sshd will accept the key
chmod 600 "$temp/etc/ssh/ssh_host_ed25519_key"
chmod 644 "$temp/etc/ssh/ssh_host_ed25519_key"

# Install NixOS to the host system with our secrets
nixos-anywhere --extra-files "$temp" --flake $hostname --target-host $target_host -i $ssh_private_key_file

rm -rf "$temp"

```
