mod commands;
mod macros;
mod ui;

use clap::Parser;
use commands::RootCommand;

fn main() {
    let cli = RootCommand::parse();

    if cli.global.verbose {
        eprintln!("Verbose mode enabled");
    }

    cli.command.execute();
}
