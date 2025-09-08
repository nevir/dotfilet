use clap::Command;

use crate::ui::style::{get_help_template, STYLES};

pub(crate) trait DotfiletCommand {
    fn setup_dotfilet_command(self, _noop: &str) -> Self;
}

impl DotfiletCommand for Command {
    /// Configures a [clap](https://docs.rs/clap) Command with Dotfilet's
    /// standard settings.
    ///
    /// Automatically called by the `dotfilet_command!` macro.
    fn setup_dotfilet_command(self, _noop: &str) -> Self {
        self.styles(STYLES).help_template(get_help_template())
    }
}
