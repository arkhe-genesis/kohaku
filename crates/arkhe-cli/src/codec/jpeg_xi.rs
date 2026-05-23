use clap::Args;

#[derive(Args)]
pub struct JpegArgs {
    #[arg(short, long)]
    pub file: Option<String>,
}

pub async fn run(_args: JpegArgs) {
    println!("Running JPEG XI codec...");
}
