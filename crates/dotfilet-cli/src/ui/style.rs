#![allow(dead_code)]
use clap::builder::styling::{AnsiColor, Color, Style, Styles};

const BLACK: Option<Color> = Some(Color::Ansi(AnsiColor::Black));
const BLUE: Option<Color> = Some(Color::Ansi(AnsiColor::Blue));
const CYAN: Option<Color> = Some(Color::Ansi(AnsiColor::Cyan));
const GREEN: Option<Color> = Some(Color::Ansi(AnsiColor::Green));
const MAGENTA: Option<Color> = Some(Color::Ansi(AnsiColor::Magenta));
const RED: Option<Color> = Some(Color::Ansi(AnsiColor::Red));
const YELLOW: Option<Color> = Some(Color::Ansi(AnsiColor::Yellow));
const WHITE: Option<Color> = Some(Color::Ansi(AnsiColor::White));

pub(crate) const TLDR: Style = Style::new().fg_color(GREEN).dimmed();
pub(crate) const HEADER: Style = Style::new().bold().dimmed();
pub(crate) const LITERAL: Style = Style::new().fg_color(YELLOW);
pub(crate) const EXAMPLE_COMMAND: Style = Style::new().fg_color(YELLOW).dimmed();
pub(crate) const EXAMPLE_ARGUMENTS: Style = Style::new().fg_color(CYAN).dimmed();
pub(crate) const ARGUMENT: Style = Style::new().fg_color(CYAN);
pub(crate) const ERROR: Style = Style::new().fg_color(RED);
pub(crate) const CONTEXT: Style = Style::new().dimmed();
pub(crate) const VALID: Style = Style::new().fg_color(GREEN);
pub(crate) const INVALID: Style = Style::new().fg_color(YELLOW);
pub(crate) const PROMPT: Style = Style::new().fg_color(MAGENTA).dimmed();

pub(crate) const STYLES: Styles = Styles::styled()
    .header(HEADER)
    .usage(HEADER)
    .literal(LITERAL)
    .placeholder(ARGUMENT)
    .error(ERROR)
    .valid(VALID)
    .invalid(INVALID)
    .context(CONTEXT)
    .context_value(ARGUMENT);

pub(crate) fn get_help_template() -> String {
    format!(
        "
\u{200c}
{{before-help}}{TLDR}{{about}}{TLDR:#}

{{usage-heading}}
{{tab}}{{usage}}

{{all-args}}{{after-help}}
\u{200c}
"
    )
}

pub(crate) fn format_examples(command: &str, examples: &[&str]) -> String {
    let formatted_examples = examples
        .iter()
        .map(|example| {
            format!("  {PROMPT}$ {PROMPT:#}{EXAMPLE_COMMAND}{command}{EXAMPLE_COMMAND:#} {EXAMPLE_ARGUMENTS}{example}{EXAMPLE_ARGUMENTS:#}")
        })
        .collect::<Vec<_>>()
        .join("\n");

    format!("{HEADER}Examples:{HEADER:#}\n{formatted_examples}")
}
