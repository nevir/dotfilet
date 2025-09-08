mod command;
mod commands;
mod ui;

use crate::command::*;

use clap::{CommandFactory, FromArgMatches};
use commands::RootCommand;

fn main() {
    let mut cmd = RootCommand::command_for_update();
    cmd = cmd.postprocess_dotfilet_command();

    let mut matches = cmd.get_matches();
    let result = RootCommand::from_arg_matches_mut(&mut matches).map_err(|err| {
        let mut err_cmd = RootCommand::command();
        err.format(&mut err_cmd)
    });
    let cli = match result {
        Ok(cli) => cli,
        Err(err) => {
            err.exit();
        }
    };

    if cli.global.verbose {
        eprintln!("Verbose mode enabled");
    }

    cli.command.execute();
}
