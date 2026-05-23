use clap::Args;

#[derive(Args)]
pub struct Mp3Args {
    #[arg(short, long)]
    pub file: Option<String>,
}

pub async fn run(_args: Mp3Args) {
    println!("Running MP3 XI codec...");
}
