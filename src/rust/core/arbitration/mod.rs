/**
 * Arbitration Module
 * Decentralised arbitration mechanism for dispute resolution.
 * Select 10 eligible arbitrators using verifiable randomness.
 * First eligible arbitrator to accept receives the case.
 */
use crate::cryptography::{PublicKey, Signature};
use crate::identity::PeerId;
use std::time::Duration;

/** Arbitrator selection result. */
#[derive(Debug, Clone)]
pub struct ArbitratorSelection {
    pub arbitrators: Vec<PeerId>,
    pub selection_proof: Vec<u8>,
    pub round_number: u64,
}

/** Arbitrator acceptance. */
#[derive(Debug, Clone)]
pub struct ArbitratorAcceptance {
    pub arbitrator: PeerId,
    pub accepted: bool,
    pub acceptance_signature: Signature,
    pub accepted_at: Instant,
}

/** Arbitration case. */
#[derive(Debug, Clone)]
pub struct ArbitrationCase {
    pub case_id: String,
    pub trade_id: u64,
    pub buyer: PeerId,
    pub seller: PeerId,
    pub submitted_at: Instant,
    pub deadline: Instant,
    pub arbitrators: Vec<PeerId>,
    pub acceptances: Vec<ArbitratorAcceptance>,
    pub status: ArbitrationStatus,
    pub winner: Option<PeerId>,
    pub decision: Option<Vec<u8>>,
    pub finalized_at: Option<Instant>,
}

/** Arbitration status. */
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArbitrationStatus {
    Open,
    InProgress,
    Resolved,
    Timeout,
    Closed,
}

/** Create a new arbitration case. */
pub fn create_arbitration_case(
    trade_id: u64,
    buyer: PeerId,
    seller: PeerId,
    deadline: Duration,
) -> ArbitrationCase {
    let case_id = format!("arb-{}", trade_id);
    let now = Instant::now();
    ArbitrationCase {
        case_id,
        trade_id,
        buyer,
        seller,
        submitted_at: now,
        deadline: now + deadline,
        arbitrators: vec![], // Would be selected via VRF
        acceptances: vec![],
        status: ArbitrationStatus::Open,
        winner: None,
        decision: None,
        finalized_at: None,
    }
}

/** Select 10 arbitrators using verifiable randomness. */
pub fn select_arbitrators(_candidates: &[PeerId], _seed: &[u8]) -> ArbitratorSelection {
    // In a full implementation, use VRF (Verifiable Random Function)
    // to randomly select 10 arbitrators from candidates
    let arbitrators: Vec<PeerId> = _candidates
        .iter()
        .take(10)
        .cloned()
        .collect();
    ArbitratorSelection {
        arbitrators,
        selection_proof: vec![0u8; 32],
        round_number: 1,
    }
}

/** Record an arbitrator's acceptance. */
pub fn record_acceptance(
    _case: &mut ArbitrationCase,
    _arbitrator: PeerId,
    _signature: Signature,
) -> Result<(), String> {
    // Check if arbitrator is in the case's arbitrator list
    if !_case.arbitrators.iter().any(|a| *a == _arbitrator) {
        return Err("Arbitrator not in selected list".to_string());
    }

    // Check if already accepted
    if _case.acceptances.iter().any(|a| a.arbitrator == _arbitrator) {
        return Err("Arbitrator already accepted".to_string());
    }

    _case.acceptances.push(ArbitratorAcceptance {
        arbitrator: _arbitrator,
        accepted: true,
        acceptance_signature: _signature,
        accepted_at: Instant::now(),
    });

    // Check if this is the first acceptance (model: first valid acceptance handles disputes)
    if _case.acceptances.len() == 1 {
        _case.status = ArbitrationStatus::InProgress;
        _case.winner = Some(_arbitrator);
    }

    Ok(())
}

/** Check if arbitration has reached a decision. */
pub fn check_decision(_case: &ArbitrationCase) -> bool {
    // Model: first eligible arbitrator to accept receives the case
    _case.acceptances
        .iter()
        .any(|a| a.accepted && a.arbitrator == _case.winner.unwrap())
}

/** Finalize arbitration decision. */
pub fn finalize_arbitration(_case: &mut ArbitrationCase, _decision: Vec<u8>) {
    _case.status = ArbitrationStatus::Resolved;
    _case.decision = Some(_decision);
    _case.finalized_at = Some(Instant::now());
}

/** Check if arbitration has timed out. */
pub fn check_timeout(_case: &ArbitrationCase, _now: Instant) -> bool {
    _now > _case.deadline
}