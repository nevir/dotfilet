mod commands;

use clap::Parser;
use commands::Commands;

#[derive(Parser)]
#[command(name = "dotfilet")]
#[command(about = "A declarative configuration management tool for developer environments")]
#[command(
    long_about = "Dotfilet treats your machine's configuration as code, enabling versioning,
sharing, and reliable reproduction across multiple machines. It provides bi-directional
sync between your system state and configuration files, preventing configuration drift."
)]
#[command(version)]
#[command(after_help = "EXAMPLES:
    dotfilet init                     Initialize a new Dotfilet repository
    dotfilet diff                     Show all pending configuration changes
    dotfilet diff programs.vscode     Show changes for VS Code only
    dotfilet apply                    Apply all configuration changes
    dotfilet apply --plan changes.json Apply changes from a specific plan file
    dotfilet agent start              Start the background sync agent

For more information about a specific command, run: dotfilet <COMMAND> --help")]
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
