use clap::Args;

#[derive(Args)]
pub struct EthicsArgs {
    #[arg(short, long)]
    pub rule: Option<String>,
}

pub async fn run(_args: EthicsArgs) {
    println!("Running Ethics...");
}
