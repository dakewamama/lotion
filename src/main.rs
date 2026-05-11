use lotion::{GossipData, GossipMessage};
use solana_winternitz::privkey::WinternitzPrivkey;

fn main() {
    println!("generating keypair");
    let privkey = WinternitzPrivkey::generate();
    let pubkey = privkey.pubkey();

    println!("building gossip message");
    let data = GossipData {
        sender: b"lotion-node-1".to_vec(),
        wallclock: 1234567890,
        payload: b"hello from a post-quantum validator".to_vec(),
    };

    println!("signing");
    let msg = GossipMessage::sign(&privkey, data);

    println!("message hash: {}", hex_short(&msg.hash));
    println!("signature size: {} bytes", msg.signature.len());

    println!("verifying");
    if msg.verify(&pubkey) {
        println!("signature valid");
    } else {
        println!("signature INVALID");
    }
}

fn hex_short(bytes: &[u8]) -> String {
    bytes.iter().take(8).map(|b| format!("{:02x}", b)).collect()
}