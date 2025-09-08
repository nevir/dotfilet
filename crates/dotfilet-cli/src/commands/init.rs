use clap::Parser;

/// Scaffold a new Dotfilet repository
#[derive(Parser)]
pub(crate) struct InitCommand;

impl InitCommand {
    pub(crate) fn execute(self) {
        println!("dotfilet init: Not yet implemented");
        println!("This command will scaffold a new Dotfilet repository with sample configuration");
    }
}
