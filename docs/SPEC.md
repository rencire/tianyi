# Rebuild Command Specification
# Version: 1.0
# Systems: NixOS, nix-darwin

## COMMANDS

### build
Build system configuration without activation

OPTIONS:
  --simulate    Dry-run build process (no changes made)

### switch
Build and activate configuration in one step

OPTIONS:
  --simulate-build     Dry-run only the build phase
  --simulate-activate  Dry-run only the activation phase
  --simulate-all       Dry-run both phases (build + activate)

### activate
Activate an already-built configuration

OPTIONS:
  --simulate    Dry-run activation (no changes made)

## BEHAVIOR

### Dry-run Guarantees
- No system modifications when any --simulate* flag is used
- Symlinks (/run/current-system, /nix/var/nix/profiles/system) remain unchanged
- No services are restarted or modified

### Command Equivalence
switch = build + activate
switch --simulate-all = build --simulate + activate --simulate

### Exit Codes
0 = Success (or successful dry-run)
1 = Error (configuration error or execution failure)

## EXAMPLES

# Check if configuration builds
ty build --simulate

# Test full activation flow
ty switch --simulate-all

# Verify service changes
ty activate --simulate
