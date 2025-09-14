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
