// CITA
// Copyright 2016-2018 Cryptape Technologies LLC.

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

use std::convert::TryInto;

use cita_types::{traits::LowerHex, H256, U256};
use jsonrpc_types::rpctypes::{Data, FullTransaction, RpcTransaction};

use crate::from_into::FromProto;

impl FromProto<libproto::FullTransaction> for RpcTransaction {
    fn from_proto(mut ptransaction: libproto::FullTransaction) -> Self {
        let stx = ptransaction.take_transaction();
        let mut bhash: H256 = H256::default();
        bhash.0.clone_from_slice(ptransaction.block_hash.as_slice());

        let unverified_tx = stx.get_transaction_with_sig();
        let tx = unverified_tx.get_transaction();
        let version = tx.get_version();
        trace!(
            "GET ProtoTransaction: nonce {:?}, block_limit {:?}, data {:?}, quota {:?}, to {:?}",
            tx.get_nonce(),
            tx.get_valid_until_block(),
            tx.get_data(),
            tx.get_quota(),
            if version == 0 {
                tx.get_to().to_owned()
            } else if version == 1 {
                tx.get_to_v1().lower_hex()
            } else {
                error!("unexpected version {}!", version);
                "unknown".to_owned()
            }
        );

        RpcTransaction {
            hash: H256::from_slice(stx.get_tx_hash()),
            content: Data::new(unverified_tx.try_into().unwrap()),
            from: stx.from(),
            block_number: U256::from(ptransaction.block_number),
            block_hash: bhash,
            index: U256::from(ptransaction.index),
        }
    }
}

impl FromProto<libproto::SignedTransaction> for FullTransaction {
    fn from_proto(stx: libproto::SignedTransaction) -> Self {
        FullTransaction {
            hash: H256::from_slice(stx.get_tx_hash()),
            content: Data::new(stx.get_transaction_with_sig().try_into().unwrap()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use cita_crypto::KeyPair;
    use cita_crypto_trait::CreateKey;
    use libproto::Transaction;

    #[test]
    fn test_from_proto_full_transaction() {
        let keypair = KeyPair::gen_keypair();
        let privkey = keypair.privkey();

        let data = vec![1];
        let mut tx = Transaction::new();
        tx.set_data(data);
        tx.set_nonce("0".to_string());
        tx.set_to_v1(vec![1, 2, 3]);
        tx.set_valid_until_block(66);
        tx.set_quota(314159265);
        tx.set_value(vec![1]);
        tx.set_chain_id_v1(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0]);
        tx.set_version(1);

        let signed_tx = tx.sign(*privkey);
        let block_hash =
            H256::from_str("ed76641c68a1c641aee09a94b3b471f4dc0316efe5ac19cf488e2674cf8d05b5")
                .unwrap();

        let mut full_tx = libproto::FullTransaction::new();
        full_tx.set_transaction(signed_tx.clone());
        full_tx.set_block_number(2077);
        full_tx.set_block_hash(block_hash.to_vec());
        full_tx.set_index(0);

        let rpc_tx = RpcTransaction::from_proto(full_tx);

        assert_eq!(rpc_tx.hash, H256::from_slice(signed_tx.get_tx_hash()));
        assert_eq!(
            rpc_tx.content,
            Data::new(signed_tx.get_transaction_with_sig().try_into().unwrap())
        );
        assert_eq!(rpc_tx.from, keypair.address());
        assert_eq!(rpc_tx.block_number, U256::from(2077));
        assert_eq!(rpc_tx.block_hash, block_hash);
        assert_eq!(rpc_tx.index, U256::from(0));
    }
}
