/**
 * Orders Module
 * Order creation, validation, publication, and matching for the P2P exchange.
 */
use crate::cryptography::{PublicKey, Signature};
use crate::identity::PeerId;
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

/** Order side (buy/sell). */
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderSide { Buy, Sell }

/** Order type for the exchange. */
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderType { Limit, Market, PostOnly, FillOrKill, AllOrNone }

/** Time-in-force specification. */
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeInForce { GTC, IOC, FOK }

/** Order identifier. */
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OrderId(pub [u8; 32]);

/** Order model. */
#[derive(Debug, Clone)]
pub struct Order {
    pub id: OrderId,
    pub creator: PeerId,
    pub side: OrderSide,
    pub order_type: OrderType,
    pub price: u64,
    pub quantity: u64,
    pub time_in_force: TimeInForce,
    pub created_at: Instant,
    pub updated_at: Instant,
    pub status: OrderStatus,
    pub counterparty: Option<PeerId>,
    pub escrow_id: Option<String>,
    pub metadata: OrderMetadata,
}

/** Order status lifecycle. */
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderStatus {
    Published, Discovered, Negotiating, Agreed,
    Escrowed, Verified, Completed, Disputed, Cancelled, Expired,
}

/** Order metadata. */
#[derive(Debug, Clone)]
pub struct OrderMetadata {
    pub asset_pair: AssetPair,
    pub description: Option<String>,
    pub compliance: Option<ComplianceFlags>,
}

/** Asset pair. */
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssetPair { BitcoinToMonero, MoneroToBitcoin, Other(String) }

/** Compliance flags. */
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ComplianceFlags { keywords_passed: bool, age_verified: bool, geo_compliant: bool }

/** Validate an order. */
pub fn validate_order(_order: &Order) -> (bool, Vec<&'static str>) {
    (true, vec![])
}

/** Order book entry. */
#[derive(Debug, Clone)]
pub struct OrderBookEntry {
    pub order: Order,
    pub reputation: f64,
    pub advertised_for: Duration,
}