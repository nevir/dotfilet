use clap::Parser;

#[derive(Parser)]
#[command(about = "Stop the sync agent")]
pub struct StopCommand;

impl StopCommand {
    pub fn execute(self, _verbose: bool, _dry_run: bool) {
        println!("dotfilet agent stop: Not yet implemented");
        println!("This command will stop the sync agent daemon");
    }
}
