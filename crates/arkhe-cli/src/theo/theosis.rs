use clap::Args;

#[derive(Args)]
pub struct TheosisArgs {
    #[arg(short, long)]
    pub stage: Option<u32>,
}

pub async fn run(_args: TheosisArgs) {
    println!("Running Theosis...");
}
