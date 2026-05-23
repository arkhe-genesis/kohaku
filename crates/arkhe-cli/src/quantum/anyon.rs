use clap::Args;

#[derive(Args)]
pub struct AnyonArgs {
    #[arg(short, long)]
    pub braid: Option<String>,
}

pub async fn run(_args: AnyonArgs) {
    println!("Running Anyon...");
}
