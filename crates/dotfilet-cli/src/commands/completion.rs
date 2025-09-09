use std::io;

use clap::CommandFactory;
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

impl TryFrom<Shell> for SupportedShell {
    type Error = String;

    fn try_from(shell: Shell) -> Result<Self, Self::Error> {
        match shell {
            Shell::Bash => Ok(SupportedShell::Bash),
            Shell::Zsh => Ok(SupportedShell::Zsh),
            Shell::Fish => Ok(SupportedShell::Fish),
            _ => Err(format!("Unsupported shell: {:?}", shell)),
        }
    }
}

impl CompletionCommand {
    pub(crate) fn execute(self) {
        let shell = match self.shell {
            Some(shell) => shell,
            None => match Shell::from_env() {
                Some(shell) => match SupportedShell::try_from(shell) {
                    Ok(shell) => shell,
                    Err(e) => {
                        eprintln!("Error: {}", e);
                        std::process::exit(1);
                    }
                },
                None => {
                    eprintln!("Error: Unable to detect shell from environment variables. Please specify a shell explicitly.");
                    std::process::exit(1);
                }
            },
        };

        let mut app = crate::commands::RootCommand::command();
        let shell: Shell = shell.into();

        generate(shell, &mut app, "dotfilet", &mut io::stdout());
    }
}
