pub(crate) mod agent;
pub(crate) mod apply;
pub(crate) mod diff;
pub(crate) mod init;

use clap::{Parser, Subcommand};

use crate::ui::style::{get_help_template, STYLES};

#[derive(Parser)]
#[command(version, about)]
#[command(name = "dotfilet")] // not the inferred "dotfilet-cli"
#[command(styles = STYLES)]
#[command(help_template = get_help_template())]
pub(crate) struct RootCommand {
    #[command(flatten)]
    pub(crate) global: GlobalArgs,

    #[command(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Parser)]
pub(crate) struct GlobalArgs {
    /// Enable verbose output
    #[arg(long, global = true)]
    pub(crate) verbose: bool,
}

#[derive(Subcommand)]
pub(crate) enum Commands {
    Init(init::InitCommand),
    Diff(diff::DiffCommand),
    Apply(apply::ApplyCommand),
    Agent(agent::AgentCommand),
}

impl Commands {
    pub(crate) fn execute(self) {
        match self {
            Commands::Init(cmd) => cmd.execute(),
            Commands::Diff(cmd) => cmd.execute(),
            Commands::Apply(cmd) => cmd.execute(),
            Commands::Agent(cmd) => cmd.execute(),
        }
    }
}
