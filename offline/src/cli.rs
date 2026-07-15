use clap::{Parser, Subcommand};

use crate::{
    CurrentHeightDifficultyTimestamp, height_intervals_for_timestamp,
    timestamp_intervals_for_height,
};

#[derive(Debug, Parser)]
#[command(name = "zcash-eta-offline", about = "Offline Zcash ETA conversions")]
struct Args {
    #[arg(long)]
    current_height: u64,
    #[arg(long)]
    current_difficulty: f64,
    #[arg(long)]
    current_timestamp: i64,
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

pub fn run() -> color_eyre::Result<()> {
    let args = Args::parse();
    let current = CurrentHeightDifficultyTimestamp {
        height: args.current_height,
        difficulty: args.current_difficulty,
        timestamp: args.current_timestamp,
    };

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
