# Goals
- Provide one house CLI that wraps `nh` for rebuild flows and `nixos-anywhere` for installs.
- Keep Tianyi thin while still handling a few local workflow helpers such as SSH host key staging.

# Quickstart
Run `nh` through Tianyi:

```sh
nix run github:rencire/tianyi -- os switch . -H my-host
nix run github:rencire/tianyi -- darwin switch . -H my-mac
nix run github:rencire/tianyi -- home switch . -c alice@my-host
nix run github:rencire/tianyi -- search ripgrep
```

Run `nixos-anywhere` through Tianyi helpers (`provision`):

```sh
nix run github:rencire/tianyi -- provision . -H host --target-host root@example \
  --host-keys-dir ./ssh-host-keys/host \
  -i ~/.ssh/id_ed25519 \
  --generate-hardware-config nixos-facter ./hosts/host/facter.json \
  --phases disko,install,reboot
```

In that example, `.` means “use the flake in the current directory”, and `-H host` maps to host output `.#host`.

Pass through extra `nixos-anywhere` args directly:

```sh
nix run github:rencire/tianyi -- provision . -H host --target-host root@example --debug
```

Or use the direct passthrough proxy (`anywhere`) without Tianyi helpers:

```sh
nix run github:rencire/tianyi -- anywhere \
  --flake .#host \
  --target-host root@example \
  --phases disko,install,reboot \
  --debug
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
tianyi os switch . -H host
# runs: nh os switch . -H host

tianyi darwin switch . -H mac
# runs: nh darwin switch . -H mac

tianyi home build . -c alice@host
# runs: nh home build . -c alice@host

tianyi search --json ripgrep
# runs: nh search --json ripgrep
```

`tianyi anywhere` is also a pure passthrough. Whatever appears after `anywhere` is forwarded unchanged to `nixos-anywhere`.

Examples:

```sh
tianyi anywhere --flake .#host --target-host root@example --phases disko,install,reboot --debug
# runs: nixos-anywhere --flake .#host --target-host root@example --phases disko,install,reboot --debug

tianyi anywhere --target-host root@example --flake .#host -i ~/.ssh/id_ed25519
# runs: nixos-anywhere --target-host root@example --flake .#host -i ~/.ssh/id_ed25519
```

`tianyi provision` is the structured helper and accepts nh-style input only:

```text
tianyi provision <flake_ref> -H <host_name> --target-host <target_host> [nixos-anywhere args...]
```

It maps that input to this `nixos-anywhere` command shape:

```text
nixos-anywhere --flake <flake_ref>#<host_name> --target-host <target_host>
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
tianyi provision . -H host --target-host root@example \
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
- `tianyi provision ...` maps nh-style host selection to `nixos-anywhere` and can stage host keys via `--host-keys-dir`
- `tianyi anywhere ...` forwards to `nixos-anywhere ...`
- Set `NIXOS_ANYWHERE_BIN` if you want Tianyi to use a specific `nixos-anywhere` binary path

# Troubleshooting
- **`-i` still prompts for passphrase:** using `-i` may still trigger passphrase prompts even if an agent is running.
- **Agent behavior differences:** without `-i`, SSH can rely on agent-loaded identities (`ssh-agent`/`gpg-agent`), which often avoids repeated prompts after the passphrase is cached.
- **Why this matters:** some execution paths may stage or invoke keys in ways that do not fully reuse prior agent-authenticated state, so `-i` can behave differently from relying on default SSH identity selection.

# Development
- Agentic PR flow: see `docs/agentic-flow.md`
