use clap::builder::{styling::AnsiColor, styling::Style, Styles};

pub fn get_custom_styles() -> Styles {
    let blue = Some(AnsiColor::Blue.into());
    let green = Some(AnsiColor::Green.into());
    let magenta = Some(AnsiColor::Magenta.into());
    let yellow = Some(AnsiColor::Yellow.into());

    Styles::default()
        .header(Style::new().fg_color(magenta).bold())
        .usage(Style::new().fg_color(magenta).bold())
        .literal(Style::new().fg_color(blue).bold())
        .invalid(Style::new().fg_color(yellow))
        .placeholder(Style::new().fg_color(green))
}
