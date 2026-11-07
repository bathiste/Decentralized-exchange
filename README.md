# Decentralized-exchange

A fully decentralised, privacy-focused peer-to-peer exchange with no servers, admin authorities or trusted middlemen.

## Project Goals

A fully decentralised, privacy-focused peer-to-peer exchange with no servers, admin authorities or trusted middlemen, but with a fully automated filter system for potentially illegal keywords, e.g. drugs, underage content, or living things, etc.

The platform will use a distributed P2P order book, like a blockchain normally does, for users to trade cryptocurrency and possibly real-life items, with payments secured through cryptographic escrow and disputes handled by a matching system that randomly selects independent (authorised) arbitrators.

The network will use Tor and I2P for peer discovery and decentralised routing, end-to-end encryption to protect packet content, and a mixture of different post-quantum encryption protocols to make it future-proof. The platform will also use privacy-preserving transaction protocols to minimise the exposure of identities, payment information and trading activity.

The end goal is to create a censorship-resistant, mesh-based and trust-minimised exchange where users retain control of what they own, their identities and their transactions, without relying on a central server or authority.

## Components

| Component | Description |
|---|---|
| **Architecture** | Fully decentralised P2P network with no central servers or administrators. |
| **Order Book** | Distributed peer-to-peer order book for creating, discovering and matching trades. |
| **Trading** | Cryptocurrency-to-cryptocurrency and cryptocurrency-to-real-world goods. |
| **Escrow** | Cryptographic escrow to secure funds until trade conditions are fulfilled. |
| **Arbitration** | 10 randomly selected arbitrators; first valid acceptance handles disputes. |
| **Networking** | Tor + I2P routing to reduce IP exposure and network-level tracking. |
| **Encryption** | End-to-end encryption so intermediate nodes cannot read packet contents. |
| **Cryptography** | Hybrid classical and post-quantum cryptography, including ML-KEM. |
| **Privacy** | Minimise identity exposure, transaction metadata and payment correlation. |
| **Censorship Resistance** | No central authority capable of independently controlling the exchange. |

 

---

## Development Roadmap

**Phase 1 – Project Foundation**
- Rust workspace and TypeScript application setup.
- Modular project architecture.
- Initial trade-state machine.
- Error handling, logging and unit tests.

**Phase 2 – Core Protocol & Identity**
- Cryptographic peer identities and key management.
- X25519 and ML-KEM-1024 key exchange research.
- Ed25519 and ML-DSA-87 signatures.
- End-to-end encrypted messaging.
- Message authentication and replay protection.
- Initial peer discovery and DHT record structures.
- Tor and I2P transport abstractions.

**Future Phases**

| Phase | Description |
|---|---|
| 3 | P2P networking and DHT |
| 4 | Distributed order book |
| 5 | Bitcoin and Monero integration |
| 6 | Cryptographic escrow and settlement |
| 7 | Decentralised arbitration |
| 8 | Financial infrastructure |
| 9 | Security and privacy auditing |
| 10 | User interface |
| 11 | Integration testing |
| 12 | Release preparation |

## Project Status

Early development. Phases 1 and 2 establish the foundations for subsequent decentralised networking and trading functionality.
