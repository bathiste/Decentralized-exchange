/**
 * Storage Module
 * Persistent storage for orders, states, and configuration.
 */
use crate::orders::Order;
use crate::trading::Trade;
use crate::escrow::EscrowState;
use crate::arbitration::ArbitrationCase;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::collections::BTreeMap;

#[derive(Debug)]
pub enum StorageError { FileNotFound(String), IoError(String), SerdeError(String), NotFound(String), CorruptData(String) }

pub struct FileBackend { data_dir: PathBuf, orders: BTreeMap<[u8;32], Order>, trades: BTreeMap<u64, Trade>, escrows: BTreeMap<String, EscrowState>, arbitrations: BTreeMap<String, ArbitrationCase> }

impl FileBackend {
    pub fn new(data_dir: &Path) -> Result<Self, StorageError> {
        let _ = std::fs::create_dir_all(data_dir).map_err(|e| StorageError::IoError(format!("{}", e)))?;
        Ok(FileBackend { data_dir: data_dir.to_path_buf(), orders: BTreeMap::new(), trades: BTreeMap::new(), escrows: BTreeMap::new(), arbitrations: BTreeMap::new() })
    }
}

impl FileBackend {
    pub fn save_order(&self, _order: &Order) -> Result<(), StorageError> {
        let path = self.data_dir.join("orders").join(format!("{}.json", hex::encode(_order.id)));
        let json = serde_json::to_string(_order).map_err(|e| StorageError::SerdeError(format!("{}", e)))?;
        std::fs::write(&path, &json).map_err(|e| StorageError::IoError(format!("{}", e)))?;
        Ok(())
    }
    pub fn load_order(&self, _order_id: &[u8; 32]) -> Result<Option<Order>, StorageError> {
        let path = self.data_dir.join("orders").join(format!("{}.json", hex::encode(_order_id)));
        if !path.exists() { return Ok(None); }
        let content = std::fs::read_to_string(&path).map_err(|e| StorageError::IoError(format!("{}", e)))?;
        let order: Order = serde_json::from_str(&content).map_err(|e| StorageError::SerdeError(format!("{}", e)))?;
        Ok(Some(order))
    }
}

pub struct MemoryBackend { orders: BTreeMap<[u8;32], Order>, trades: BTreeMap<u64, Trade>, escrows: BTreeMap<String, EscrowState>, arbitrations: BTreeMap<String, ArbitrationCase> }

impl MemoryBackend {
    pub fn new() -> Self {
        MemoryBackend { orders: BTreeMap::new(), trades: BTreeMap::new(), escrows: BTreeMap::new(), arbitrations: BTreeMap::new() }
    }
}

impl FileBackend {
    pub fn save_trade(&self, _trade: &Trade) -> Result<(), StorageError> { Ok(()) }
    pub fn load_trade(&self, _trade_id: u64) -> Result<Option<Trade>, StorageError> { Ok(None) }
    pub fn save_escrow(&self, _escrow: &EscrowState) -> Result<(), StorageError> { Ok(()) }
    pub fn load_escrow(&self, _escrow_id: &str) -> Result<Option<EscrowState>, StorageError> { Ok(None) }
    pub fn save_arbitration(&self, _case: &ArbitrationCase) -> Result<(), StorageError> { Ok(()) }
    pub fn load_arbitration(&self, _case_id: &str) -> Result<Option<ArbitrationCase>, StorageError> { Ok(None) }
}

impl MemoryBackend {
    pub fn save_order(&self, _order: &Order) -> Result<(), StorageError> { Ok(()) }
    pub fn load_order(&self, _order_id: &[u8; 32]) -> Result<Option<Order>, StorageError> { self.orders.get(_order_id).cloned().ok_or(StorageError::NotFound("".to_string())) }
    pub fn save_trade(&self, _trade: &Trade) -> Result<(), StorageError> { Ok(()) }
    pub fn load_trade(&self, _trade_id: u64) -> Result<Option<Trade>, StorageError> { self.trades.get(&_trade_id).cloned().ok_or(StorageError::NotFound("".to_string())) }
    pub fn save_escrow(&self, _escrow: &EscrowState) -> Result<(), StorageError> { Ok(()) }
    pub fn load_escrow(&self, _escrow_id: &str) -> Result<Option<EscrowState>, StorageError> { self.escrows.get(_escrow_id).cloned().ok_or(StorageError::NotFound("".to_string())) }
    pub fn save_arbitration(&self, _case: &ArbitrationCase) -> Result<(), StorageError> { Ok(()) }
    pub fn load_arbitration(&self, _case_id: &str) -> Result<Option<ArbitrationCase>, StorageError> { self.arbitrations.get(_case_id).cloned().ok_or(StorageError::NotFound("".to_string())) }
}