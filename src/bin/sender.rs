use anyhow::Result;
use lotion::{GossipData, GossipMessage};
use solana_winternitz::privkey::WinternitzPrivkey;
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use tokio::net::UdpSocket;
use tokio::time::sleep;

const BIND_ADDR: &str = "127.0.0.1:9001";
const TARGET_ADDR: &str = "127.0.0.1:9000";
const SEND_INTERVAL_MS: u64 = 100;

#[tokio::main]
async fn main() -> Result<()> {
    let socket = UdpSocket::bind(BIND_ADDR).await?;
    println!("sender bound to {}", BIND_ADDR);
    println!("targeting {}", TARGET_ADDR);

    let sender_id = b"lotion-node-1".to_vec();
    let mut counter: u64 = 0;

    loop {
        let privkey = WinternitzPrivkey::generate();

        let data = GossipData {
            sender: sender_id.clone(),
            wallclock: now_millis(),
            payload: format!("gossip-tick-{}", counter).into_bytes(),
        };

        let message = GossipMessage::sign(&privkey, data);
        let bytes = bincode::serialize(&message)?;

        socket.send_to(&bytes, TARGET_ADDR).await?;

        println!(
            "[{}] sent {} bytes (signature {} bytes)",
            counter,
            bytes.len(),
            message.signature.len()
        );

        counter += 1;
        sleep(Duration::from_millis(SEND_INTERVAL_MS)).await;
    }
}

fn now_millis() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}