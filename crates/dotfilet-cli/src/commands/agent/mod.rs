pub mod associate;
pub mod start;
pub mod stop;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(about = "Manage the sync agent")]
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
    pub fn execute(self, verbose: bool, dry_run: bool) {
        self.command.execute(verbose, dry_run);
    }
}

impl AgentCommands {
    pub fn execute(self, verbose: bool, dry_run: bool) {
        match self {
            AgentCommands::Associate(cmd) => cmd.execute(verbose, dry_run),
            AgentCommands::Start(cmd) => cmd.execute(verbose, dry_run),
            AgentCommands::Stop(cmd) => cmd.execute(verbose, dry_run),
        }
    }
}
