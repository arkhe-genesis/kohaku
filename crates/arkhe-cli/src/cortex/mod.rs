use clap::Subcommand;

#[derive(Subcommand)]
pub enum CortexCommand {
    Think,
}

pub async fn handler(args: CortexCommand) {
    match args {
        CortexCommand::Think => println!("Running Cortex Think..."),
    }
}
