# CLI Help Output Examples

This document shows concrete examples of how Dotfilet's CLI help output should appear using standard ANSI colors. Each example demonstrates the semantic color usage and formatting defined in [CLI-Help-Design.md](CLI-Help-Design.md).

## Color Legend

For reference, here are the ANSI escape codes used:
- **BRAND** (Bright Blue): `\x1b[94m\x1b[1m`
- **COMMAND** (Blue): `\x1b[34m\x1b[1m`
- **SUBCOMMAND** (Magenta): `\x1b[35m\x1b[1m`
- **ARGUMENT** (Cyan): `\x1b[36m`
- **OPTION** (Yellow): `\x1b[33m`
- **OPTION_VALUE** (Bright Cyan): `\x1b[96m`
- **HEADER** (Bright White): `\x1b[97m\x1b[1m`
- **DESCRIPTION** (White): `\x1b[37m`
- **SUBTLE** (Bright Black): `\x1b[90m`
- **RESET**: `\x1b[0m`

## Example 1: Main Help Output

```
dotfilet - Declarative configuration management for developer environments

USAGE
  dotfilet <command> [options] [arguments]

CORE COMMANDS
  apply         Apply configuration to your system
  sync          Sync manual changes back to configuration
  validate      Validate configuration files
  status        Show current system state vs configuration

PLUGIN COMMANDS
  plugin        Manage plugins and their lifecycle
  agent         Run plugin agents for specific tasks

CONFIGURATION COMMANDS
  config        Manage dotfilet configuration
  template      Work with configuration templates

ADDITIONAL COMMANDS
  completion    Generate shell completion scripts
  doctor        Diagnose common issues
  version       Show version information

OPTIONS
  -h, --help         Show help for command
  -v, --verbose      Enable verbose output
  -q, --quiet        Suppress non-essential output
      --config       Path to configuration file
      --no-color     Disable colored output

EXAMPLES
  $ dotfilet apply
  $ dotfilet sync --dry-run
  $ dotfilet plugin list
  $ dotfilet status --verbose

LEARN MORE
  Use 'dotfilet <command> --help' for more information about a command.
  Visit https://github.com/nevir/dotfilet for documentation.
```

## Example 2: Command-Specific Help (dotfilet apply)

```
dotfilet apply - Apply configuration to your system

USAGE
  dotfilet apply [options] [profile]

DESCRIPTION
  Applies the configuration defined in your dotfilet files to the current
  system. This includes installing packages, configuring applications, and
  setting up your development environment.

ARGUMENTS
  profile          Optional profile name to apply (default: current system)

OPTIONS
  -h, --help       Show help for this command
  -n, --dry-run    Show what would be done without making changes
  -f, --force      Force apply even if no changes detected
  -v, --verbose    Show detailed progress information
      --only       Only apply specific plugins (comma-separated)
      --skip       Skip specific plugins (comma-separated)

EXAMPLES
  $ dotfilet apply
  $ dotfilet apply --dry-run
  $ dotfilet apply work-laptop
  $ dotfilet apply --only homebrew,git
  $ dotfilet apply --skip docker --verbose

LEARN MORE
  Use 'dotfilet status' to see what changes would be applied.
  Use 'dotfilet validate' to check configuration before applying.
```

## Example 3: Subcommand Help (dotfilet plugin)

```
dotfilet plugin - Manage plugins and their lifecycle

USAGE
  dotfilet plugin <subcommand> [options] [arguments]

SUBCOMMANDS
  list            List installed plugins
  install         Install a plugin
  update          Update plugin to latest version
  remove          Remove an installed plugin
  info            Show detailed plugin information
  enable          Enable a disabled plugin
  disable         Disable an active plugin

OPTIONS
  -h, --help      Show help for command
  -v, --verbose   Show detailed information

EXAMPLES
  $ dotfilet plugin list
  $ dotfilet plugin install homebrew
  $ dotfilet plugin update --all
  $ dotfilet plugin info git

LEARN MORE
  Use 'dotfilet plugin <subcommand> --help' for more information about a subcommand.
  Plugin documentation: https://github.com/nevir/dotfilet/wiki/plugins
```

## Example 4: Detailed Subcommand Help (dotfilet plugin install)

