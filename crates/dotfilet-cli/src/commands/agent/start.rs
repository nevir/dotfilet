use clap::Parser;

#[derive(Parser)]
#[command(about = "Start the sync agent")]
pub struct StartCommand;

impl StartCommand {
    pub fn execute(self, _verbose: bool, _dry_run: bool) {
        println!("dotfilet agent start: Not yet implemented");
        println!("This command will start the sync agent daemon");
    }
}
