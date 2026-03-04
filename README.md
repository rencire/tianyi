# Goals
- Support common nix rebuild commands for nixos and darwin-nix via an easy-to-use interface.
e.g.

Replace:
```
 sudo darwin-rebuild switch --flake .#<hostname>
```

or

```
sudo nixos-rebuild -- switch --flake .#<hostname>
```

with:

```
ty switch .#<hostname>
```

# Quickstart
Try command via `nix run`:

e.g. rebuild host and switch with `my-host` configuration defined in `./flake.nix` file:
```
nix run github:rencire/tianyi -- switch .#my-host
```

# Installation
TODO

# Install NixOS Remotely
Install a new NixOS machine with `nixos-anywhere` through the `install` command:

```bash
nix run github:rencire/tianyi -- install .#<hostname> <target_host> /path/to/host-keys /path/to/facter.json
```

Optional: pass `-i` or `--identity` to force a specific SSH private key for the login user:

```bash
nix run github:rencire/tianyi -- install .#<hostname> <target_host> --identity ~/.ssh/<target-login-key> /path/to/host-keys /path/to/facter.json
```

Examples:

```bash
nix run github:rencire/tianyi -- install .#vm0 installer@installer.local ./secrets/vm0/ssh-host-keys ./nix/nixos/vm0/facter.json
```

```bash
nix run github:rencire/tianyi -- install .#vm0 installer@installer.local --identity ~/.ssh/installer_ed25519 ./secrets/vm0/ssh-host-keys ./nix/nixos/vm0/facter.json
```

`<target_host>` can be either:
- `installer@host` or another non-root user with passwordless `sudo`
- `root@host` if direct root SSH is enabled in your environment

If you pass `--identity`, `~/.ssh/<target-login-key>` must be a private key authorized for the account in `<target_host>`.
If you omit `--identity`, SSH defaults such as `~/.ssh/config`, SSH host aliases, or `ssh-agent` are used.

Recommended: use a dedicated install user with passwordless `sudo` and a dedicated SSH key, instead of enabling direct root SSH.

Why: this keeps the install flow compatible with the common hardening practice of disabling direct SSH access for `root`, while still allowing `nixos-anywhere` to run privileged install steps through `sudo`.

Supported authentication and privilege combinations:
- `root` + SSH password
- `root` + SSH key
- `user` + SSH password + passwordless `sudo`
- `user` + SSH key + passwordless `sudo`

Not supported for the normal non-root install path:
- `user` + SSH password + sudo password prompt
- `user` + SSH key + sudo password prompt

The host key directory must contain:
- `ssh_host_ed25519_key`
- `ssh_host_ed25519_key.pub`

# Notes


Better names:
- Commands:
  - build
    - --simulate
  - switch
    - --simulate-build
    - --simulate-activate
    - --simulate-all
  - activate
    - --simulate
