mod commands;
mod styles;

use crate::styles::CUSTOM_STYLES;

// use crate::styles::HEADER;
// use crate::styles::LITERAL;
use clap::Parser;
use commands::Commands;

#[derive(Parser)]
#[command(name = "dotfilet")]
#[command(about = "A declarative configuration management tool for developer environments")]
#[command(
    long_about = styles::long_about("Dotfilet treats your machine's configuration as code, enabling versioning,
sharing, and reliable reproduction across multiple machines. It provides bi-directional
sync between your system state and configuration files, preventing configuration drift.")
)]
#[command(version)]
#[command(after_help = styles::examples(&[
    ("dotfilet init", "Initialize a new Dotfilet repository"),
    ("dotfilet diff", "Show all pending configuration changes"),
    ("dotfilet diff programs.vscode", "Show changes for VS Code only"),
    ("dotfilet apply", "Apply all configuration changes"),
    ("dotfilet apply --plan changes.json", "Apply changes from a specific plan file"),
    ("dotfilet agent start", "Start the background sync agent"),
]))]
// #[command(after_help = format!("{HEADER}Examples{HEADER:#}:
//     {LITERAL}dotfilet init{LITERAL:#}                      Initialize a new Dotfilet repository
//     {LITERAL}dotfilet diff{LITERAL:#}                      Show all pending configuration changes
//     {LITERAL}dotfilet diff programs.vscode{LITERAL:#}      Show changes for VS Code only
//     {LITERAL}dotfilet apply{LITERAL:#}                     Apply all configuration changes
//     {LITERAL}dotfilet apply --plan changes.json{LITERAL:#} Apply changes from a specific plan file
//     {LITERAL}dotfilet agent start{LITERAL:#}               Start the background sync agent

// For more information about a specific command, run: dotfilet <COMMAND> --help"))]
#[command(styles = CUSTOM_STYLES)]
struct Cli {
    #[command(flatten)]
    global: GlobalArgs,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Parser)]
struct GlobalArgs {
    #[arg(long, global = true, help = "Enable verbose output")]
    verbose: bool,
}

fn main() {
    let cli = Cli::parse();

    if cli.global.verbose {
        eprintln!("Verbose mode enabled");
    }

    cli.command.execute();
}
