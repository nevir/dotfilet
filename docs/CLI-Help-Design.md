# CLI Help Output Design

This document defines the visual design and formatting standards for Dotfilet's CLI help output, emphasizing readability, ergonomics, and effective use of color.

## Design Principles

1. **Clarity Over Complexity**: Information should be immediately scannable with clear visual hierarchy
2. **Progressive Disclosure**: Most important information first, with details available on demand
3. **Contextual Color**: Use color purposefully to convey meaning and improve readability
4. **Responsive Layout**: Adapt gracefully to different terminal widths
5. **Accessibility**: Ensure readability across different terminal themes and capabilities

## Semantic Design Tokens

### Color Palette

Our color system uses semantic naming to ensure consistent meaning across all help output:

#### Primary Colors
- `brand`: `#2563eb` (blue-600) - For the Dotfilet brand/name
- `accent`: `#0ea5e9` (sky-500) - For highlights and emphasis

#### Status Colors
- `success`: `#059669` (emerald-600) - For positive states
- `warning`: `#d97706` (amber-600) - For cautions and warnings
- `error`: `#dc2626` (red-600) - For errors and critical information

#### Content Colors
- `primary`: `#374151` (gray-700) - Primary text content
- `secondary`: `#6b7280` (gray-500) - Secondary/supporting text
- `muted`: `#9ca3af` (gray-400) - Least important text
- `inverse`: `#f9fafb` (gray-50) - Text on dark backgrounds

#### Syntax Colors (for code examples)
- `command`: `#059669` (emerald-600) - Command names
- `argument`: `#2563eb` (blue-600) - Required arguments
- `option`: `#7c3aed` (violet-600) - Optional parameters/flags
- `placeholder`: `#d97706` (amber-600) - User-replaceable content
- `comment`: `#6b7280` (gray-500) - Explanatory comments

### Typography Scale

#### Weights
- `normal`: Regular text weight
- `medium`: For emphasis and section headers
- `bold`: For strong emphasis and primary commands

#### Size Hierarchy (using spacing for visual weight)
- `h1`: Main command title (bold, with decorative elements)
- `h2`: Section headers (medium weight, uppercase)
- `h3`: Subsection headers (medium weight)
- `body`: Standard text
- `small`: Supporting information and metadata

### Spacing System

- `xs`: 1 space - Minimal separation
- `sm`: 2 spaces - Standard indentation
- `md`: 4 spaces - Section indentation
- `lg`: 8 spaces - Major section separation
- `xl`: 2 lines - Between major sections

### Layout Tokens

- `margin-left`: 2 spaces - Standard left margin for content
- `indent-command`: 4 spaces - Indentation for command listings
- `indent-detail`: 6 spaces - Indentation for command descriptions
- `column-gap`: 2 spaces - Minimum gap between columns
- `max-width`: 100 characters - Maximum line length before wrapping

## Information Hierarchy

### Level 1: Command Identity
- Command name and version
- Brief one-line description
- Visual prominence through brand coloring

### Level 2: Usage Pattern
- Basic syntax template
- Clear indication of required vs optional elements
- Immediate actionability

### Level 3: Command Groups
- Logical grouping of related commands
- Clear category headers
- Consistent organization (core → specialized → meta)

### Level 4: Individual Commands
- Command name with consistent styling
- Concise description aligned for scannability
- Optional usage hints for complex commands

### Level 5: Detailed Information
- Flags and options
- Examples
- Additional help references
- Error context (when applicable)

## Layout Structure

### Standard Help Layout

```
[BRAND NAME] [VERSION]
[Brief description]

USAGE
  [usage pattern with syntax highlighting]

[COMMAND GROUPS]
  [group 1 header]
    [command]: [description]
    [command]: [description]
  
  [group 2 header]
    [command]: [description]

[FLAGS]
  [flag]: [description]

[EXAMPLES]
  $ [command example]
  $ [command example]

[LEARN MORE]
  [additional resources]
```

### Command-Specific Help Layout

```
[COMMAND NAME] - [brief description]

USAGE
  [specific usage patterns]

DESCRIPTION
  [detailed explanation if needed]

[ARGUMENTS/OPTIONS sections as needed]

EXAMPLES
  [relevant examples]
```

## Formatting Rules

### Headers
- Section headers: `UPPERCASE`, `medium` weight, `secondary` color
- Followed by single line break
- No trailing punctuation

### Command Names
- Use `command` color in listings
- `medium` weight for emphasis
- Consistent left alignment

### Descriptions
- Use `primary` color for main descriptions
- Use `secondary` color for supplementary information
- Align descriptions for scannability (typically column-aligned)

### Usage Patterns
- Commands: `command` color
- Required arguments: `<argument>` in `argument` color
- Optional arguments: `[argument]` in `option` color
- Placeholders: `<PLACEHOLDER>` in `placeholder` color

### Examples
- Prefix with `$` in `muted` color
- Commands in `command` color
- Arguments in appropriate semantic colors
- Include realistic, useful examples

### Flags and Options
- Flag names: `option` color
- Descriptions: `primary` color
- Default values: `muted` color in parentheses

## Accessibility Considerations

### Terminal Compatibility
- Graceful fallback for terminals without color support
- No essential information conveyed through color alone
- Support for high contrast mode

### Responsive Design
- Adapt column widths based on terminal width
- Wrap long lines intelligently
- Maintain readability at narrow widths (minimum 60 characters)

### Screen Reader Support
- Logical heading structure
- Descriptive text for visual elements
- Clear indication of required vs optional elements

## Implementation Notes

### Color Detection
- Detect terminal color capability
- Provide monochrome fallback
- Support both light and dark terminal themes

### Dynamic Layout
- Calculate optimal column widths
- Adjust spacing based on content length
- Handle terminal width changes gracefully

### Internationalization Ready
- Design accommodates variable text lengths
- No hard-coded spacing assumptions
- UTF-8 character support