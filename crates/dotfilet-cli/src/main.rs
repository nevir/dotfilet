mod commands;

use clap::Parser;
use commands::Commands;

#[derive(Parser)]
#[command(name = "dotfilet")]
#[command(about = "A declarative configuration management tool for developer environments")]
#[command(version)]
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

    #[arg(long, global = true, help = "Perform a dry run without making changes")]
    dry_run: bool,
}

fn main() {
    let cli = Cli::parse();

    if cli.global.verbose {
        eprintln!("Verbose mode enabled");
    }

    if cli.global.dry_run {
        eprintln!("Dry run mode enabled - no changes will be made");
    }

    cli.command.execute(cli.global.verbose, cli.global.dry_run);
}
