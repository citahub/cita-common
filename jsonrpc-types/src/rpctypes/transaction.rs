// CITA
// Copyright 2016-2017 Cryptape Technologies LLC.

// This program is free software: you can redistribute it
// and/or modify it under the terms of the GNU General Public
// License as published by the Free Software Foundation,
// either version 3 of the License, or (at your option) any
// later version.

// This program is distributed in the hope that it will be
// useful, but WITHOUT ANY WARRANTY; without even the implied
// warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR
// PURPOSE. See the GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use cita_types::{H256, U256};
use libproto::{
    FullTransaction as ProtoFullTransaction, SignedTransaction as ProtoSignedTransaction,
    Transaction as ProtoTransaction,
};
use rpctypes::{Data, Integer, Quantity};
use std::convert::TryInto;

// TODO: No need Deserialize. Just because test in trans.rs
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct FullTransaction {
    pub hash: H256,
    pub content: Data,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct RpcTransaction {
    pub hash: H256,
    pub content: Data,
    #[serde(rename = "blockNumber")]
    pub block_number: U256,
    #[serde(rename = "blockHash")]
    pub block_hash: H256,
    pub index: U256,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BlockTransaction {
    Full(FullTransaction),
    Hash(H256),
}

impl From<ProtoFullTransaction> for RpcTransaction {
    fn from(mut ptransaction: ProtoFullTransaction) -> Self {
        let stx = ptransaction.take_transaction();
        let mut bhash: H256 = H256::default();
        bhash.0.clone_from_slice(ptransaction.block_hash.as_slice());

        let unverified_tx = stx.get_transaction_with_sig();
        let tx = unverified_tx.get_transaction();
        trace!(
            "GET ProtoTransaction: nonce {:?}, block_limit {:?}, data {:?}, quota {:?}, to {:?}",
            tx.get_nonce(),
            tx.get_valid_until_block(),
            tx.get_data(),
            tx.get_quota(),
            tx.get_to()
        );

        RpcTransaction {
            hash: H256::from_slice(stx.get_tx_hash()),
            content: Data::new(unverified_tx.try_into().unwrap()),
            block_number: U256::from(ptransaction.block_number),
            block_hash: bhash,
            index: U256::from(ptransaction.index),
        }
    }
}

impl From<ProtoSignedTransaction> for FullTransaction {
    fn from(stx: ProtoSignedTransaction) -> Self {
        FullTransaction {
            hash: H256::from_slice(stx.get_tx_hash()),
            content: Data::new(stx.get_transaction_with_sig().try_into().unwrap()),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq)]
pub struct Transaction {
    pub to: String,
    pub nonce: String,
    pub quota: Integer,
    pub valid_until_block: Integer,
    pub data: Data,
    pub value: Quantity,
    pub chain_id: Integer,
    pub version: Integer,
}

impl From<ProtoTransaction> for Transaction {
    fn from(tx: ProtoTransaction) -> Self {
        Transaction {
            to: tx.to,
            nonce: tx.nonce,
            quota: tx.quota.into(),
            valid_until_block: tx.valid_until_block.into(),
            data: tx.data.into(),
            value: tx.value.into(),
            chain_id: tx.chain_id.into(),
            version: tx.version.into(),
        }
    }
}
