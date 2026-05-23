use clap::Args;

#[derive(Args)]
pub struct QkdArgs {
    #[arg(short, long)]
    pub protocol: Option<String>,
}

pub async fn run(_args: QkdArgs) {
    println!("Running QKD...");
}
