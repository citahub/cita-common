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

use cita_types::{traits::LowerHex, H256, U256};
use jsonrpc_types::{
    rpc_types::{Data, FullTransaction, RpcTransaction},
    Error,
};
use libproto::{
    FullTransaction as ProtoFullTransaction, SignedTransaction as ProtoSignedTransaction, TryInto,
    UnverifiedTransaction as ProtoUnverifiedTransaction,
};

use crate::{error::ErrorExt, from_into::TryFromProto};

impl TryFromProto<ProtoUnverifiedTransaction> for Data {
    type Error = Error;

    fn try_from_proto(utx: ProtoUnverifiedTransaction) -> Result<Self, Self::Error> {
        let content: Vec<u8> = utx
            .try_into()
            .map_err(Error::transaction_data_encode_error)?;

        Ok(Data::new(content))
    }
}

impl TryFromProto<ProtoFullTransaction> for RpcTransaction {
    type Error = Error;

    fn try_from_proto(mut ptransaction: ProtoFullTransaction) -> Result<Self, Self::Error> {
        let mut stx = ptransaction.take_transaction();
        let unverified_tx = stx.take_transaction_with_sig();

        {
            let tx = unverified_tx.get_transaction();
            trace!(
                "GET ProtoTransaction: nonce {:?}, block_limit {:?}, data {:?}, quota {:?}, to {:?}",
                tx.get_nonce(),
                tx.get_valid_until_block(),
                tx.get_data(),
                tx.get_quota(),
                match tx.get_version() {
                    0 => tx.get_to().to_owned(),
                    1 => tx.get_to_v1().lower_hex(),
                    v => {
                        error!("unexpected version {}!", v);
                        "unknown".to_owned()
                    }
                }
            );
        }

        Ok(RpcTransaction {
            hash: H256::from_slice(stx.get_tx_hash()),
            content: Data::try_from_proto(unverified_tx)?,
            from: stx.from(),
            block_number: U256::from(ptransaction.block_number),
            block_hash: H256::from_slice(ptransaction.block_hash.as_slice()),
            index: U256::from(ptransaction.index),
        })
    }
}

impl TryFromProto<ProtoSignedTransaction> for FullTransaction {
    type Error = Error;

    fn try_from_proto(mut stx: ProtoSignedTransaction) -> Result<Self, Self::Error> {
        Ok(FullTransaction {
            hash: H256::from_slice(stx.get_tx_hash()),
            content: Data::try_from_proto(stx.take_transaction_with_sig())?,
            from: stx.from(),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use cita_crypto::{CreateKey, KeyPair};

    fn new_dummy_signed_ptx() -> (KeyPair, ProtoSignedTransaction) {
        let keypair = KeyPair::gen_keypair();

        // TODO: quickcheck
        let mut ptx = libproto::Transaction::new();
        ptx.set_data(vec![1]);
        ptx.set_nonce(String::from("0"));
        ptx.set_to_v1(vec![1, 2, 3]);
        ptx.set_valid_until_block(66);
        ptx.set_quota(314159265);
        ptx.set_value(vec![1]);
        ptx.set_chain_id_v1(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0]);
        ptx.set_version(1);

        let sig_ptx = ptx.sign(*keypair.privkey());
        (keypair, sig_ptx)
    }

    #[test]
    fn test_try_from_proto_utx_for_data() {
        use libproto::TryInto;

        let (_keypair, sig_ptx) = new_dummy_signed_ptx();
        let p_utx = sig_ptx.get_transaction_with_sig();

        let data = Data::try_from_proto(p_utx.clone()).unwrap();
        assert_eq!(data, Data::new(p_utx.try_into().unwrap()));
    }

    #[test]
    fn test_try_from_proto_sig_tx_for_json_full_tx() {
        let (_keypair, sig_ptx) = new_dummy_signed_ptx();
        let json_tx = FullTransaction::try_from_proto(sig_ptx.clone()).unwrap();

        let hash = sig_ptx.get_tx_hash();
        let content = sig_ptx.get_transaction_with_sig().try_into().unwrap();
        assert_eq!(json_tx.hash, H256::from_slice(hash));
        assert_eq!(json_tx.content, Data::new(content));
    }

    #[test]
    fn test_from_proto_full_transaction() {
        let (keypair, sig_ptx) = new_dummy_signed_ptx();
        let dummy_hash = "ed76641c68a1c641aee09a94b3b471f4dc0316efe5ac19cf488e2674cf8d05b5";
        let block_hash = H256::from_str(dummy_hash).unwrap();

        let mut full_tx = ProtoFullTransaction::new();
        full_tx.set_transaction(sig_ptx.clone());
        full_tx.set_block_number(2077);
        full_tx.set_block_hash(block_hash.to_vec());
        full_tx.set_index(0);

        let rpc_tx = RpcTransaction::try_from_proto(full_tx).unwrap();
        let rpc_data = Data::new(sig_ptx.get_transaction_with_sig().try_into().unwrap());
        assert_eq!(rpc_tx.hash, H256::from_slice(sig_ptx.get_tx_hash()));
        assert_eq!(rpc_tx.content, rpc_data);
        assert_eq!(rpc_tx.from, keypair.address());
        assert_eq!(rpc_tx.block_number, U256::from(2077));
        assert_eq!(rpc_tx.block_hash, block_hash);
        assert_eq!(rpc_tx.index, U256::from(0));
    }
}
