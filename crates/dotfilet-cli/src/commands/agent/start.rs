use crate::dotfilet_command;

dotfilet_command! {
    /// Start the sync agent
    pub(crate) struct StartCommand;
}

impl StartCommand {
    pub(crate) fn execute(self) {
        println!("dotfilet agent start: Not yet implemented");
        println!("This command will start the sync agent daemon");
    }
}
