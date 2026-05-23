use clap::Subcommand;

#[derive(Subcommand)]
pub enum CryptoCommand {
    Hash,
}

pub async fn handler(args: CryptoCommand) {
    match args {
        CryptoCommand::Hash => println!("Running Crypto Hash..."),
    }
}
