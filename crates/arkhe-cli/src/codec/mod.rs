pub mod mp3_xi;
pub mod jpeg_xi;

use clap::Subcommand;

#[derive(Subcommand)]
pub enum CodecCommand {
    Mp3(mp3_xi::Mp3Args),
    Jpeg(jpeg_xi::JpegArgs),
}

pub async fn handler(args: CodecCommand) {
    match args {
        CodecCommand::Mp3(a) => mp3_xi::run(a).await,
        CodecCommand::Jpeg(a) => jpeg_xi::run(a).await,
    }
}
