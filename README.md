# Goals
- Provide one house CLI that wraps `nh` for rebuild flows and `nixos-anywhere` for installs.
- Keep Tianyi thin while still handling a few local workflow helpers such as SSH host key staging.

# Quickstart
Run `nh` through Tianyi:

```sh
nix run . -- os switch .#my-host
nix run . -- darwin switch .#my-mac
nix run . -- home switch .#my-home
nix run . -- search ripgrep
```

Run `nixos-anywhere` through Tianyi:

```sh
nix run . -- provision .#host root@example \
  --host-keys-dir ./ssh-host-keys/host \
  -i ~/.ssh/id_ed25519 \
  --generate-hardware-config nixos-facter ./hosts/host/facter.json \
  --phases disko,install,reboot
```

`provision` also supports nh-style host selection:

```sh
nix run . -- provision . -H vishnu --target-host vishnu-deploy \
  -i ~/.ssh/id_ed25519 --phases disko,install,reboot
```

In that example, `.` means “use the flake in the current directory”, and `-H vishnu` maps to host output `.#vishnu`.

Pass through extra `nixos-anywhere` args directly:

```sh
nix run . -- provision .#host root@example --debug
```

Or forward directly to `nixos-anywhere` without Tianyi provision helpers:

```sh
nix run . -- anywhere --phases disko,install,reboot --debug
```

# Command Mapping
Tianyi maps to the underlying tools like this:

```text
tianyi os <args...>        -> nh os <args...>
tianyi darwin <args...>    -> nh darwin <args...>
tianyi home <args...>      -> nh home <args...>
tianyi search <args...>    -> nh search <args...>
tianyi clean <args...>     -> nh clean <args...>
tianyi anywhere <args...>  -> nixos-anywhere <args...>
tianyi provision ...       -> nixos-anywhere with optional host key staging
```

The `nh`-backed commands are thin passthroughs. Whatever appears after the Tianyi subcommand is forwarded unchanged after the corresponding `nh` command group.

Examples:

```sh
tianyi os switch .#host
# runs: nh os switch .#host

tianyi home build .#home
# runs: nh home build .#home

tianyi search --json ripgrep
# runs: nh search --json ripgrep
```

`tianyi anywhere` is also a pure passthrough. Whatever appears after `anywhere` is forwarded unchanged to `nixos-anywhere`.

Examples:

```sh
tianyi anywhere --phases disko,install,reboot --debug
# runs: nixos-anywhere --phases disko,install,reboot --debug

tianyi anywhere --target-host root@example --flake .#host -i ~/.ssh/id_ed25519
# runs: nixos-anywhere --target-host root@example --flake .#host -i ~/.ssh/id_ed25519
```

`tianyi provision` is the structured helper. It always starts from:

```text
nixos-anywhere --flake <hostname> --target-host <target_host>
```

It also supports this nh-style mapping:

```text
tianyi provision <flake_ref> -H <host_name> --target-host <target_host> ...
  -> nixos-anywhere --flake <flake_ref>#<host_name> --target-host <target_host> ...
```

Common case:

```text
tianyi provision . -H vishnu --target-host vishnu-deploy ...
  -> nixos-anywhere --flake .#vishnu --target-host vishnu-deploy ...
```

Then Tianyi only adds one custom behavior:

```text
--host-keys-dir <dir>      -> stages keys into a temp directory and passes --extra-files <tempdir>
```

Everything else is passthrough to `nixos-anywhere` via trailing args.

Example:

```sh
tianyi provision .#host root@example \
  --host-keys-dir ./ssh-host-keys/host \
  -i ~/.ssh/id_ed25519 \
  --generate-hardware-config nixos-facter ./hosts/host/facter.json \
  --phases disko,install \
  --post-kexec-ssh-port 2222 \
  --debug
```

Maps to:

```text
nixos-anywhere \
  --flake .#host \
  --target-host root@example \
  --extra-files <temporary directory containing etc/ssh/ssh_host_ed25519_key*> \
  -i ~/.ssh/id_ed25519 \
  --generate-hardware-config nixos-facter ./hosts/host/facter.json \
  --phases disko,install \
  --post-kexec-ssh-port 2222 \
  --debug
```

# Notes
- `tianyi os ...` forwards to `nh os ...`
- `tianyi darwin ...` forwards to `nh darwin ...`
- `tianyi home ...` forwards to `nh home ...`
- `tianyi search ...` forwards to `nh search ...`
- `tianyi clean ...` forwards to `nh clean ...`
- `tianyi provision ...` calls `nixos-anywhere` directly
- `tianyi anywhere ...` forwards to `nixos-anywhere ...`
- `tianyi install ...` remains as an alias of `tianyi provision ...`
- Set `NIXOS_ANYWHERE_BIN` if you want Tianyi to use a specific `nixos-anywhere` binary path

# Troubleshooting
- **`-i` still prompts for passphrase:** using `-i` may still trigger passphrase prompts even if an agent is running.
- **Agent behavior differences:** without `-i`, SSH can rely on agent-loaded identities (`ssh-agent`/`gpg-agent`), which often avoids repeated prompts after the passphrase is cached.
- **Why this matters:** some execution paths may stage or invoke keys in ways that do not fully reuse prior agent-authenticated state, so `-i` can behave differently from relying on default SSH identity selection.

# Development
- Agentic PR flow: see `docs/agentic-flow.md`
