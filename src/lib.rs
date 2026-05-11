use serde::{Deserialize, Serialize};
use solana_nostd_keccak::hash;
use solana_winternitz::{
    HASH_LENGTH,
    privkey::WinternitzPrivkey,
    pubkey::WinternitzPubkey,
    signature::WinternitzSignature,
};

/// Total signature size: 32 hash chains × HASH_LENGTH bytes each.
pub const SIGNATURE_BYTES: usize = HASH_LENGTH * 32;

/// Mirrors the shape of Solana's CrdsValue:
/// a signature over a data payload, plus a hash for deduplication.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GossipMessage {
    #[serde(with = "serde_big_array::BigArray")]
    pub signature: [u8; SIGNATURE_BYTES],
    pub data: GossipData,
    pub hash: [u8; HASH_LENGTH],
}

/// ContactInfo-style payload for a gossip message.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GossipData {
    pub sender: Vec<u8>,
    pub wallclock: u64,
    pub payload: Vec<u8>,
}

impl GossipMessage {
    /// Build and sign a new gossip message with a Winternitz private key.
    pub fn sign(privkey: &WinternitzPrivkey, data: GossipData) -> Self {
        let data_bytes = bincode::serialize(&data).expect("serialize gossip data");
        let signature: [u8; SIGNATURE_BYTES] = privkey.sign(&data_bytes).into();
        let full_hash = hash(&data_bytes);
        let hash: [u8; HASH_LENGTH] = full_hash[..HASH_LENGTH]
            .try_into()
            .expect("hash slice must be HASH_LENGTH bytes");

        Self {
            signature,
            data,
            hash,
        }
    }

    /// Verify the signature against a public key.
    pub fn verify(&self, pubkey: &WinternitzPubkey) -> bool {
        let data_bytes = match bincode::serialize(&self.data) {
            Ok(b) => b,
            Err(_) => return false,
        };

        let signature = WinternitzSignature::from(self.signature);
        signature.verify(&data_bytes, pubkey)
    }
}