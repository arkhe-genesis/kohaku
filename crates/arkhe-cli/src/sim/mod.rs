use clap::Subcommand;

#[derive(Subcommand)]
pub enum SimCommand {
    Run,
}

pub async fn handler(args: SimCommand) {
    match args {
        SimCommand::Run => println!("Running Sim Run..."),
    }
}
