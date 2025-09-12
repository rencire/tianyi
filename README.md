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
ty switch <hostname>
```

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
