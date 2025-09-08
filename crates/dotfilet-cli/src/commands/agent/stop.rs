use clap::Parser;

/// Stop the sync agent
#[derive(Parser)]
pub(crate) struct StopCommand;

impl StopCommand {
    pub(crate) fn execute(self) {
        println!("dotfilet agent stop: Not yet implemented");
        println!("This command will stop the sync agent daemon");
    }
}
