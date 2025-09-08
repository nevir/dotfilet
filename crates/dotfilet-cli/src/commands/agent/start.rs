use clap::Parser;

/// Start the sync agent
#[derive(Parser)]
pub struct StartCommand;

impl StartCommand {
    pub fn execute(self) {
        println!("dotfilet agent start: Not yet implemented");
        println!("This command will start the sync agent daemon");
    }
}
