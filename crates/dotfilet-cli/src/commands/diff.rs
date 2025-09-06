use crate::styles;
use crate::styles::CUSTOM_STYLES;
use crate::styles::{HEADER, LITERAL};
use clap::Parser;

#[derive(Parser)]
#[command(about = styles::about("Display pending configuration changes"))]
#[command(
    long_about = styles::long_about("Compare your current system state against the desired configuration
and display what changes would be applied. This is effectively a 'dry-run' mode
that shows you exactly what Dotfilet would do without making any changes.")
)]
#[command(after_help = format!( "{HEADER}Examples{HEADER:#}
    {LITERAL}dotfilet diff{LITERAL:#}                     Show all pending changes
    {LITERAL}dotfilet diff programs.vscode{LITERAL:#}     Show only VS Code changes
    {LITERAL}dotfilet diff macos.dock{LITERAL:#}          Show only Dock configuration changes
    {LITERAL}dotfilet diff programs.vscode programs.chrome{LITERAL:#}   Show changes for multiple programs

{HEADER}Resource Examples{HEADER:#}
    {LITERAL}programs.vscode{LITERAL:#}                   Specific application
    {LITERAL}programs.vscode.settings{LITERAL:#}          Application subsection
    {LITERAL}macos.dock{LITERAL:#}                        System component
    {LITERAL}macos.dock.autohide{LITERAL:#}               Specific setting"))]
#[command(styles = CUSTOM_STYLES)]
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
