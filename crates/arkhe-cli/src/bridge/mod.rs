use clap::Subcommand;

#[derive(Subcommand)]
pub enum BridgeCommand {
    Link,
}

pub async fn handler(args: BridgeCommand) {
    match args {
        BridgeCommand::Link => println!("Running Bridge Link..."),
    }
}
