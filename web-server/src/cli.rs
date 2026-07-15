use std::net::SocketAddr;

use clap::Parser;

use crate::run_server;

#[derive(Debug, Parser)]
#[command(
    name = "zcash-eta-web-server",
    about = "HTTP server for Zcash ETA conversions"
)]
struct Args {
    #[arg(long, default_value = "127.0.0.1:3000")]
    listen: SocketAddr,
}

pub async fn run() -> color_eyre::Result<()> {
    let args = Args::parse();
    run_server(args.listen).await
}
