// Copyright Rivtower Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use cita_types::{Address, H256, U256};
use jsonrpc_types::rpc_types::{
    Block, BlockBody, BlockHeader, BlockTransaction, FullTransaction, Proof, RpcBlock,
};
use jsonrpc_types::Error;

use crate::from_into::TryFromProto;

pub trait BlockExt {
    type Error;

    fn try_from_rpc_block(rpc_block: RpcBlock) -> Result<Block, Self::Error>;
}

impl TryFromProto<libproto::BlockHeader> for BlockHeader {
    type Error = Error;

    fn try_from_proto(proto_header: libproto::BlockHeader) -> Result<Self, Self::Error> {
        let proof: Option<Proof> = match proto_header.get_height() {
            0 | 1 => None,
            _ => Some(Proof::try_from_proto(proto_header.clone().take_proof())?),
        };
        trace!(
            "number = {}, proof = {:?}",
            U256::from(proto_header.get_height()),
            proof
        );

        Ok(BlockHeader {
            timestamp: proto_header.timestamp,
            prev_hash: H256::from(proto_header.get_prevhash()),
            number: U256::from(proto_header.get_height()),
            state_root: H256::from(proto_header.get_state_root()),
            transactions_root: H256::from(proto_header.get_transactions_root()),
            receipts_root: H256::from(proto_header.get_receipts_root()),
            quota_used: U256::from(proto_header.get_quota_used()),
            proof,
            proposer: Address::from(proto_header.get_proposer()),
        })
    }
}

impl BlockExt for Block {
    type Error = Error;

    fn try_from_rpc_block(rpc_block: RpcBlock) -> Result<Self, Self::Error> {
        use crate::error::ErrorExt;
        use libproto::TryFrom;

        let mut blk = libproto::Block::try_from(&rpc_block.block) // from chain
            .map_err(|err| Error::rpc_block_decode_error(Box::new(err)))?;

        let block_transactions = blk.take_body().take_transactions();
        let transactions = if rpc_block.include_txs {
            block_transactions
                .into_iter()
                .map(|x| FullTransaction::try_from_proto(x).map(BlockTransaction::Full))
                .collect::<Result<Vec<BlockTransaction>, Error>>()?
        } else {
            block_transactions
                .into_iter()
                .map(|x| BlockTransaction::Hash(H256::from_slice(x.get_tx_hash())))
                .collect()
        };
        let header = BlockHeader::try_from_proto(blk.take_header())?;

        Ok(Block {
            version: blk.version,
            header,
            body: BlockBody { transactions },
            hash: H256::from_slice(&rpc_block.hash),
        })
    }
}
