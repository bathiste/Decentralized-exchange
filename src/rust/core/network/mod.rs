/**
 * Network Module
 * Peer-to-peer communication layer with DHT, peer discovery,
 * authenticated identities, and Tor/I2P transport support.
 *
 * Threat model:
 * - Sybil attacks: Defended by authenticated peer identities
 * - Eclipse attacks: Defended by diverse peer sampling and reputation
 * - Routing manipulation: Defended by signed network messages
 * - DHT poisoning: Defended by verified key lookups and validation
 * - Replay attacks: Defended by nonce-based message protection
 * - Traffic correlation: Defended by end-to-end encryption and routing
 * - Malicious peers: Defended by reputation system and capability validation
 * - Network-level metadata leakage: Defended by Tor/I2P routing
 *   and end-to-end encryption
 */
use crate::cryptography::{PublicKey, Signature};
use crate::identity::PeerId;

/** Network message types transmitted between peers. */
#[derive(Debug, Clone)]
pub enum NetworkMessage {
    PeerDiscovery(PeerDiscoveryMessage),
    DHTMessage(DHTMessage),
    OrderMessage(OrderMessageContent),
    TradeMessage(TradeMessageContent),
    ArbitrationMessage(ArbitrationMessageContent),
    Heartbeat(HeartbeatMessage),
    Disconnect(DisconnectMessage),
}

/** Peer discovery message types. */
#[derive(Debug, Clone)]
pub enum PeerDiscoveryMessage {
    Request,
    Response(Vec<PeerInfo>),
}

/** Peer information for DHT routing. */
#[derive(Debug, Clone)]
pub struct PeerInfo {
    pub id: PeerId,
    pub address: PeerAddress,
    pub capabilities: PeerCapabilities,
    pub reputation: f64,
}

/** Peer address types. */
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PeerAddress {
    TcpAddr(std::net::SocketAddr),
    TorOnion(TorOnionAddr),
    I2PDest(I2PDestAddr),
}

/** Tor onion service address (32-byte). */
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TorOnionAddr(pub [u8; 32]);

/** I2p destination address (32-byte). */
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct I2PDestAddr(pub [u8; 32]);

/** Peer capabilities bitmask. */
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PeerCapabilities {
    pub order_validation: bool,
    pub escrow_support: bool,
    pub arbitration_support: bool,
    pub tor_support: bool,
    pub i2p_support: bool,
    pub bitcoin_support: bool,
    pub monero_support: bool,
}

/** DHT message types. */
#[derive(Debug, Clone)]
pub enum DHTMessage {
    FindNode {
        target: PeerId,
    },
    FindNodeResponse {
        nodes: Vec<PeerInfo>,
    },
    FindValue {
        target: PeerId,
    },
    FindValueResponse {
        value: Option<Vec<PeerInfo>>,
        nodes: Vec<PeerInfo>,
    },
    Store {
        key: PeerId,
        value: Vec<u8>,
    },
}

/** Heartbeat message for connection liveness. */
#[derive(Debug, Clone)]
pub struct HeartbeatMessage {
    pub sender: PeerId,
    pub timestamp: u64,
    pub nonce: [u8; 32],
}

/** Disconnect message. */
#[derive(Debug, Clone)]
pub struct DisconnectMessage {
    pub reason: DisconnectReason,
}

/** Disconnect reasons. */
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisconnectReason {
    Shutdown,
    ResourceExceeded,
    InvalidMessage,
    Timeout,
    Requested,
}

/** Signed network message with sender identity. */
#[derive(Debug, Clone)]
pub struct SignedNetworkMessage {
    pub message: NetworkMessage,
    pub signature: Signature,
    pub sender_key: PublicKey,
}

/** Network event emitted by the P2P layer. */
#[derive(Debug)]
pub enum NetworkEvent {
    PeerConnected(PeerId),
    PeerDisconnected(PeerId, DisconnectReason),
    MessageReceived(PeerId, NetworkMessage),
    RouteUpdated(PeerId, Vec<PeerInfo>),
    CapabilitiesAdvertised(PeerId, PeerCapabilities),
}

/** Configuration for the network layer. */
#[derive(Debug, Clone)]
pub struct NetworkConfig {
    pub max_connections: usize,
    pub peer_ttl: std::time::Duration,
    pub dht_refresh_interval: std::time::Duration,
    pub tor_proxy: Option<std::net::SocketAddr>,
    pub i2p_proxy: Option<std::net::SocketAddr>,
    pub tor_traffic_percentage: f64,
    pub i2p_traffic_percentage: f64,
    pub reputation_update_interval: std::time::Duration,
    pub min_reputation: f64,
    pub invalid_message_threshold: usize,
}

/** Result of a network operation. */
#[derive(Debug)]
pub enum NetworkResult {
    Success,
    Error(String),
    WouldBlock,
    NotAvailable,
}

/** Network error types. */
#[derive(Debug)]
pub enum NetworkError {
    InvalidMessage,
    SignatureVerificationFailed,
    PeerNotConnected(PeerId),
    DHTLookupFailed(String),
    RoutingTableFull,
    ConnectionLimitExceeded,
    InvalidPeerAddress,
    TransportError(String),
    ReplayAttackDetected,
    PeerBanned(PeerId),
}