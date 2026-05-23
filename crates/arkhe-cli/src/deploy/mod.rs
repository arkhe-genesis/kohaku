use clap::Subcommand;

#[derive(Subcommand)]
pub enum DeployCommand {
    Execute,
}

pub async fn handler(args: DeployCommand) {
    match args {
        DeployCommand::Execute => println!("Running Deploy Execute..."),
    }
}
