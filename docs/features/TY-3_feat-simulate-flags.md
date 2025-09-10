# TY-3_feat-simulate-flags.md
## Metadata
- **ID**: TY-3
- **Status**: Draft
- **Depends**: TY-1, TY-2
- **Owner**: @rencire
- **Created**: 2025-09-09

## User Story
> *As a cautious user,
> I want `--simulate-*` flags as defined in SPEC.md,
> So I can validate changes before applying them.*

## Specs
### Technical Requirements
- [ ] Implement `--simulate` for `build` command
- [ ] Implement `--simulate-build` for `switch` command
- [ ] Implement `--simulate-activate` for `switch` command
- [ ] Implement `--simulate-all` for `switch` command

### Design Notes
- Must match behavior defined in SPEC.md
- Dry-runs should show diff of planned changes
- Should work for both NixOS and nix-darwin

## PR Links

## History
- 2025-09-09: Draft created
