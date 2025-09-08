use clap::Parser;

/// Display pending configuration changes
#[derive(Parser)]
pub struct DiffCommand {
    /// Specific resources to check for changes
    #[arg()]
    pub resources: Vec<String>,
}

impl DiffCommand {
    pub fn execute(self) {
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
