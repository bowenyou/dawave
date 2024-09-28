use anyhow::Result;
use clap::{Parser, Subcommand};
use listener::listen_to_blobs;
use message::DAWaveMessage;
use sender::send_message;

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
    SendMessage { channel: String, message: String },
    Listen { node_url: String, channel: String },
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
        Commands::SendMessage { channel, message } => {
            if let Err(e) = send_message(DAWaveMessage { channel, message }).await {
                eprintln!("Error sending message: {}", e);
            }
        }
    }

    Ok(())
}
