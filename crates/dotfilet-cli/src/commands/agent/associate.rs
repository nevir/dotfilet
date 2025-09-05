use clap::Parser;

#[derive(Parser)]
#[command(about = "Configure the sync agent to use current configuration")]
pub struct AssociateCommand;

impl AssociateCommand {
    pub fn execute(self, _verbose: bool, _dry_run: bool) {
        println!("dotfilet agent associate: Not yet implemented");
        println!("This command will configure the sync agent for the current directory");
    }
}
