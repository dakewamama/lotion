use anyhow::Result;
use lotion::GossipMessage;
use std::time::Instant;
use tokio::net::UdpSocket;

const BIND_ADDR: &str = "127.0.0.1:9000";
const BUFFER_SIZE: usize = 4096;

#[tokio::main]
async fn main() -> Result<()> {
    let socket = UdpSocket::bind(BIND_ADDR).await?;
    println!("receiver bound to {}", BIND_ADDR);
    println!("waiting for gossip...");

    let mut buf = vec![0u8; BUFFER_SIZE];
    let mut received: u64 = 0;
    let mut verified: u64 = 0;
    let mut failed: u64 = 0;

    loop {
        let (len, src) = socket.recv_from(&mut buf).await?;
        received += 1;

        let message: GossipMessage = match bincode::deserialize(&buf[..len]) {
            Ok(m) => m,
            Err(e) => {
                failed += 1;
                println!("[{}] deserialization failed from {}: {}", received, src, e);
                continue;
            }
        };

        let verify_start = Instant::now();
        let valid = message.verify();
        let verify_elapsed = verify_start.elapsed();

        if valid {
            verified += 1;
            println!(
                "[{}] from {} | {} bytes | verify {:.2}ms | sender={:?} | payload={:?} | VERIFIED",
                received,
                src,
                len,
                verify_elapsed.as_secs_f64() * 1000.0,
                String::from_utf8_lossy(&message.data.sender),
                String::from_utf8_lossy(&message.data.payload),
            );
        } else {
            failed += 1;
            println!(
                "[{}] from {} | {} bytes | verify {:.2}ms | VERIFICATION FAILED",
                received,
                src,
                len,
                verify_elapsed.as_secs_f64() * 1000.0
            );
        }

        if received % 10 == 0 {
            println!(
                "--- stats: received={} verified={} failed={} ---",
                received, verified, failed
            );
        }
    }
}