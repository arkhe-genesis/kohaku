use clap::Subcommand;

#[derive(Subcommand)]
pub enum McpCommand {
    Start,
}

pub async fn handler(args: McpCommand) {
    match args {
        McpCommand::Start => println!("Running Mcp Start..."),
    }
}
