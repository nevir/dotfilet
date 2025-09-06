# CLI Help Output Design Specification

This document defines the design specification for Dotfilet's CLI help output, emphasizing readability, ergonomics, and accessibility using only standard ANSI colors.

## Design Principles

- **Terminal Compatibility**: Use only standard ANSI colors (16 colors total) for maximum compatibility
- **User Respect**: Honor the user's terminal theme and color preferences
- **Accessibility First**: Ensure help text is readable in all terminal environments
- **Semantic Color Usage**: Colors should convey meaning, not just decoration
- **Graceful Degradation**: Work perfectly in monochrome terminals

## Standard ANSI Color Palette

Our design uses only the 16 standard ANSI colors that are universally supported:

### Basic Colors (0-7)
- **Black** (`\x1b[30m`) - Background elements, separators
- **Red** (`\x1b[31m`) - Errors, warnings, critical information
- **Green** (`\x1b[32m`) - Success states, positive indicators
- **Yellow** (`\x1b[33m`) - Caution, optional parameters, examples
- **Blue** (`\x1b[34m`) - Commands, primary actions
- **Magenta** (`\x1b[35m`) - Subcommands, secondary actions
- **Cyan** (`\x1b[36m`) - Arguments, values, file paths
- **White** (`\x1b[37m`) - Default text, descriptions

### Bright Colors (8-15)
- **Bright Black** (`\x1b[90m`) - Subtle text, dimmed content
- **Bright Red** (`\x1b[91m`) - Error emphasis, critical warnings
- **Bright Green** (`\x1b[92m`) - Success emphasis, confirmations
- **Bright Yellow** (`\x1b[93m`) - Highlighted examples, tips
- **Bright Blue** (`\x1b[94m`) - Command emphasis, brand elements
- **Bright Magenta** (`\x1b[95m`) - Subcommand emphasis
- **Bright Cyan** (`\x1b[96m`) - Argument emphasis, special values
- **Bright White** (`\x1b[97m`) - Headers, important text

## Semantic Design Tokens

### Color Semantic Mapping

```
BRAND:              Bright Blue     - Dotfilet branding, main command
COMMAND:            Blue            - Primary commands (apply, sync, etc.)
SUBCOMMAND:         Magenta         - Subcommands (agent, plugin, etc.)
ARGUMENT:           Cyan            - Required arguments, values
OPTION:             Yellow          - Flags, optional parameters
OPTION_VALUE:       Bright Cyan     - Option values, examples
HEADER:             Bright White    - Section headers (USAGE, COMMANDS, etc.)
DESCRIPTION:        White           - Default description text
SUBTLE:             Bright Black    - Secondary text, hints
SUCCESS:            Green           - Success indicators, confirmations
ERROR:              Red             - Error messages, failures
WARNING:            Yellow          - Warnings, cautions
EMPHASIS:           Bright White    - Important text emphasis
SEPARATOR:          Black           - Dividers, borders (if needed)
```

### Typography Tokens

```
BOLD:               \x1b[1m         - Headers, important elements
DIM:                \x1b[2m         - Secondary text, subtle elements
ITALIC:             \x1b[3m         - Examples, placeholders (limited support)
UNDERLINE:          \x1b[4m         - Links, emphasis (limited support)
RESET:              \x1b[0m         - Reset all formatting
```

## Information Hierarchy

### Level 1: Command Identity
- **Brand Name**: BRAND color, bold
- **Brief Description**: DESCRIPTION color
- **Usage Line**: HEADER + COMMAND + ARGUMENT colors

### Level 2: Section Headers
- **Headers**: HEADER color, bold, uppercase
- **Examples**: "USAGE", "COMMANDS", "OPTIONS", "EXAMPLES"

### Level 3: Command Groups
- **Group Names**: HEADER color, title case
- **Examples**: "Core Commands", "Plugin Commands", "Help Topics"

### Level 4: Individual Commands/Options
- **Command Name**: COMMAND or SUBCOMMAND color, bold
- **Option Name**: OPTION color
- **Description**: DESCRIPTION color, indented

