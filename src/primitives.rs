pub mod alloy2reth;


use reth_primitives::{Address, Bytes, Header, TransactionSignedNoHash, Withdrawal, B256};
use revm::primitives::HashMap;
use serde::{Deserialize, Serialize};

/// Necessary information to prove the execution of Ethereum blocks inside Valida.
#[derive(Clone, Serialize, Deserialize)]
pub struct ValidaRethInput {
    /// The 160-bit address to which all fees collected from the successful mining of this block
    /// be transferred.
    pub beneficiary: Address,

    /// A scalar value equal to the current limit of gas expenditure per block.
    pub gas_limit: u64,

    /// A scalar value equal to the reasonable output of Unix's time() at this block's inception.
    pub timestamp: u64,

    /// An arbitrary byte array containing data relevant to this block. This must be 32 bytes or
    /// fewer.
    pub extra_data: Bytes,

}
