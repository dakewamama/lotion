use solana_winternitz::privkey::WinternitzPrivkey;

fn main() {
    println!("generating private key...");
    let privkey = WinternitzPrivkey::generate();

    println!("deriving public key...");
    let pubkey = privkey.pubkey();

    println!("signing message...");
    let message = b"lotion: post-quantum gossip primitive";
    let signature = privkey.sign(message);

    println!("done");
    println!("pubkey: {:?}", pubkey);
    println!("signature length: {} bytes", std::mem::size_of_val(&signature));
}