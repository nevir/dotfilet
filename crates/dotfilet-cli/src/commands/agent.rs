use clap::Subcommand;

#[derive(Subcommand)]
pub enum AgentCommands {
    #[command(about = "Configure the sync agent to use current configuration")]
    Associate,

    #[command(about = "Start the sync agent")]
    Start,

    #[command(about = "Stop the sync agent")]
    Stop,
}

pub fn execute(command: AgentCommands, _verbose: bool, _dry_run: bool) {
    match command {
        AgentCommands::Associate => {
            println!("dotfilet agent associate: Not yet implemented");
            println!("This command will configure the sync agent for the current directory");
        }
        AgentCommands::Start => {
            println!("dotfilet agent start: Not yet implemented");
            println!("This command will start the sync agent daemon");
        }
        AgentCommands::Stop => {
            println!("dotfilet agent stop: Not yet implemented");
            println!("This command will stop the sync agent daemon");
        }
    }
}
