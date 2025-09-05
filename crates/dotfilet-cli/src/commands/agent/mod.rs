pub mod associate;
pub mod start;
pub mod stop;

use clap::Subcommand;

#[derive(Subcommand)]
pub enum AgentCommands {
    #[command(about = "Configure the sync agent to use current configuration")]
    Associate,

    #[command(about = "Start the sync agent")]
    Start,

    #[command(about = "Stop the sync agent")]
    Stop,
}

impl AgentCommands {
    pub fn execute(self, verbose: bool, dry_run: bool) {
        match self {
            AgentCommands::Associate => associate::execute(verbose, dry_run),
            AgentCommands::Start => start::execute(verbose, dry_run),
            AgentCommands::Stop => stop::execute(verbose, dry_run),
        }
    }
}
