pub mod theosis;
pub mod ethics;

use clap::{Subcommand, Args};

#[derive(Subcommand)]
pub enum TheoCommand {
    Monitor,
    Reason(ReasonArgs),
    Apophatic(ApophaticArgs),
    SacredText(SacredTextArgs),
    Audit(AuditArgs),
    Theosis(theosis::TheosisArgs),
    Ethics(ethics::EthicsArgs),
}

#[derive(Args)]
pub struct ReasonArgs {}

#[derive(Args)]
pub struct ApophaticArgs {}

#[derive(Args)]
pub struct SacredTextArgs {}

#[derive(Args)]
pub struct AuditArgs {}

pub async fn handler(args: TheoCommand) {
    match args {
        TheoCommand::Monitor => println!("Running Monitor..."),
        TheoCommand::Reason(_) => println!("Running Reason..."),
        TheoCommand::Apophatic(_) => println!("Running Apophatic..."),
        TheoCommand::SacredText(_) => println!("Running SacredText..."),
        TheoCommand::Audit(_) => println!("Running Audit..."),
        TheoCommand::Theosis(a) => theosis::run(a).await,
        TheoCommand::Ethics(a) => ethics::run(a).await,
    }
}
