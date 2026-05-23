pub mod qkd;
pub mod anyon;
pub mod ftqc;

use clap::{Subcommand, Args};

#[derive(Subcommand)]
pub enum QuantumCommand {
    Qkd(qkd::QkdArgs),
    Entangle(EntangleArgs),
    Teleport(TeleportArgs),
    Boost(BoostArgs),
    SurfaceCode(SurfaceCodeArgs),
    Anyon(anyon::AnyonArgs),
    Simulate(SimulateArgs),
    Ftqc(ftqc::FtqcArgs),
}

#[derive(Args)]
pub struct EntangleArgs {}

#[derive(Args)]
pub struct TeleportArgs {}

#[derive(Args)]
pub struct BoostArgs {}

#[derive(Args)]
pub struct SurfaceCodeArgs {}

#[derive(Args)]
pub struct SimulateArgs {}

pub async fn handler(args: QuantumCommand) {
    match args {
        QuantumCommand::Qkd(a) => qkd::run(a).await,
        QuantumCommand::Entangle(_) => { println!("Running Entangle..."); }
        QuantumCommand::Teleport(_) => { println!("Running Teleport..."); }
        QuantumCommand::Boost(_) => { println!("Running Boost..."); }
        QuantumCommand::SurfaceCode(_) => { println!("Running SurfaceCode..."); }
        QuantumCommand::Anyon(a) => anyon::run(a).await,
        QuantumCommand::Simulate(_) => { println!("Running Simulate..."); }
        QuantumCommand::Ftqc(a) => ftqc::run(a).await,
    }
}
