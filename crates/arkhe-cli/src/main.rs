mod cli;
mod quantum;
mod codec;
mod theo;
mod mesh;
mod sim;
mod cortex;
mod security;
mod deploy;
mod mcp;
mod bridge;
mod crypto;
mod telemetry;
mod lib_module;
mod ossi;

use clap::Parser;
use cli::commands::Commands;

#[derive(Parser)]
#[command(name = "arkhe")]
#[command(version = "∞.Ω.584.0")]
#[command(about = "ARKHE OS Constitutional Runtime — Windows Native CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Verify(args) => cli::commands::cmd_verify(args).await,
        Commands::Substrate(args) => cli::commands::cmd_substrate(args).await,
        Commands::Quantum(args) => quantum::handler(args).await,
        Commands::Codec(args) => codec::handler(args).await,
        Commands::Theo(args) => theo::handler(args).await,
        Commands::Mesh(args) => mesh::handler(args).await,
        Commands::Sim(args) => sim::handler(args).await,
        Commands::Cortex(args) => cortex::handler(args).await,
        Commands::Security(args) => security::handler(args).await,
        Commands::Deploy(args) => deploy::handler(args).await,
        Commands::Mcp(args) => mcp::handler(args).await,
        Commands::Bridge(args) => bridge::handler(args).await,
        Commands::Crypto(args) => crypto::handler(args).await,
        Commands::Telemetry(args) => telemetry::handler(args).await,
        Commands::Lib(args) => lib_module::handler(args).await,
        Commands::Ossi(args) => ossi::handler(args).await,
    }
}
