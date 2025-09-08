pub(crate) mod associate;
pub(crate) mod start;
pub(crate) mod stop;

use crate::dotfilet_command;
use clap::Subcommand;

dotfilet_command! {
    /// Manage the sync agent
    pub(crate) struct AgentCommand {
        #[command(subcommand)]
        pub(crate) command: AgentCommands,
    }
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
