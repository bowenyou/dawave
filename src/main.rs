use std::{fs::File, io::Read};

use anyhow::Result;
use clap::{Parser, Subcommand};
use listener::{listen_to_blobs, play_audio};
use message::DAWaveMessage;
use sender::{compress_audio, send_message};

mod constants;
mod listener;
mod message;
mod sender;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    SuggestSong { channel: String, audio_path: String },
    Listen { node_url: String, channel: String },
    TestPlayback { audio_path: String },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Listen { node_url, channel } => {
            if let Err(e) = listen_to_blobs(&node_url, &channel).await {
                eprintln!("Error listening for blobs: {}", e);
            }
        }
        Commands::SuggestSong {
            channel,
            audio_path,
        } => {
            let mut file = File::open(audio_path)?;
            let mut audio_data = Vec::new();
            file.read_to_end(&mut audio_data)?;

            let compressed = compress_audio(&audio_data)?;

            let message = DAWaveMessage {
                channel,
                audio: compressed.clone(),
            };

            println!("{:?}, {:?}", audio_data.len(), compressed.len());

            if let Err(e) = send_message(message).await {
                eprintln!("Error sending message: {}", e);
            }
        }
        Commands::TestPlayback { audio_path } => {
            let mut file = File::open(audio_path)?;
            let mut audio_data = Vec::new();
            file.read_to_end(&mut audio_data)?;

            play_audio(audio_data)?;
        }
    }

    Ok(())
}
