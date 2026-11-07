/**
 * Cryptography Module
 *
 * Cryptographic primitives and key exchange using established libraries.
 * Hybrid classical + post-quantum key exchange using established protocols.
 *
 * Uses only vetted, constant-time implementations from established crates:
 * - X25519 for ECDH key exchange
 * - Ed25519 for signatures
 * - ML-KEM-1024 (post-quantum KEM)
 * - ML-DSA-87 (post-quantum signatures)
 * - HKDF-SHA-512 for key derivation
 * - AES-256-GCM and ChaCha20-Poly1305 for authenticated encryption
 *
 * Never implement custom cryptographic primitives.
 * Always use established crate APIs.
 *
 * Threat model:
 * - Forward secrecy: Supported via ephemeral key exchange
 * - Key compromise: Limited forward secrecy via ephemeral keys
 * - Replay attacks: Protected via nonce-based message protection
 * - Man-in-the-middle: Protected by authenticated key exchange
 * - Side-channel attacks: Mitigated via constant-time implementations
 */
use crypto_bigint::Mod;
use elliptic_curve::{
    CurveArithmetic, IsHigh, ProjectiveCurve, ScalarRepresentation, Validated,
};
use mL_dsa::ml_dsa::MLDSA;
use mL_kem::ml_kem::MLKEM;
use sha2::{Digest, Sha512};

/** Cryptographic key types. */
pub mod key;

/** Signature types and operations. */
pub mod signature;

/** Key exchange protocols. */
pub mod key_exchange;

/** Authenticated encryption (AEAD). */
pub mod aead;

/** Key derivation functions. */
pub mod kdf;

/** Message authentication and integrity. */
pub mod authentication;

/** Post-quantum cryptography support. */
pub mod post_quantum;

/** Export main types for convenience. */
pub use key::*;
pub use signature::*;
pub use key_exchange::*;
pub use aead::*;
pub use kdf::*;
pub use authentication::*;
pub use post_quantum::*;