use crate::command::*;

#[derive(Parser)]
/// Display pending configuration changes
#[command(examples = &[
    "",
    "--plan plan.json",
    "programs.vscode macos.dock",
    "\"macos.*\""
])]
pub(crate) struct DiffCommand {
    /// Specific resources to check for changes
    #[arg()]
    resources: Vec<String>,

    /// Write the diff to a plan file, for use with `apply --plan`
    #[arg(long)]
    plan: Option<String>,
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
