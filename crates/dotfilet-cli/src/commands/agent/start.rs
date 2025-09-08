use clap::Parser;

/// Start the sync agent
#[derive(Parser)]
pub(crate) struct StartCommand;

impl StartCommand {
    pub(crate) fn execute(self) {
        println!("dotfilet agent start: Not yet implemented");
        println!("This command will start the sync agent daemon");
    }
}
