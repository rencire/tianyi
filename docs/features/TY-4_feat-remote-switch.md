# TY-4_feat-remote-switch.md
## Metadata
- **ID**: TY-4
- **Status**: Done
- **Depends**: TY-1, TY-2
- **Owner**: @rencire
- **Created**: 2025-09-13

## User Story
> *As a user,
> I want to easily deploy configurations to remote nixos machines,
> So I can update remote nixos machines with new configurations

## Specs
### Technical Requirements
- [x] Implement remote nixos configuration deployments for `switch` command

### Design Notes
Essentially, we want to replace:
```
nix run nixpkgs#nixos-rebuild -- --fast --flake .#my-host --target-host my-host-alias --use-remote-sudo switch
```
with:
```
nix run gihub.com:rencire/tianyi -- switch .#my-host my-host-alias
```

## PR Links

## History
- 2025-09-13: Done
- 2025-09-13: Draft created
