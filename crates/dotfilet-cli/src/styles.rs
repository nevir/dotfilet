use clap::builder::{styling::AnsiColor, styling::Color, styling::Style, Styles};

const BLUE: Option<Color> = Some(Color::Ansi(AnsiColor::Blue));
const CYAN: Option<Color> = Some(Color::Ansi(AnsiColor::Cyan));
// const GREEN: Option<Color> = Some(Color::Ansi(AnsiColor::Green));
const MAGENTA: Option<Color> = Some(Color::Ansi(AnsiColor::Magenta));
const YELLOW: Option<Color> = Some(Color::Ansi(AnsiColor::Yellow));

pub const LONG_ABOUT: Style = Style::new().fg_color(CYAN);
pub const HEADER: Style = Style::new().fg_color(YELLOW).bold();
pub const LITERAL: Style = Style::new().fg_color(MAGENTA).bold();
pub const PLACEHOLDER: Style = Style::new().fg_color(BLUE);
pub const INVALID: Style = Style::new().fg_color(YELLOW);

pub const CUSTOM_STYLES: Styles = Styles::plain()
    .header(HEADER)
    .invalid(INVALID)
    .literal(LITERAL)
    .placeholder(PLACEHOLDER)
    .usage(HEADER);

pub fn about(text: &str) -> &str {
    return text;
}

pub fn long_about(text: &str) -> String {
    format!("{LONG_ABOUT}{text}{LONG_ABOUT:#}")
}

pub fn examples(examples: &[(&str, &str)]) -> String {
    let max_command_length = examples.iter().map(|(cmd, _)| cmd.len()).max().unwrap_or(0);

    let mut result = String::new();
    result.push_str(&format!("{HEADER}Examples{HEADER:#}:\n"));
    for (cmd, desc) in examples {
        let padding = " ".repeat(max_command_length - cmd.len());
        result.push_str(&format!("    {LITERAL}{cmd}{LITERAL:#}{padding}  {desc}\n"));
    }

    result
}
