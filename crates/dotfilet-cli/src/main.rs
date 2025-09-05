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
}

fn main() {
    let cli = Cli::parse();

    if cli.global.verbose {
        eprintln!("Verbose mode enabled");
    }

    cli.command.execute();
}
