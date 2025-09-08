pub mod agent;
pub mod apply;
pub mod diff;
pub mod init;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, name = "dotfilet")]
pub struct RootCommand {
    #[command(flatten)]
    pub global: GlobalArgs,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Parser)]
pub struct GlobalArgs {
    /// Enable verbose output
    #[arg(long, global = true)]
    pub verbose: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    Init(init::InitCommand),
    Diff(diff::DiffCommand),
    Apply(apply::ApplyCommand),
    Agent(agent::AgentCommand),
}

impl Commands {
    pub fn execute(self) {
        match self {
            Commands::Init(cmd) => cmd.execute(),
            Commands::Diff(cmd) => cmd.execute(),
            Commands::Apply(cmd) => cmd.execute(),
            Commands::Agent(cmd) => cmd.execute(),
        }
    }
}
