use crate::macros::dotfilet_command;

dotfilet_command! {
    /// Stop the sync agent
    pub(crate) struct StopCommand;
}

impl StopCommand {
    pub(crate) fn execute(self) {
        println!("dotfilet agent stop: Not yet implemented");
        println!("This command will stop the sync agent daemon");
    }
}
