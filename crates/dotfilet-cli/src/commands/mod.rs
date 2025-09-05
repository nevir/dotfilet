pub mod agent;
pub mod apply;
pub mod diff;
pub mod init;

use clap::Subcommand;

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