### Level 5: Details and Examples
- **Arguments**: ARGUMENT color
- **Option Values**: OPTION_VALUE color
- **Example Commands**: SUBTLE color with $ prompt
- **File Paths**: ARGUMENT color

## Layout Structure

### Standard Sections (in order)

1. **Command Identity**
   ```
   dotfilet - Declarative configuration management for developer environments
   ```

2. **Usage**
   ```
   USAGE
     dotfilet <command> [options] [arguments]
   ```

3. **Command Groups** (if applicable)
   ```
   CORE COMMANDS
     apply       Apply configuration to system
     sync        Sync manual changes back to config
     
   PLUGIN COMMANDS
     plugin      Manage plugins
     agent       Run plugin agents
   ```

4. **Options** (global flags)
   ```
   OPTIONS
     -h, --help      Show help for command
     -v, --verbose   Enable verbose output
     --version       Show version information
   ```

5. **Examples**
   ```
   EXAMPLES
     $ dotfilet apply
     $ dotfilet sync --dry-run
     $ dotfilet plugin list
   ```

6. **Additional Information**
   ```
   LEARN MORE
     Use 'dotfilet <command> --help' for more information about a command.
     Visit https://github.com/nevir/dotfilet for documentation.
   ```

## Formatting Rules

### Spacing and Alignment

- **Section Spacing**: 2 blank lines between major sections
- **Item Spacing**: 1 blank line between command groups, no spacing within groups
- **Indentation**: 2 spaces for descriptions, 4 spaces for sub-items
- **Alignment**: Command names left-aligned, descriptions aligned to column 16

### Text Formatting

- **Command Names**: Always bold, appropriate semantic color
- **Descriptions**: Regular weight, DESCRIPTION color
- **Arguments**: Angle brackets `<required>`, square brackets `[optional]`
- **Flags**: Always prefixed with `-` or `--`, OPTION color
- **Examples**: Always prefixed with `$ `, SUBTLE color

## Terminal Compatibility

### Color Detection Strategy

1. **Check TERM environment variable**
   - `TERM=dumb` → No colors
   - `TERM=*-mono` → No colors
   - `TERM=*-color` or `TERM=*-256color` → Standard ANSI colors only

2. **Check NO_COLOR environment variable**
   - If `NO_COLOR` is set (any value) → No colors

3. **Check FORCE_COLOR environment variable**
   - If `FORCE_COLOR=1` → Enable standard ANSI colors
   - If `FORCE_COLOR=0` → No colors

4. **TTY Detection**
   - If output is not a TTY → No colors (unless FORCE_COLOR=1)

### Fallback Strategy

When colors are disabled:
- Use **bold** for command names and headers
- Use **indentation** for hierarchy
- Use **spacing** for grouping
- Use **symbols** for emphasis (`*`, `-`, `>`)

### Width Adaptation

- **Narrow terminals** (< 80 cols): Stack descriptions below commands
- **Standard terminals** (80+ cols): Align descriptions in columns
- **Wide terminals** (120+ cols): May use wider column spacing

## Accessibility Considerations

### Screen Readers
- Ensure all formatting enhances rather than replaces semantic meaning
- Use clear, descriptive text for all elements
- Avoid relying solely on color to convey information

### Visual Impairments
- High contrast combinations only
- Respect user's terminal theme colors
- Provide monochrome alternatives
- Use bold/dim for additional visual hierarchy

### Motor Impairments
- Clear, unambiguous command syntax in examples
- Consistent flag naming patterns
- Tab completion support hints

## Implementation Guidelines

### Color Application Functions

```rust
// Recommended function signatures for implementation
fn apply_semantic_color(text: &str, semantic_token: SemanticColor) -> String
fn format_command_help(command: &Command) -> String
fn detect_color_support() -> ColorSupport
fn get_terminal_width() -> Option<usize>
```

### Testing Requirements

- Test in various terminal environments (xterm, tmux, screen, etc.)
- Test with different TERM values
- Test with NO_COLOR and FORCE_COLOR
- Test with narrow terminal widths
- Verify monochrome fallback appearance

### Configuration

Users should be able to override color choices via:
- Environment variables (`DOTFILET_COLOR_SCHEME=mono`)
- Config file settings
- Command line flags (`--no-color`, `--color=always`)