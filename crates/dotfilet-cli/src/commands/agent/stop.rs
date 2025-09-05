use clap::Parser;

#[derive(Parser)]
#[command(about = "Stop the sync agent")]
pub struct StopCommand;

impl StopCommand {
    pub fn execute(self) {
        println!("dotfilet agent stop: Not yet implemented");
        println!("This command will stop the sync agent daemon");
    }
}
