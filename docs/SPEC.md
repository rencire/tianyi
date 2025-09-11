# Rebuild Command Specification
# Version: 1.1
# Systems: nix-darwin

## COMMANDS

### build <hostname>
Build system configuration without activation

OPTIONS:
  --simulate    Dry-run build process (no changes made)

### switch <hostname>
Build and activate configuration in one step

OPTIONS:
  --simulate-build     Dry-run only the build phase
  --simulate-activate  Dry-run only the activation phase
  --simulate-all       Dry-run both phases (build + activate)

### activate <hostname>
Activate an already-built configuration

OPTIONS:
  --simulate    Dry-run activation (no changes made)

## BEHAVIOR

### Command Implementation
- All commands delegate directly to `darwin-rebuild` with `--flake` option
- Example: `build myhost` → `darwin-rebuild build --flake myhost`

### Error Handling
- Errors are passed through directly from `darwin-rebuild`
- No custom error processing or filtering

### Exit Codes
0 = Success (or successful dry-run)
Non-zero = Error from `darwin-rebuild`

## EXAMPLES

# Check if configuration builds
ty build myhost --simulate

# Perform full activation
ty switch myhost

# Verify activation changes
ty activate myhost --simulate
