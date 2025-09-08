pub(crate) mod associate;
pub(crate) mod start;
pub(crate) mod stop;

use clap::{Parser, Subcommand};

/// Manage the sync agent
#[derive(Parser)]
pub(crate) struct AgentCommand {
    #[command(subcommand)]
    pub(crate) command: AgentCommands,
}

#[derive(Subcommand)]
pub(crate) enum AgentCommands {
    Associate(associate::AssociateCommand),
    Start(start::StartCommand),
    Stop(stop::StopCommand),
}

impl AgentCommand {
    pub(crate) fn execute(self) {
        match self.command {
            AgentCommands::Associate(cmd) => cmd.execute(),
            AgentCommands::Start(cmd) => cmd.execute(),
            AgentCommands::Stop(cmd) => cmd.execute(),
        }
    }
}
