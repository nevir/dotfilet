use clap::Command;

use crate::ui::style::{format_examples, get_help_template, STYLES};

pub(crate) trait DotfiletCommandAttributes {
    fn examples(self, examples: &[&str]) -> Self;
}

pub(crate) trait DotfiletCommandLifecycle {
    fn setup_dotfilet_command(self, _noop: &str) -> Self;
    fn postprocess_dotfilet_command(self) -> Self;
}

impl DotfiletCommandAttributes for Command {
    /// Adds example usages to the command's help text.
    fn examples(self, examples: &[&str]) -> Self {
        let current_after_help = self.get_after_help();
        let examples_section = format_examples("{command}", examples);
        let new_after_help = match current_after_help {
            Some(existing) => format!("{existing}\n\n{examples_section}"),
            None => examples_section,
        };

        self.after_help(new_after_help)
    }
}

impl DotfiletCommandLifecycle for Command {
    /// Configures a [clap](https://docs.rs/clap) Command with Dotfilet's
    /// standard settings.
    ///
    /// Automatically called by the `dotfilet_command!` macro.
    fn setup_dotfilet_command(self, _noop: &str) -> Self {
        self.styles(STYLES).help_template(get_help_template())
    }

    fn postprocess_dotfilet_command(mut self) -> Self {
        self.build();

        self.mut_subcommands(|sub| {
            if let Some(text) = sub.get_after_help() {
                let text = text
                    .ansi()
                    .to_string()
                    .replace("{command}", sub.get_bin_name().unwrap());
                sub.after_help(text)
            } else {
                sub
            }
        })
    }
}
