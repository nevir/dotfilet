use clap::Parser;
use crate::styles::get_custom_styles;

#[derive(Parser)]
#[command(about = "Display pending configuration changes")]
#[command(
    long_about = "Compare your current system state against the desired configuration
and display what changes would be applied. This is effectively a 'dry-run' mode
that shows you exactly what Dotfilet would do without making any changes."
)]
#[command(after_help = "Examples:
    dotfilet diff                     Show all pending changes
    dotfilet diff programs.vscode     Show only VS Code changes
    dotfilet diff macos.dock          Show only Dock configuration changes
    dotfilet diff programs.vscode programs.chrome   Show changes for multiple programs
    
Resource Examples:
    programs.vscode                   Specific application
    programs.vscode.settings          Application subsection
    macos.dock                        System component
    macos.dock.autohide              Specific setting")]
#[command(styles = get_custom_styles())]
pub struct DiffCommand {
    #[arg(help = "Specific resources to check for changes (e.g., programs.vscode, macos.dock)")]
    pub resources: Vec<String>,
}

impl DiffCommand {
    pub fn execute(self) {
        // Basic validation for resource names
        for resource in &self.resources {
            if resource.is_empty() {
                eprintln!("❌ Error: Empty resource name is not allowed.");
                eprintln!();
                eprintln!("💡 Valid resource examples:");
                eprintln!("   • programs.vscode");
                eprintln!("   • macos.dock");
                eprintln!("   • programs.chrome.settings");
                std::process::exit(1);
            }

            // Basic format validation (just checking for reasonable patterns)
            if !resource.contains('.')
                && !resource
                    .chars()
                    .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
            {
                eprintln!(
                    "❌ Warning: Resource '{}' may not follow expected naming conventions.",
                    resource
                );
                eprintln!("   Expected format: category.name or category.name.subcategory");
                eprintln!();
            }
        }

        if self.resources.is_empty() {
            println!("🚧 Command not yet implemented");
            println!();
            println!("When ready, this command will:");
            println!("  • Compare current system state against your configuration files");
            println!("  • Display exactly what changes would be applied");
            println!("  • Show additions, modifications, and removals in a clear format");
            println!("  • Support filtering by specific resources or applications");
            println!();
            println!("This will be your primary tool for previewing changes before applying them.");
        } else {
            println!("🚧 Command not yet implemented");
            println!();
            println!("When ready, this command will show pending changes for:");
            for resource in &self.resources {
                println!("  • {}", resource);
            }
            println!();
            println!(
                "Resource filtering will help you focus on specific parts of your configuration."
            );
        }
    }
}