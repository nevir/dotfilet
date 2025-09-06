# CLI Help Output Examples

This document provides concrete examples of how the Dotfilet CLI help design should appear in practice, applying the design tokens and hierarchy defined in [CLI-Help-Design.md](./CLI-Help-Design.md).

> [!NOTE]
>
> Color annotations in this document use `[color:text]` notation to indicate the intended styling. In actual output, these would be rendered with ANSI color codes.

## Main Help Output (`dotfilet --help`)

```
[brand:dotfilet] [muted:v0.1.0]
[primary:Declarative configuration management for developer environments]

[secondary:USAGE]
  [command:dotfilet] [argument:<command>] [option:[subcommand]] [option:[flags]]

[secondary:CORE COMMANDS]
  [command:init]:      [primary:Initialize a new dotfilet repository]
  [command:diff]:      [primary:Show configuration changes that would be applied]
  [command:apply]:     [primary:Apply configuration to the current machine]

[secondary:SYNC AGENT COMMANDS]
  [command:agent]:     [primary:Manage the bidirectional sync agent]

[secondary:PLUGIN COMMANDS]
  [command:plugin]:    [primary:Manage dotfilet plugins]

[secondary:UTILITY COMMANDS]
  [command:version]:   [primary:Show version information]
  [command:config]:    [primary:Manage dotfilet configuration and variables]

[secondary:FLAGS]
  [option:--help]:     [primary:Show help for command]
  [option:--version]:  [primary:Show dotfilet version]
  [option:--verbose]:  [primary:Enable verbose output]
  [option:--dry-run]:  [primary:Preview changes without applying them]

[secondary:EXAMPLES]
  [muted:$] [command:dotfilet init] [muted:# Initialize a new configuration repository]
  [muted:$] [command:dotfilet diff] [muted:# Preview changes before applying]
  [muted:$] [command:dotfilet apply] [argument:programs.vscode] [muted:# Apply only VS Code configuration]
  [muted:$] [command:dotfilet agent start] [muted:# Start bidirectional sync]

[secondary:LEARN MORE]
  Use [command:dotfilet] [argument:<command>] [option:--help] for detailed information about specific commands.
  Read the documentation at [accent:https://github.com/nevir/dotfilet]
  Report issues at [accent:https://github.com/nevir/dotfilet/issues]
```

## Command-Specific Help (`dotfilet apply --help`)

```
[command:dotfilet apply] [muted:- Apply configuration to the current machine]

[secondary:USAGE]
  [command:dotfilet apply] [option:[...resources]] [option:[flags]]

[secondary:DESCRIPTION]
  [primary:Applies the configuration from your repository to the local machine. This will]
  [primary:read your CUE configuration files, resolve any variables, and update system]
  [primary:settings to match your desired state.]

  [primary:You can optionally specify specific resources to limit the scope of changes.]
  [primary:This is useful for testing changes or applying only certain applications.]

[secondary:ARGUMENTS]
  [argument:resources]:  [primary:Specific configuration resources to apply]
                [secondary:Examples:] [placeholder:programs.vscode], [placeholder:macos.dock], [placeholder:programs.git.user.name]

[secondary:OPTIONS]
  [option:--plan] [placeholder:<file>]:     [primary:Apply changes from a previously generated plan file]
  [option:--dry-run]:         [primary:Preview changes without applying them]
  [option:--force]:           [primary:Apply changes even if there are warnings]
  [option:--parallel]:        [primary:Enable parallel plugin execution] [muted:(default: true)]
  [option:--help]:            [primary:Show this help message]

[secondary:EXAMPLES]
  [muted:$] [command:dotfilet apply]
    [secondary:Apply all configuration to the current machine]

  [muted:$] [command:dotfilet apply] [argument:programs.vscode] [argument:programs.git]
    [secondary:Apply only VS Code and Git configuration]

  [muted:$] [command:dotfilet apply] [option:--dry-run]
    [secondary:Preview all changes without applying them]

  [muted:$] [command:dotfilet diff] [option:--plan] [placeholder:changes.plan] [muted:&& dotfilet apply --plan] [placeholder:changes.plan]
    [secondary:Generate a plan, review it, then apply the exact same changes]

[secondary:NOTES]
  [primary:• Changes are applied idempotently - running apply multiple times is safe]
  [primary:• Plugin failures will halt the operation to prevent inconsistent state]
  [primary:• The sync agent will be started automatically if not already running]

[secondary:SEE ALSO]
  [command:dotfilet diff]:   [secondary:Preview changes before applying]
  [command:dotfilet agent]:  [secondary:Manage the bidirectional sync agent]
```

## Subcommand Help (`dotfilet agent --help`)

