pub mod associate;
pub mod start;
pub mod stop;

use clap::{builder::StyledStr, Parser, Subcommand};

#[derive(Parser)]
#[command(about = "Manage the sync agent")]
#[command(override_help = "blahblah")]
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
    // You cannot directly override get_override_help from clap's internals,
    // as it's not a trait method and is not designed for user override.
    // Instead, you should use the #[command(override_help = "...")] attribute
    // or the .override_help("...") builder method on your command struct.

    // Example using the attribute (already in your code):
    // #[command(override_help = "your custom help text")]

    // If you need dynamic help text, you must construct your clap::Command manually
    // and call .override_help(your_dynamic_text) before parsing.

    // There is no supported way to override get_override_help via an inherent method.

    pub fn execute(self) {
        match self.command {
            AgentCommands::Associate(cmd) => cmd.execute(),
            AgentCommands::Start(cmd) => cmd.execute(),
            AgentCommands::Stop(cmd) => cmd.execute(),
        }
    }
}
