use clap::Command;

use crate::ui::style::format_examples;

pub(crate) trait DotfiletCommandExamples {
    fn examples(self, examples: &[&str]) -> Self;
    fn _postprocess_examples(self) -> Self;
}

impl DotfiletCommandExamples for Command {
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

    fn _postprocess_examples(mut self) -> Self {
        if let Some(text) = self.get_after_help() {
            let text = text
                .ansi()
                .to_string()
                .replace("{command}", self.get_bin_name().unwrap());
            self = self.after_help(text);
        }

        self
    }
}
