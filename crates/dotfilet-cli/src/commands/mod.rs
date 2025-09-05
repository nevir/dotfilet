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
    pub fn execute(self, verbose: bool, dry_run: bool) {
        match self {
            Commands::Init(cmd) => cmd.execute(verbose, dry_run),
            Commands::Diff(cmd) => cmd.execute(verbose, dry_run),
            Commands::Apply(cmd) => cmd.execute(verbose, dry_run),
            Commands::Agent(cmd) => cmd.execute(verbose, dry_run),
        }
    }
}
