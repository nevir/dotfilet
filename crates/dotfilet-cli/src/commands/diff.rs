use clap::Parser;

#[derive(Parser)]
#[command(about = "Display pending configuration changes")]
pub struct DiffCommand {
    #[arg(help = "Specific resources to check for changes")]
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
