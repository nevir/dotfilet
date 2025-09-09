use std::env;
use std::io;

use clap::{CommandFactory};
use clap_complete::{generate, Shell};

use crate::command::*;

#[derive(Parser)]
/// Generate shell completion scripts
pub(crate) struct CompletionCommand {
    /// Shell to generate completion for
    #[arg(value_enum)]
    pub(crate) shell: Option<SupportedShell>,
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub(crate) enum SupportedShell {
    Bash,
    Zsh,
    Fish,
}

impl From<SupportedShell> for Shell {
    fn from(shell: SupportedShell) -> Self {
        match shell {
            SupportedShell::Bash => Shell::Bash,
            SupportedShell::Zsh => Shell::Zsh,
            SupportedShell::Fish => Shell::Fish,
        }
    }
}

impl CompletionCommand {
    pub(crate) fn execute(self) {
        let shell = match self.shell {
            Some(shell) => shell,
            None => match detect_shell() {
                Ok(shell) => shell,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            },
        };

        let mut app = crate::commands::RootCommand::command();
        let shell: Shell = shell.into();
        
        generate(shell, &mut app, "dotfilet", &mut io::stdout());
    }
}

fn detect_shell() -> Result<SupportedShell, String> {
    let shell_var = env::var("SHELL").map_err(|_| {
        "Unable to detect shell: $SHELL environment variable is not set. Please specify a shell explicitly."
    })?;

    let shell_name = shell_var
        .split('/')
        .last()
        .ok_or("Invalid $SHELL environment variable")?;

    match shell_name {
        "bash" => Ok(SupportedShell::Bash),
        "zsh" => Ok(SupportedShell::Zsh),
        "fish" => Ok(SupportedShell::Fish),
        _ => Err(format!(
            "Unsupported shell: {}. Supported shells are: bash, zsh, fish",
            shell_name
        )),
    }
}