```
[command:dotfilet agent] [muted:- Manage the bidirectional sync agent]

[secondary:USAGE]
  [command:dotfilet agent] [argument:<subcommand>] [option:[flags]]

[secondary:DESCRIPTION]
  [primary:The sync agent enables bidirectional synchronization between your configuration]
  [primary:files and system state. It monitors for changes made outside of dotfilet and]
  [primary:can update your CUE files to prevent configuration drift.]

[secondary:SUBCOMMANDS]
  [command:associate]:  [primary:Associate agent with current configuration directory]
  [command:start]:      [primary:Start the sync agent (installs if needed)]
  [command:stop]:       [primary:Stop the running sync agent]
  [command:status]:     [primary:Show agent status and configuration]
  [command:install]:    [primary:Install agent as a system service]
  [command:uninstall]:  [primary:Remove agent system service]
  [command:logs]:       [primary:Show agent log output]

[secondary:GLOBAL OPTIONS]
  [option:--help]:      [primary:Show help for command]
  [option:--verbose]:   [primary:Enable verbose output]

[secondary:EXAMPLES]
  [muted:$] [command:dotfilet agent status]
    [secondary:Check if the agent is running and show its configuration]

  [muted:$] [command:dotfilet agent start]
    [secondary:Start the agent for the current configuration directory]

  [muted:$] [command:dotfilet agent stop]
    [secondary:Stop the currently running agent]

[secondary:LEARN MORE]
  Use [command:dotfilet agent] [argument:<subcommand>] [option:--help] for detailed help on specific subcommands.
```

## Error Context Help

```
[error:Error:] [primary:Configuration validation failed]

[primary:The following issues were found in your configuration:]

[primary:In] [accent:programs/vscode.cue][primary::]
  [error:×] [primary:Line 15: Unknown setting] [placeholder:editor.fontLigatures]
    [secondary:VS Code plugin doesn't recognize this setting. Check spelling or update the plugin.]

[primary:In] [accent:macos/dock.cue][primary::]
  [warning:!] [primary:Line 8: Deprecated setting] [placeholder:show-recents]
    [secondary:Use] [option:showRecents] [secondary:instead.]

[secondary:SUGGESTIONS]
  [muted:$] [command:dotfilet diff] [option:--verbose] [muted:# Show detailed validation output]
  [muted:$] [command:dotfilet plugin update] [muted:# Update plugins to latest versions]

[secondary:LEARN MORE]
  Configuration validation: [accent:https://github.com/nevir/dotfilet/docs/validation]
  Plugin troubleshooting: [accent:https://github.com/nevir/dotfilet/docs/plugins]
```

## List Command Output (`dotfilet plugin list`)

```
[secondary:INSTALLED PLUGINS]

[secondary:Core Plugins (bundled)]
  [success:✓] [command:programs/1password]     [muted:v2.1.0] [secondary:1Password configuration]
  [success:✓] [command:programs/git]          [muted:v1.3.0] [secondary:Git configuration and setup]
  [success:✓] [command:programs/vscode]       [muted:v1.8.2] [secondary:Visual Studio Code settings and extensions]
  [success:✓] [command:macos/dock]            [muted:v1.2.0] [secondary:macOS Dock configuration]
  [success:✓] [command:macos/security]        [muted:v1.1.0] [secondary:macOS security and privacy settings]

[secondary:Third-Party Plugins]
  [success:✓] [command:programs/raycast]      [muted:v0.9.1] [secondary:Raycast launcher configuration]
  [warning:!] [command:programs/notion]       [muted:v0.5.0] [secondary:Notion workspace setup] [warning:(update available)]

[secondary:Plugin Status]
  [success:7 active] [muted:•] [warning:1 update available] [muted:•] [error:0 errors]

[secondary:USAGE]
  [command:dotfilet plugin update]:     [secondary:Update all plugins to latest versions]
  [command:dotfilet plugin install]:    [secondary:Install new plugins]
  [command:dotfilet plugin info] [argument:<name>]: [secondary:Show detailed plugin information]
```

## Design Implementation Notes

### Color Fallbacks

When color is not available, the design uses these fallback strategies:

1. **Semantic prefixes**: `[ERROR]`, `[WARNING]`, `[INFO]` for status messages
2. **Visual separators**: Increased use of spacing and line breaks
3. **Typographic emphasis**: ALL CAPS for headers, indentation for hierarchy
4. **Symbolic indicators**: `*`, `-`, `>` for different content types

### Responsive Behavior

- **Narrow terminals** (< 60 chars): Stack command descriptions below command names
- **Medium terminals** (60-100 chars): Standard two-column layout
- **Wide terminals** (> 100 chars): May add third column for additional metadata

### Accessibility Features

- Screen readers can navigate by headers (USAGE, COMMANDS, etc.)
- All color-coded information has textual equivalents
- Consistent indentation creates logical structure
- Commands and options are clearly marked as interactive elements