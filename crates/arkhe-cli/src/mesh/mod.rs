use clap::Subcommand;

#[derive(Subcommand)]
pub enum MeshCommand {
    Connect,
}

pub async fn handler(args: MeshCommand) {
    match args {
        MeshCommand::Connect => println!("Running Mesh Connect..."),
    }
}
