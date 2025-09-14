# TY-2_feat-darwin-rebuild.md
## Metadata
- **ID**: TY-2
- **Status**: Done
- **Depends**: TY-1
- **Owner**: @rencire
- **Created**: 2025-09-09

## User Story
> *As a macOS user,
> I want equivalent `darwin-rebuild` commands to NixOS,
> So I can manage my system with the same workflow.*

## Specs
### Technical Requirements
- [x] Implement `build` command
- [x] Implement `switch` command
- [x] Implement `activate` command

### Design Notes
The commands should map to darwin-rebuild commands, with `--flake`
option.
- `build <hostname>` maps to `build --flake <hostname>`
- `switch <hostname>` maps to `switch --flake <hostname>`
- `activate <hostname>` maps to `activate --flake <hostname>`


See: [/docs/SPEC.md#commands](/docs/SPEC.md#commands)

## PR Links

## Related
- [TY-1](/docs/features/TY-1_feat-nixos-rebuild.md)

## History
- 2025-09-12: Done 
- 2025-09-09: Draft created
