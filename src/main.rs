use std::env;

use iroh::{protocol::Router, Endpoint, endpoint::presets};
use iroh_blobs::{store::mem::MemStore, BlobsProtocol, ticket::BlobTicket};

fn print_help() {
    println!("toss - simple peer-to-peer file transfer\n");
    println!("Usage:");
    println!("  toss -s <filename>             send a file");
    println!("  toss -r <passcode> <filename>  receive a file, saved as <filename>");
    println!("  toss -h                        show this help");
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_help();
        return Ok(());
    }

    match args[1].as_str() {
        "-h" => print_help(),
        "-s" => {
            let Some(filename) = args.get(2) else {
                println!("Missing filename. Usage: toss -s <filename>");
                return Ok(());
            };
            send(filename).await?;
        }
        "-r" => {
            let Some(passcode) = args.get(2) else {
                println!("Missing passcode. Usage: toss -r <passcode> <filename>");
                return Ok(());
            };
            let Some(filename) = args.get(3) else {
                println!("Missing filename. Usage: toss -r <passcode> <filename>");
                return Ok(());
            };
            receive(passcode, filename).await?;
        }
        other => {
            println!("Unknown flag: {other}");
            print_help();
        }
    }

    Ok(())
}

async fn send(filename: &str) -> anyhow::Result<()> {
    println!(".....  this is toss ....  ");
    let endpoint = Endpoint::bind(presets::N0).await?;
    let store = MemStore::new();

    let file_bytes = std::fs::read(filename)?;
    let tag = store.add_slice(&file_bytes).await?;

    let blobs_protocol = BlobsProtocol::new(&store, None);
    let router = Router::builder(endpoint.clone())
        .accept(iroh_blobs::ALPN, blobs_protocol)
        .spawn();

    endpoint.online().await;

    let endpoint_id = endpoint.id();
    let ticket = BlobTicket::new(endpoint_id.into(), tag.hash, tag.format);

    println!("\nShare this exact passcode ticket:\n\n{ticket}");
    println!("\n[Keeping server alive. Press Ctrl+C to cancel the toss]");
    tokio::signal::ctrl_c().await?;
    router.shutdown().await?;
    Ok(())
}

async fn receive(passcode: &str, filename: &str) -> anyhow::Result<()> {
    println!(".....  this is toss ....  ");
    let endpoint = Endpoint::bind(presets::N0).await?;
    let store = MemStore::new();

    let ticket: BlobTicket = passcode.parse()?;

    println!("Connecting directly to the sender node...");
    let downloader = store.downloader(&endpoint);
    downloader
        .download(ticket.hash(), Some(ticket.addr().id))
        .await?;

    let abs_path = std::path::absolute(filename)?;
    store.blobs().export(ticket.hash(), abs_path).await?;
    println!("🎯 Success! Stored incoming data as '{filename}'");
    Ok(())
}