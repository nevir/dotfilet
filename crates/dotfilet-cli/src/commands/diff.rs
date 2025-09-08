use crate::macros::dotfilet_command;

dotfilet_command! {
    /// Display pending configuration changes
    pub(crate) struct DiffCommand {
        /// Specific resources to check for changes
        #[arg()]
        resources: Vec<String>,
    }
}

impl DiffCommand {
    pub(crate) fn execute(self) {
        if self.resources.is_empty() {
            println!("dotfilet diff: Not yet implemented");
            println!("This command will show all pending configuration changes");
        } else {
            println!("dotfilet diff: Not yet implemented");
            println!(
                "This command will show pending changes for: {}",
                self.resources.join(", ")
            );
        }
    }
}
