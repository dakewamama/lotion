# Lotion

Post-quantum signed gossip primitive for Solana. Hash-based Winternitz one-time signatures over UDP, with the signature and public key embedded in every message for self-contained verification.

## What this is

Lotion is a research prototype demonstrating that Solana validator gossip can be migrated to a hash-based post-quantum signature scheme today, using only primitives that already exist in the ecosystem.

**This is not a validator.** It is a two-process test harness that proves the cryptographic primitive and wire format work end-to-end over real UDP. The intent is that when a validator team is ready to integrate post-quantum signing into their gossip module, the math, the message structure, and the bandwidth cost are already known and measured. Lotion is the proving ground for the layer that would later sit inside [`agave/gossip`](https://github.com/anza-xyz/agave/tree/master/gossip) or any equivalent client.

The cryptographic core is [`solana-winternitz`](https://crates.io/crates/solana-winternitz) by [Dean Little](https://github.com/deanmlittle), a 224-bit Keccak-based Winternitz one-time signature crate that has been live on Solana mainnet for over two years via the [Winternitz Vault](https://github.com/deanmlittle/solana-winternitz-vault), and was cited by Google Quantum AI in their 2026 post-quantum threat assessment.

## Why now

On April 27, 2026, two independent teams published post-quantum migration roadmaps for Solana on the same day:

- Anza's [*Securing Solana Against a Powerful Quantum Adversary*](https://www.anza.xyz/blog/securing-solana-against-a-powerful-quantum-adversary), naming FALCON as the leading candidate and SIMD-0461 as the proposed precompile path.
- Jump Crypto's [*Quantum Migration Paths for Solana*](https://jumpcrypto.com/writing/quantum-migration-paths-for-solana/), naming SIMD-0416 and confirming gossip-layer Ed25519 as a quantum-vulnerable surface.

Both posts explicitly call out gossip, Turbine, and QUIC as Ed25519-dependent components that will need post-quantum migration. Neither post ships a reference implementation for the network layer. Lotion fills that gap with the most conservative cryptographic assumption stack available: pure hash functions, no lattices, no new math.

## How it works

The system consists of two binaries, `sender` and `receiver`, communicating over UDP. They are standalone test processes, not validators.

The sender generates a fresh Winternitz keypair for each gossip message (Winternitz keys are one-time-use), constructs a `GossipMessage` shaped after Solana's [`CrdsValue`](https://github.com/anza-xyz/agave/blob/master/gossip/src/crds_value.rs), signs the serialized payload, and transmits the full message including the public key.

The receiver listens on UDP, deserializes incoming bytes, and verifies the signature using the public key embedded in the message itself. No prior key knowledge is required.

```
sender (port 9001) ──UDP──> receiver (port 9000)
  │                              │
  ├─ generate keypair             ├─ deserialize
  ├─ sign GossipMessage           ├─ verify signature
  └─ serialize + send             └─ log result
```

The `GossipMessage` struct mirrors `CrdsValue` from agave: a signature over a serialized data payload, a hash for deduplication, and (unique to one-time schemes) the public key needed for verification. When this primitive is later moved inside a real validator's gossip service, the struct shape and verification path can transfer directly.

## Measurements

All numbers below taken on a single x86-64 core in release mode. Debug builds run 30-200x slower due to disabled optimizations.

| Metric | Lotion (Winternitz) | Ed25519 reference |
|---|---|---|
| Signature size | 896 bytes | 64 bytes |
| Public key size | 896 bytes (embedded) | 32 bytes |
| Total wire size | ~1,870 bytes | ~130 bytes |
| Sign time | ~10ms | ~0.05ms |
| Verify time | ~2ms | ~0.12ms |
| Throughput tested | 10 msg/sec, 100% verified | — |

Ed25519 reference numbers from [ed25519.cr.yp.to](https://ed25519.cr.yp.to/) and standard libsodium benchmarks. Lotion's bandwidth cost is approximately 14x Ed25519. Verification cost is approximately 17x but remains comfortably within real-time gossip budgets — a single core can verify ~500 signed messages per second.

## Honest constraints

This is a v1 prototype. Three things it does not claim to be:

**Not a validator and not production-ready for validator deployment.** Real gossip propagation involves push/pull/prune logic, peer discovery, bloom filters over CRDS state, and signature aggregation. Lotion implements the per-hop cryptographic primitive only. Integration into a validator client is future work.

**Not many-time signing.** Winternitz is one-time-use by construction. Production deployment requires a Merkle tree of one-time keys (XMSS or SPHINCS+ style). Lotion generates a fresh keypair per message to remain cryptographically correct, at the cost of 10ms key generation per send.

**Not side-channel hardened.** This uses standard Rust implementations of Keccak with no constant-time guarantees. Production deployment requires audited cryptographic primitives.

## Roadmap

- v0.2: Merkle-tree extension for many-time signing (XMSS-style)
- v0.3: Multi-peer fanout matching Solana's push protocol semantics
- v0.4: Sidecar proxy in front of stock agave validators
- Long term: contribute reference benchmarks to SIMD-0461 / SIMD-0416 discussion

## Building and running

Requires Rust 1.94+ and a recent Linux toolchain.

```
cargo run --release --bin receiver
```

In a second terminal:

```
cargo run --release --bin sender
```

The receiver will print verified messages and per-batch statistics every 10 packets.

## Credits

- [`solana-winternitz`](https://crates.io/crates/solana-winternitz) crate by [Dean Little](https://github.com/deanmlittle) ([Blueshift](https://blueshift.gg))
- [Winternitz Vault](https://github.com/deanmlittle/solana-winternitz-vault) — on-chain post-quantum primitive cited by Google Quantum AI
- Solana gossip protocol reference: [agave gossip module](https://github.com/anza-xyz/agave/tree/master/gossip) by [Anza](https://www.anza.xyz)
- Post-quantum migration roadmaps:
  - Anza — [Resnick & Kim, 2026](https://www.anza.xyz/blog/securing-solana-against-a-powerful-quantum-adversary)
  - Jump Crypto — [Rubin, Cesena & Latif, 2026](https://jumpcrypto.com/writing/quantum-migration-paths-for-solana/)

## License

MIT