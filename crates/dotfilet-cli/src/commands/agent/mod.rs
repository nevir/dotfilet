pub mod associate;
pub mod start;
pub mod stop;

use clap::{Parser, Subcommand};

/// Manage the sync agent
#[derive(Parser)]
pub struct AgentCommand {
    #[command(subcommand)]
    pub command: AgentCommands,
}

#[derive(Subcommand)]
pub enum AgentCommands {
    Associate(associate::AssociateCommand),
    Start(start::StartCommand),
    Stop(stop::StopCommand),
}

impl AgentCommand {
    pub fn execute(self) {
        match self.command {
            AgentCommands::Associate(cmd) => cmd.execute(),
            AgentCommands::Start(cmd) => cmd.execute(),
            AgentCommands::Stop(cmd) => cmd.execute(),
        }
    }
}
