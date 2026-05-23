use clap::Args;

#[derive(Args)]
pub struct FtqcArgs {
    #[arg(short, long)]
    pub code: Option<String>,
}

pub async fn run(_args: FtqcArgs) {
    println!("Running FTQC...");
}
