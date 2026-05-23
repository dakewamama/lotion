# Lotion

A learning project exploring post-quantum signatures over UDP, shaped after Solana's gossip message format.

## What this is

Lotion is a two-binary test harness. A sender generates a Winternitz one-time keypair, signs a UDP message shaped after Solana's `CrdsValue` struct, and transmits it. A receiver verifies the signature using the public key embedded in the message. That's the whole system.

This is not a validator. It is not integrated with agave's gossip module. It does not implement push, pull, prune, peer discovery, CRDS state synchronization, replay protection, or any other gossip-protocol behavior. It demonstrates one thing: that a hash-based post-quantum signature can authenticate a single gossip-shaped message over real UDP, with measured cost.

## Why post-quantum at all

Today's signature schemes — Ed25519 on Solana, secp256k1 on Bitcoin and Ethereum — rest on the assumption that recovering a private key from a public key is computationally infeasible. A sufficiently capable quantum computer breaks that assumption. The 2026 disclosures from Google Research and Oratomic shifted the estimated probability of a cryptographically relevant quantum computer within five years from "negligible" to "a few percent." Both Anza and Jump Crypto published migration roadmaps on April 27, 2026, naming multiple Ed25519-dependent surfaces in Solana that will eventually need to migrate: transaction signatures, consensus votes, and the gossip protocol among them.

When that migration happens, the gossip layer is one of the surfaces that will need a post-quantum signature scheme. Lotion explores what one such scheme — hash-based Winternitz — looks like and costs in that context.

## What was built

Two Rust binaries communicating over UDP.

- `sender`: generates a fresh Winternitz keypair for each message (Winternitz is one-time-use), builds a `GossipMessage` struct shaped after Solana's `CrdsValue` (signature, public key, data payload, hash), serializes with bincode, sends over UDP on a 100ms timer.
- `receiver`: binds a UDP socket, deserializes incoming bytes, verifies the embedded signature against the embedded public key, logs the result.

The cryptographic primitive is the `solana-winternitz` crate by Dean Little (Blueshift), used as a standard dependency. The crate has been live on Solana mainnet for two years via the Winternitz Vault.

## What was measured

All numbers on a single x86-64 core in release mode.

| | Lotion | Ed25519 reference |
|---|---|---|
| Signature size | 896 bytes | 64 bytes |
| Public key size | 896 bytes | 32 bytes |
| Total UDP payload | ~1,870 bytes | ~130 bytes |
| Sign time | ~10ms | ~0.05ms |
| Verify time | ~2ms | ~0.12ms |

These are measurements of *Lotion specifically* — a single sender talking to a single receiver over localhost UDP. They are not measurements of post-quantum gossip at scale. They are a first data point.

## What was not built

- No integration with agave or any validator client
- No push/pull/prune logic
- No CRDS state synchronization
- No peer discovery or multi-peer fanout
- No replay protection or wallclock freshness enforcement
- No equivocation detection
- No stable validator identity binding (Winternitz one-time keys have no persistent identity; production would need an XMSS-style Merkle tree)
- No side-channel hardening of the cryptographic implementation

## Roadmap

This is a learning project. If it grows:
- v0.2: Merkle-tree extension for many-time signing
- v0.3: Wallclock-based replay protection
- v0.4: Multi-peer fanout

## Background reading

- [Anza — Securing Solana Against a Powerful Quantum Adversary](https://www.anza.xyz/blog/securing-solana-against-a-powerful-quantum-adversary)
- [Jump Crypto — Quantum Migration Paths for Solana](https://jumpcrypto.com/writing/quantum-migration-paths-for-solana/)
- [Dean Little — solana-winternitz](https://crates.io/crates/solana-winternitz)
- [Anza — agave gossip module](https://github.com/anza-xyz/agave/tree/master/gossip)

## License

MIT