```
dotfilet plugin install - Install a plugin

USAGE
  dotfilet plugin install [options] <name> [version]

DESCRIPTION
  Installs a plugin from the official registry or from a custom source.
  Plugins extend dotfilet's functionality to manage specific applications
  and configurations.

ARGUMENTS
  name            Name of the plugin to install (required)
  version         Specific version to install (default: latest)

OPTIONS
  -h, --help      Show help for this command
  -f, --force     Force reinstall if already installed
      --source    Install from custom source (git URL or path)
      --branch    Git branch to install from (with --source)
  -v, --verbose   Show detailed installation progress

EXAMPLES
  $ dotfilet plugin install homebrew
  $ dotfilet plugin install git 1.2.3
  $ dotfilet plugin install custom --source https://github.com/user/plugin
  $ dotfilet plugin install local --source ./my-plugin

LEARN MORE
  Browse available plugins: https://github.com/nevir/dotfilet/wiki/plugins
  Create your own plugin: https://github.com/nevir/dotfilet/wiki/plugin-development
```

## Example 5: List Command Output (dotfilet plugin list)

```
INSTALLED PLUGINS

Active plugins:
  homebrew      1.2.1    Package manager for macOS
  git           2.0.0    Git configuration management
  zsh           1.5.2    Zsh shell configuration
  vscode        3.1.0    Visual Studio Code settings

Disabled plugins:
  docker        0.9.1    Docker configuration (disabled)

Updates available:
  git           2.0.0 → 2.1.0
  vscode        3.1.0 → 3.2.1

Use 'dotfilet plugin update' to install available updates.
Use 'dotfilet plugin info <name>' for detailed information about a plugin.
```

## Example 6: Error Context Help

```
Error: Unknown command 'aplpy'

Did you mean one of these?
  apply         Apply configuration to your system
  
Other available commands:
  sync          Sync manual changes back to configuration
  validate      Validate configuration files
  status        Show current system state

Use 'dotfilet --help' to see all available commands.
Use 'dotfilet <command> --help' for help with a specific command.
```

## Example 7: Validation Output with Colors

```
dotfilet validate - Configuration validation results

✓ Configuration syntax is valid
✓ All plugins are available
✓ No circular dependencies found
⚠ Warning: Plugin 'homebrew' has updates available (1.2.1 → 1.3.0)
⚠ Warning: Configuration references deprecated option 'git.use_ssh' (use 'git.protocol' instead)
✗ Error: Plugin 'nonexistent' is not installed

2 warnings, 1 error found.

Run 'dotfilet plugin update homebrew' to update plugins.
Run 'dotfilet apply --dry-run' to see what would be applied.
```

## Monochrome Fallback Examples

When colors are disabled (NO_COLOR=1 or non-TTY output):

```
dotfilet - Declarative configuration management for developer environments

USAGE
  dotfilet <command> [options] [arguments]

CORE COMMANDS
  apply         Apply configuration to your system
  sync          Sync manual changes back to configuration
  validate      Validate configuration files
  status        Show current system state vs configuration

OPTIONS
  -h, --help         Show help for command
  -v, --verbose      Enable verbose output
  -q, --quiet        Suppress non-essential output

EXAMPLES
  $ dotfilet apply
  $ dotfilet sync --dry-run
  $ dotfilet plugin list

LEARN MORE
  Use 'dotfilet <command> --help' for more information about a command.
```

## Narrow Terminal Adaptation (< 80 columns)

```
dotfilet - Declarative configuration management

USAGE
  dotfilet <command> [options] [arguments]

CORE COMMANDS
  apply
    Apply configuration to your system
  sync
    Sync manual changes back to configuration
  validate
    Validate configuration files

OPTIONS
  -h, --help
    Show help for command
  -v, --verbose
    Enable verbose output

EXAMPLES
  $ dotfilet apply
  $ dotfilet sync --dry-run

LEARN MORE
  Use 'dotfilet <command> --help' for more info.
```

## Implementation Notes

### Color Application in Code

Each example above would be generated using the semantic color functions:

```rust
// Example implementation pattern
format!(
    "{}{}\n\n{}{}\n  {}{} {}{} {}[options] [arguments]",
    BRAND, "dotfilet",
    DESCRIPTION, "Declarative configuration management for developer environments",
    HEADER, "USAGE",
    COMMAND, "dotfilet",
    ARGUMENT, "<command>",
    RESET
)
```

### Responsive Behavior

- **Wide terminals (120+ cols)**: Use full-width layout with aligned descriptions
- **Standard terminals (80-119 cols)**: Use compact aligned layout
- **Narrow terminals (< 80 cols)**: Stack descriptions below commands
- **Very narrow (< 60 cols)**: Truncate descriptions, use abbreviated text

### Accessibility Features

- All examples work perfectly in monochrome
- Screen readers will read the text naturally without color codes
- High contrast ensured by semantic color choices
- Bold text provides visual hierarchy without relying on color alone