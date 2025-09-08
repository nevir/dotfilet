use clap::Parser;

/// Configure the sync agent to use current configuration
#[derive(Parser)]
pub(crate) struct AssociateCommand;

impl AssociateCommand {
    pub(crate) fn execute(self) {
        println!("dotfilet agent associate: Not yet implemented");
        println!("This command will configure the sync agent for the current directory");
    }
}
