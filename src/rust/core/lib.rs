/**
 * Decentralized Exchange - Core Library
 * 
 * Modular architecture with clearly separated responsibilities.
 * Each module is independent and can be developed/tested separately.
 */
pub mod network;
pub mod cryptography;
pub mod identity;
pub mod orders;
pub mod trading;
pub mod bitcoin;
pub mod monero;
pub mod escrow;
pub mod arbitration;
pub mod storage;