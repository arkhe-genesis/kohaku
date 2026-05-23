use clap::Subcommand;

#[derive(Subcommand)]
pub enum TelemetryCommand {
    Ping,
}

pub async fn handler(args: TelemetryCommand) {
    match args {
        TelemetryCommand::Ping => println!("Running Telemetry Ping..."),
    }
}
