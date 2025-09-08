use crate::macros::dotfilet_command;

dotfilet_command! {
    /// Scaffold a new Dotfilet repository
    pub(crate) struct InitCommand;
}

impl InitCommand {
    pub(crate) fn execute(self) {
        println!("dotfilet init: Not yet implemented");
        println!("This command will scaffold a new Dotfilet repository with sample configuration");
    }
}
