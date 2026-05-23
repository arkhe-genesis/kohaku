use clap::Subcommand;

#[derive(Subcommand)]
pub enum LibCommand {
    Load,
}

pub async fn handler(args: LibCommand) {
    match args {
        LibCommand::Load => println!("Running Lib Load..."),
    }
}
