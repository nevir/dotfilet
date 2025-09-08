use clap::Command;

use crate::{
    command::examples::DotfiletCommandExamples,
    ui::style::{get_help_template, STYLES},
};

pub(crate) trait DotfiletCommandLifecycle {
    fn postprocess_dotfilet_command(self) -> Self;
    fn _postprocess_inner(self) -> Self;
}

impl DotfiletCommandLifecycle for Command {
    /// Called after the command hierarchy has been built (in main.rs).
    ///
    /// We apply Dotfilet-specific configuration, and perform post-processing
    /// for Dotfilet-custom attributes.
    ///
    /// This should be called once on the root command only.
    fn postprocess_dotfilet_command(mut self) -> Self {
        self = self.styles(STYLES);
        self.build();

        self._postprocess_inner()
    }

    fn _postprocess_inner(mut self) -> Self {
        self = self.help_template(get_help_template());
        self = self._postprocess_examples();

        self.mut_subcommands(|sub| sub._postprocess_inner())
    }
}
