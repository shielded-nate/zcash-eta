use clap::{Parser, Subcommand};

use crate::fetch_current_height_difficulty_timestamp;
use zcash_eta_offline::{height_intervals_for_timestamp, timestamp_intervals_for_height};

#[derive(Debug, Parser)]
#[command(
    name = "zcash-eta-headless",
    about = "Fetch current data then convert Zcash ETA"
)]
struct Args {
    #[arg(long)]
    endpoint: String,
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    HeightToTimestamp {
        #[arg(long)]
        target_height: u64,
    },
    TimestampToHeight {
        #[arg(long)]
        target_timestamp: i64,
    },
}

pub async fn run() -> color_eyre::Result<()> {
    let args = Args::parse();
    let current = fetch_current_height_difficulty_timestamp(&args.endpoint).await?;

    match args.command {
        Command::HeightToTimestamp { target_height } => {
            println!(
                "{}",
                serde_json::to_string_pretty(&timestamp_intervals_for_height(
                    current,
                    target_height
                ))?
            );
        }
        Command::TimestampToHeight { target_timestamp } => {
            println!(
                "{}",
                serde_json::to_string_pretty(&height_intervals_for_timestamp(
                    current,
                    target_timestamp
                ))?
            );
        }
    }

    Ok(())
}
