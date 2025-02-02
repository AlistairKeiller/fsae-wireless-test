mod client;
mod mqtt;
mod server;
use clap::Parser;
use server::post;
use client::subscriber;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    server: bool,

    #[arg(short, long, default_value_t = 8, requires = "server")]
    payload: usize,

    #[arg(short, long, default_value = "127.0.0.1")]
    ip: String,

    #[arg(short, long, default_value_t = 10)]
    queue: usize,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    println!("{:?}", args);

    if args.server {
        std::thread::spawn(mqtt::mqttd);

        tokio::spawn(post(args.queue, args.payload, args.ip));
    } else {
        tokio::spawn(subscriber(args.queue, args.ip));
    }

    tokio::signal::ctrl_c().await?;

    Ok(())
}
