pub mod associate;
pub mod start;
pub mod stop;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(about = "Manage the sync agent")]
#[command(
    long_about = "The sync agent runs in the background to monitor your system for changes
and automatically update your configuration files when you make manual changes via
GUI or other tools. This enables bi-directional synchronization."
)]
#[command(after_help = "EXAMPLES:
    dotfilet agent associate          Configure agent for current directory
    dotfilet agent start              Start the background sync agent
    dotfilet agent stop               Stop the background sync agent
    
TYPICAL WORKFLOW:
    1. dotfilet agent associate       Link agent to your config directory
    2. dotfilet agent start           Begin background monitoring
    3. Make changes via System Preferences, app settings, etc.
    4. Agent automatically detects changes and updates your .cue files")]
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
