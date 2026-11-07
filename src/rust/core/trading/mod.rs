/**
 * Trading Module
 * Trade state machine, escrow, and settlement for the P2P exchange.
 */
use crate::cryptography::{PublicKey, Signature};
use crate::identity::PeerId;
use crate::orders::{Order, OrderStatus, OrderSide, OrderType};
use std::time::Duration;

/** Trade identifier. */
pub type TradeId = u64;

/** Trade state machine states. */
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TradeStatus {
    Created,
    Published,
    Discovered,
    Negotiating,
    Agreed,
    EscrowFunded,
    DeliveryInProgress,
    Completed,
    Disputed,
    Refunded,
    Cancelled,
}

/** Trade model representing a peer-to-peer trade. */
#[derive(Debug, Clone)]
pub struct Trade {
    pub id: TradeId,
    pub buyer: PeerId,
    pub seller: PeerId,
    pub buy_order: Order,
    pub sell_order: Order,
    pub status: TradeStatus,
    pub created_at: Instant,
    pub updated_at: Instant,
    pub escrow_config: Option<EscrowConfig>,
    pub settlement_tx: Option<SettlementTx>,
    pub dispute_id: Option<String>,
}

/** Escrow configuration for a trade. */
#[derive(Debug, Clone)]
pub struct EscrowConfig {
    pub arbitrator_count: usize,
    pub timeout: Duration,
    pub multi_sig_threshold: usize,
    pub escrow_type: EscrowType,
}

/** Escrow type. */
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EscrowType {
    BitcoinMultiSig,
    MoneroMultiSig,
    AtomicSwap,
    AdaptorSignature,
}

/** Settlement transaction state. */
#[derive(Debug, Clone)]
pub struct SettlementTx {
    pub transaction_id: String,
    pub transaction_type: SettlementType,
    pub status: SettlementStatus,
    pub completed_at: Option<Instant>,
}

/** Settlement type. */
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettlementType {
    BitcoinOnChain,
    MoneroOnChain,
    AtomicSwap,
}

/** Settlement status. */
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettlementStatus {
    Pending,
    Broadcast,
    Confirmed,
    Completed,
    Failed,
}

/** Create a new trade from agreed orders. */
pub fn create_trade(
    buy_order: Order,
    sell_order: Order,
    buyer: PeerId,
    seller: PeerId,
) -> Result<Trade, &'static str> {
    // Validate that orders are compatible
    if buy_order.side != OrderSide::Buy {
        return Err("Buy order expected from buyer");
    }
    if sell_order.side != OrderSide::Sell {
        return Err("Sell order expected from seller");
    }
    if buy_order.quantity.0 != sell_order.quantity.0 {
        return Err("Order quantities must match");
    }

    let trade_id = rand::random::<u64>();
    let trade = Trade {
        id: trade_id,
        buyer,
        seller,
        buy_order,
        sell_order,
        status: TradeStatus::Created,
        created_at: Instant::now(),
        updated_at: Instant::now(),
        escrow_config: None,
        settlement_tx: None,
        dispute_id: None,
    };

    Ok(trade)
}

/** Transition trade status with validation. */
pub fn transition_trade_status(
    trade: &mut Trade,
    new_status: TradeStatus,
) -> Result<(), &'static str> {
    let allowed = match trade.status {
        TradeStatus::Created => {
            vec![TradeStatus::Published, TradeStatus::Cancelled].contains(&&new_status)
        }
        TradeStatus::Published => {
            vec![TradeStatus::Discovered, TradeStatus::Cancelled].contains(&&new_status)
        }
        TradeStatus::Discovered => {
            vec![TradeStatus::Negotiating, TradeStatus::Cancelled].contains(&&new_status)
        }
        TradeStatus::Negotiating => {
            vec![
                TradeStatus::Agreed,
                TradeStatus::Cancelled,
            ]
            .contains(&&new_status)
        }
        TradeStatus::Agreed => {
            vec![
                TradeStatus::EscrowFunded,
                TradeStatus::Disputed,
                TradeStatus::Cancelled,
            ]
            .contains(&&new_status)
        }
        TradeStatus::EscrowFunded => {
            vec![
                TradeStatus::DeliveryInProgress,
                TradeStatus::Disputed,
                TradeStatus::Refunded,
            ]
            .contains(&&new_status)
        }
        TradeStatus::DeliveryInProgress => {
            vec![
                TradeStatus::Completed,
                TradeStatus::Disputed,
            ]
            .contains(&&new_status)
        }
        TradeStatus::Completed | TradeStatus::Disputed | TradeStatus::Refunded | TradeStatus::Cancelled => {
            // Terminal states
            false
        }
    };

    if !allowed {
        return Err("Invalid trade status transition");
    }

    trade.status = new_status;
    trade.updated_at = Instant::now();
    Ok(())
}

/** Complete a trade with settlement. */
pub fn complete_trade(trade: &mut Trade) -> Result<SettlementTx, &'static str> {
    if !matches!(trade.status, TradeStatus::EscrowFunded) {
        return Err("Trade must be in EscrowFunded state to complete");
    }

    let tx_id = format!("tx-{}-{}", trade.id, trade.updated_at.elapsed().as_secs());
    let settlement = SettlementTx {
        transaction_id: tx_id,
        transaction_type: SettlementType::AtomicSwap,
        status: SettlementStatus::Broadcast,
        completed_at: Some(Instant::now()),
    };

    trade.settlement_tx = Some(settlement);
    trade.status = TradeStatus::Completed;
    trade.updated_at = Instant::now();

    Ok(settlement)
}