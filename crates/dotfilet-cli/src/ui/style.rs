use clap::builder::styling::{Style, Styles};
// use clap::builder::styling::{AnsiColor, Color, Style, Styles};

// const BLUE: Option<Color> = Some(Color::Ansi(AnsiColor::Blue));
// const GREEN: Option<Color> = Some(Color::Ansi(AnsiColor::Green));
// const YELLOW: Option<Color> = Some(Color::Ansi(AnsiColor::Yellow));
// const RED: Option<Color> = Some(Color::Ansi(AnsiColor::Red));
// const CYAN: Option<Color> = Some(Color::Ansi(AnsiColor::Cyan));
// const MAGENTA: Option<Color> = Some(Color::Ansi(AnsiColor::Magenta));

pub(crate) const HEADER: Style = Style::new().bold();
pub(crate) const USAGE: Style = Style::new();
pub(crate) const LITERAL: Style = Style::new();
pub(crate) const PLACEHOLDER: Style = Style::new();
pub(crate) const VALID: Style = Style::new();
pub(crate) const INVALID: Style = Style::new();
pub(crate) const ERROR: Style = Style::new();
pub(crate) const CONTEXT: Style = Style::new().dimmed();
pub(crate) const CONTEXT_VALUE: Style = Style::new();
// //pub(crate) const WARN: Style = Style::new().fg_color(YELLOW).bold();
// //pub(crate) const NOTE: Style = Style::new().fg_color(CYAN).bold();
// //pub(crate) const GOOD: Style = Style::new().fg_color(GREEN).bold();

pub(crate) const STYLES: Styles = Styles::styled()
    .header(HEADER)
    .usage(USAGE)
    .literal(LITERAL)
    .placeholder(PLACEHOLDER)
    .error(ERROR)
    .valid(VALID)
    .invalid(INVALID)
    .context(CONTEXT)
    .context_value(CONTEXT_VALUE);

pub(crate) fn get_help_template() -> String {
    format!(
        "\
{HEADER}TESTING:{HEADER:#}
name: {{name}}
bin: {{bin}}
version: {{version}}
author: {{author}}
author-with-newline: {{author-with-newline}}
author-section: {{author-section}}
about: {{about}}
about-with-newline: {{about-with-newline}}
about-section: {{about-section}}
usage-heading: {{usage-heading}}
usage: {{usage}}
all-args: {{all-args}}
options: {{options}}
positionals: {{positionals}}
subcommands: {{subcommands}}
tab: {{tab}}
after-help: {{after-help}}
before-help: {{before-help}}"
    )
}
