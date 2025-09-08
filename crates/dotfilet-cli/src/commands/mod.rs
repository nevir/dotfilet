pub(crate) mod agent;
pub(crate) mod apply;
pub(crate) mod diff;
pub(crate) mod init;

use crate::dotfilet_command;
use clap::{Parser, Subcommand};

dotfilet_command! {
    #[command(version, about)]
    #[command(name = "dotfilet")] // not the inferred "dotfilet-cli"
    #[command(arg_required_else_help = true)]
    pub(crate) struct RootCommand {
        #[command(flatten)]
        pub(crate) global: GlobalArgs,

        #[command(subcommand)]
        pub(crate) command: Commands,
    }
}

#[derive(Parser)]
pub(crate) struct GlobalArgs {
    /// Path to the Dotfilet repository to use.
    #[arg(short, long, global = true, default_value = ".")]
    pub(crate) project: String,

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
