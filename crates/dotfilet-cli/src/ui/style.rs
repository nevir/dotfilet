use clap::builder::styling::{AnsiColor, Effects, Style, Styles};

//pub(crate) const NOP: Style = Style::new();
pub(crate) const HEADER: Style = AnsiColor::Green.on_default().effects(Effects::BOLD);
pub(crate) const USAGE: Style = AnsiColor::Green.on_default().effects(Effects::BOLD);
pub(crate) const LITERAL: Style = AnsiColor::Cyan.on_default().effects(Effects::BOLD);
pub(crate) const PLACEHOLDER: Style = AnsiColor::Cyan.on_default();
pub(crate) const ERROR: Style = AnsiColor::Red.on_default().effects(Effects::BOLD);
//pub(crate) const WARN: Style = AnsiColor::Yellow.on_default().effects(Effects::BOLD);
//pub(crate) const NOTE: Style = AnsiColor::Cyan.on_default().effects(Effects::BOLD);
//pub(crate) const GOOD: Style = AnsiColor::Green.on_default().effects(Effects::BOLD);
pub(crate) const VALID: Style = AnsiColor::Cyan.on_default().effects(Effects::BOLD);
pub(crate) const INVALID: Style = AnsiColor::Yellow.on_default().effects(Effects::BOLD);

/// Cargo's color style
/// [source](https://github.com/crate-ci/clap-cargo/blob/master/src/style.rs)
pub(crate) const STYLES: Styles = Styles::styled()
    .header(HEADER)
    .usage(USAGE)
    .literal(LITERAL)
    .placeholder(PLACEHOLDER)
    .error(ERROR)
    .valid(VALID)
    .invalid(INVALID);

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
