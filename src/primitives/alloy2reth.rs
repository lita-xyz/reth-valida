// Modified from the implementation of sp1-reth, in turn modified from Zeth.
//
// Reference: https://github.com/risc0/zeth
//
// Copyright 2023 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
//! Common conversions between Alloy and Reth types.

use alloy_rpc_types::Header as AlloyHeader;
use alloy_consensus::{TxLegacy, TxEip2930, TxEip1559, TxEip4844, TxEip7702};
use alloy_consensus::Transaction as AlloyTransactionTrait;
use alloy_primitives::U256;
use alloy_primitives::Signature as RethSignature;
use alloy_rpc_types::Signature as AlloySignature;
use alloy_rpc_types::Transaction as AlloyTransaction;
use alloy_rpc_types::AccessList;
use reth_primitives::Header as RethHeader;
use reth_primitives::TransactionSignedNoHash as RethTransaction;
use reth_primitives::{Transaction, TransactionSignedNoHash};

/// A trait to convert from Alloy types to Reth types.
pub trait IntoReth<T> {
    fn into_reth(self) -> T;
}

impl IntoReth<RethTransaction> for AlloyTransaction {
    fn into_reth(self) -> RethTransaction {
        let tx_type: u64 = self.transaction_type.unwrap_or(0).into();
        let inner_tx = match tx_type {
           0  => Transaction::Legacy(TxLegacy {
                chain_id: self.chain_id(),
                nonce: self.nonce(),
                gas_price: self.gas_price().unwrap(),
                gas_limit: self.gas_limit().try_into().unwrap(),
                to: self.kind(),
                value: self.value(),
                input: self.input().clone(),
            }),
            1 => Transaction::Eip2930(TxEip2930 {
                chain_id: self.chain_id().unwrap(),
                nonce: self.nonce(),
                gas_price: self.gas_price().unwrap(),
                gas_limit: self.gas_limit().try_into().unwrap(),
                to: self.kind(),
                value: self.value(),
                input: self.input().clone(),
                access_list: AccessList(
                    self.access_list()
                        .unwrap()
                        .iter()
                        .map(|item| item.clone())
                        .collect(),
                ),
            }),
            2 => Transaction::Eip1559(TxEip1559 {
                chain_id: self.chain_id().unwrap(),
                nonce: self.nonce(),
                max_fee_per_gas: self.max_fee_per_gas(),
                max_priority_fee_per_gas: self.max_priority_fee_per_gas().unwrap(),
                gas_limit: self.gas_limit().try_into().unwrap(),
                to: self.kind(),
                value: self.value(),
                input: self.input().clone(),
                access_list: AccessList(
                    self.access_list()
                        .unwrap()
                        .iter()
                        .map(|item| item.clone())
                        .collect(),
                ),
            }),
            3 => Transaction::Eip4844(TxEip4844 {
                chain_id: self.chain_id().unwrap(),
                nonce: self.nonce(),
                max_fee_per_gas: self.max_fee_per_gas(),
                max_priority_fee_per_gas: self.max_priority_fee_per_gas().unwrap(),
                gas_limit: self.gas_limit().try_into().unwrap(),
                to: self.to.unwrap(),
                value: self.value(),
                input: self.input().clone(),
                access_list: AccessList(
                    self.access_list()
                        .unwrap()
                        .iter()
                        .map(|item| item.clone())
                        .collect(),
                ),
                blob_versioned_hashes: self.blob_versioned_hashes().unwrap().to_vec(),
                max_fee_per_blob_gas: self.max_fee_per_blob_gas().unwrap(),
            }),
            4 => Transaction::Eip7702(TxEip7702 {
                chain_id: self.chain_id().unwrap(),
                nonce: self.nonce(),
                max_fee_per_gas: self.max_fee_per_gas(),
                max_priority_fee_per_gas: self.max_priority_fee_per_gas().unwrap(),
                gas_limit: self.gas_limit().try_into().unwrap(),
                to: self.to.unwrap(),
                value: self.value(),
                input: self.input().clone(),
                access_list: AccessList(
                    self.access_list()
                        .unwrap()
                        .iter()
                        .map(|item| item.clone())
                        .collect(),
                ),
                authorization_list: self.authorization_list().unwrap().to_vec(),
            }),
            _ => panic!("invalid tx type"),
        };
        TransactionSignedNoHash {
            signature: self.signature.unwrap().into_reth(),
            transaction: inner_tx,
        }
    }
}

impl IntoReth<RethSignature> for AlloySignature {
    fn into_reth(self) -> RethSignature {
        // TODO: should be chain_id * 2 + 35.
        let recovery_id = if self.v > U256::from(1) {
            self.v - U256::from(37)
        } else {
            self.v
        };

        RethSignature::from_rs_and_parity(self.r, self.s, recovery_id == U256::from(1)).unwrap()
    }
}

impl IntoReth<RethHeader> for AlloyHeader {
    fn into_reth(self) -> RethHeader {
        RethHeader {
            parent_hash: self.parent_hash.0.into(),
            ommers_hash: self.uncles_hash.0.into(),
            beneficiary: self.miner.0.into(),
            state_root: self.state_root.0.into(),
            transactions_root: self.transactions_root.0.into(),
            receipts_root: self.receipts_root.0.into(),
            withdrawals_root: self.withdrawals_root,
            logs_bloom: self.logs_bloom.0.into(),
            difficulty: self.difficulty,
            number: self.number,
            gas_limit: self.gas_limit.try_into().unwrap(),
            gas_used: self.gas_used.try_into().unwrap(),
            timestamp: self.timestamp,
            extra_data: self.extra_data.0.clone().into(),
            mix_hash: self.mix_hash.unwrap(),
            nonce: self.nonce.unwrap(),
            base_fee_per_gas: Some(self.base_fee_per_gas.unwrap().try_into().unwrap()),
            blob_gas_used: self.blob_gas_used.map(|x| x.try_into().unwrap()),
            excess_blob_gas: self.excess_blob_gas.map(|x| x.try_into().unwrap()),
            parent_beacon_block_root: self.parent_beacon_block_root,
            requests_hash: self.requests_hash,
        }
    }
}
