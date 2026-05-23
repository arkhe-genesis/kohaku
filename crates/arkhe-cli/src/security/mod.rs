use clap::Subcommand;

#[derive(Subcommand)]
pub enum SecurityCommand {
    Scan,
}

pub async fn handler(args: SecurityCommand) {
    match args {
        SecurityCommand::Scan => println!("Running Security Scan..."),
    }
}
