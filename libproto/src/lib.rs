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

extern crate cita_crypto as crypto;
extern crate cita_types as types;
#[macro_use]
extern crate cita_logger as logger;
extern crate hashable;
extern crate protobuf;
extern crate rlp;
extern crate rustc_serialize;
#[macro_use]
extern crate serde_derive;
extern crate cita_merklehash;
extern crate snappy;

pub mod protos;
pub use crate::protos::*;
mod autoimpl;
pub mod router;

use crate::crypto::{CreateKey, KeyPair, PrivKey, PubKey, Sign, Signature, SIGNATURE_BYTES_LEN};
use crate::types::{Address, H256};
use cita_merklehash::{merge, Tree, HASH_NULL};
use hashable::Hashable;
use protobuf::RepeatedField;
use rlp::{Decodable, DecoderError, Encodable, RlpStream, UntrustedRlp};
use std::convert::From;
use std::ops::Deref;

pub use crate::autoimpl::{
    Message, MsgClass, OperateType, Origin, RawBytes, TryFromConvertError, TryIntoConvertError,
    ZERO_ORIGIN,
};
pub use crate::autoimpl::{TryFrom, TryInto};
use types::traits::LowerHex;

//TODO respone contain error
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct TxResponse {
    pub hash: H256,
    pub status: String,
}

impl TxResponse {
    pub fn new(hash: H256, status: String) -> Self {
        TxResponse { hash, status }
    }
}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct State(pub Vec<Vec<u8>>);

impl From<RichStatus> for Status {
    fn from(rich_status: RichStatus) -> Self {
        let mut status = Status::new();
        status.hash = rich_status.get_hash().to_vec();
        status.height = rich_status.get_height();
        status
    }
}

impl Transaction {
    /// Signs the transaction by PrivKey.
    pub fn sign(&self, sk: PrivKey) -> SignedTransaction {
        let keypair = KeyPair::from_privkey(sk).unwrap();
        let pubkey = keypair.pubkey();
        let unverified_tx = self.build_unverified(sk);

        // Build SignedTransaction
        let mut signed_tx = SignedTransaction::new();
        signed_tx.set_signer(pubkey.0.to_vec());
        let bytes: Vec<u8> = (&unverified_tx).try_into().unwrap();
        signed_tx.set_tx_hash(bytes.crypt_hash().0.to_vec());
        signed_tx.set_transaction_with_sig(unverified_tx);
        signed_tx
    }

    /// Build UnverifiedTransaction
    pub fn build_unverified(&self, sk: PrivKey) -> UnverifiedTransaction {
        let mut unverified_tx = UnverifiedTransaction::new();
        let bytes: Vec<u8> = self.try_into().unwrap();
        let hash = bytes.crypt_hash();
        unverified_tx.set_transaction(self.clone());
        let signature = Signature::sign(&sk, &hash).unwrap();
        unverified_tx.set_signature(signature.0.to_vec());
        unverified_tx.set_crypto(Crypto::DEFAULT);
        unverified_tx
    }
}

impl UnverifiedTransaction {
    /// Try to recover the public key.
    pub fn recover_public(&self) -> Result<(PubKey, H256), (H256, String)> {
        let bytes: Vec<u8> = self.get_transaction().try_into().unwrap();
        let hash = bytes.crypt_hash();
        let tx_hash = self.crypt_hash();
        if self.get_signature().len() != SIGNATURE_BYTES_LEN {
            trace!("Invalid signature length {}", hash);
            Err((tx_hash, String::from("Invalid signature length")))
        } else {
            match self.get_crypto() {
                Crypto::DEFAULT => {
                    let signature = Signature::from(self.get_signature());
                    match signature.recover(&hash) {
                        Ok(pubkey) => Ok((pubkey, tx_hash)),
                        _ => {
                            trace!("Recover error {}", tx_hash);
                            Err((tx_hash, String::from("Recover error")))
                        }
                    }
                }
                _ => {
                    trace!("Unexpected crypto {}", tx_hash);
                    Err((tx_hash, String::from("Unexpected crypto")))
                }
            }
        }
    }

    pub fn crypt_hash(&self) -> H256 {
        let bytes: Vec<u8> = self.try_into().unwrap();
        bytes.crypt_hash()
    }

    pub fn tx_verify_req_msg(&self) -> VerifyTxReq {
        let version = self.get_transaction().get_version();
        let bytes: Vec<u8> = self.get_transaction().try_into().unwrap();
        let hash = bytes.crypt_hash();
        let mut verify_tx_req = VerifyTxReq::new();
        verify_tx_req.set_valid_until_block(self.get_transaction().get_valid_until_block());
        // tx hash
        verify_tx_req.set_hash(hash.0.to_vec());
        verify_tx_req.set_crypto(self.get_crypto());
        verify_tx_req.set_signature(self.get_signature().to_vec());
        verify_tx_req.set_nonce(self.get_transaction().get_nonce().to_string());
        verify_tx_req.set_value(self.get_transaction().get_value().to_vec());
        if version == 0 {
            verify_tx_req.set_chain_id(self.get_transaction().get_chain_id());
        } else if version < 3 {
            verify_tx_req.set_chain_id_v1(self.get_transaction().get_chain_id_v1().to_vec());
        } else {
            error!("unexpected version {}!", version);
        }

        verify_tx_req.set_quota(self.get_transaction().get_quota());

        // unverified tx hash
        let tx_hash = self.crypt_hash();
        verify_tx_req.set_tx_hash(tx_hash.0.to_vec());
        verify_tx_req
    }
}

impl Deref for SignedTransaction {
    type Target = UnverifiedTransaction;

    fn deref(&self) -> &Self::Target {
        &self.get_transaction_with_sig()
    }
}

impl SignedTransaction {
    /// Try to verify transaction and recover sender.
    pub fn verify_transaction(transaction: UnverifiedTransaction) -> Result<Self, H256> {
        let (public, tx_hash) = transaction.recover_public().map_err(|(hash, _)| hash)?;
        let mut signed_tx = SignedTransaction::new();
        signed_tx.set_signer(public.0.to_vec());
        signed_tx.set_tx_hash(tx_hash.0.to_vec());
        signed_tx.set_transaction_with_sig(transaction);
        Ok(signed_tx)
    }

    pub fn crypt_hash(&self) -> H256 {
        H256::from_slice(self.tx_hash.as_slice())
    }

    pub fn from(&self) -> Address {
        let signer_pubkey = self.get_signer();

        types::H160::from(signer_pubkey.crypt_hash())
    }
}

impl Eq for Proof {}

impl Decodable for Proof {
    fn decode(data: &UntrustedRlp) -> Result<Self, DecoderError> {
        data.decoder()
            .decode_value(|bytes| Ok(Proof::try_from(bytes).unwrap()))
    }
}

impl Encodable for Proof {
    fn rlp_append(&self, s: &mut RlpStream) {
        let b: Vec<u8> = self.try_into().unwrap();
        s.encoder().encode_value(&b);
    }
}

impl Block {
    pub fn crypt_hash(&self) -> H256 {
        self.get_header().crypt_hash()
    }

    pub fn crypt_hash_hex(&self) -> String {
        self.get_header().crypt_hash_hex()
    }

    pub fn check_hash(&self) -> bool {
        self.get_body().transactions_root().0 == *self.get_header().get_transactions_root()
    }

    pub fn compact(mut self) -> CompactBlock {
        let mut ret = CompactBlock::new();
        ret.set_version(self.get_version());
        ret.set_header(self.take_header());
        ret.set_body(self.take_body().compact());
        ret
    }
}

impl CompactBlock {
    pub fn crypt_hash(&self) -> H256 {
        self.get_header().crypt_hash()
    }

    pub fn crypt_hash_hex(&self) -> String {
        self.get_header().crypt_hash_hex()
    }

    pub fn check_hash(&self) -> bool {
        self.get_body().transactions_root().0 == *self.get_header().get_transactions_root()
    }

    pub fn check_txs(&self, stxs: &[SignedTransaction]) -> Result<(), Option<usize>> {
        let hashes = self.get_body().get_tx_hashes();
        if stxs.len() == hashes.len() {
            stxs.iter()
                .zip(hashes.iter())
                .enumerate()
                .try_for_each(|(idx, (stx, hash))| {
                    if &stx.tx_hash == hash {
                        Ok(())
                    } else {
                        Err(Some(idx))
                    }
                })
        } else {
            Err(None)
        }
    }

    pub fn complete(mut self, stxs: Vec<SignedTransaction>) -> Block {
        let body = BlockBody::from_transactions(stxs);
        let mut ret = Block::new();
        ret.set_version(self.get_version());
        ret.set_header(self.take_header());
        ret.set_body(body);
        ret
    }
}

impl Proposal {
    pub fn compact(mut self) -> CompactProposal {
        let mut ret = CompactProposal::new();
        ret.set_block(self.take_block().compact());
        ret.set_islock(self.get_islock());
        ret.set_lock_round(self.get_lock_round());
        ret.set_lock_votes(self.take_lock_votes());
        ret.set_round(self.get_round());
        ret.set_height(self.get_height());
        ret
    }
}

impl CompactProposal {
    pub fn complete(mut self, stxs: Vec<SignedTransaction>) -> Proposal {
        let mut ret = Proposal::new();
        ret.set_block(self.take_block().complete(stxs));
        ret.set_islock(self.get_islock());
        ret.set_lock_round(self.get_lock_round());
        ret.set_lock_votes(self.take_lock_votes());
        ret.set_round(self.get_round());
        ret.set_height(self.get_height());
        ret
    }
}

impl SignedProposal {
    pub fn compact(mut self) -> CompactSignedProposal {
        let mut ret = CompactSignedProposal::new();
        ret.set_proposal(self.take_proposal().compact());
        ret.set_signature(self.take_signature());
        ret
    }
}

impl CompactSignedProposal {
    pub fn complete(mut self, stxs: Vec<SignedTransaction>) -> SignedProposal {
        let mut ret = SignedProposal::new();
        ret.set_proposal(self.take_proposal().complete(stxs));
        ret.set_signature(self.take_signature());
        ret
    }
    pub fn create_verify_block_req(mut self) -> VerifyBlockReq {
        let mut proposal = self.take_proposal();
        let compact_block = proposal.take_block();
        let mut verify_req = VerifyBlockReq::new();
        verify_req.set_height(proposal.get_height());
        verify_req.set_round(proposal.get_round());
        verify_req.set_block(compact_block);
        verify_req
    }
}

impl BlockHeader {
    pub fn crypt_hash(&self) -> H256 {
        let bytes: Vec<u8> = self.try_into().unwrap();
        bytes.crypt_hash()
    }

    pub fn crypt_hash_hex(&self) -> String {
        let bytes: Vec<u8> = self.try_into().unwrap();
        bytes.crypt_hash().lower_hex()
    }
}

impl BlockBody {
    pub fn transaction_hashes(&self) -> Vec<H256> {
        self.get_transactions()
            .iter()
            .map(|ts| H256::from_slice(ts.get_tx_hash()))
            .collect()
    }

    pub fn transactions_root(&self) -> H256 {
        *Tree::from_hashes(self.transaction_hashes(), merge)
            .get_root_hash()
            .unwrap_or(&HASH_NULL)
    }

    pub fn from_transactions(stxs: Vec<SignedTransaction>) -> BlockBody {
        let mut ret = BlockBody::new();
        ret.set_transactions(RepeatedField::from_vec(stxs));
        ret
    }

    pub fn compact(mut self) -> CompactBlockBody {
        let mut ret = CompactBlockBody::new();
        let tx_hashes = self
            .take_transactions()
            .into_iter()
            .map(|stx| stx.tx_hash)
            .collect::<Vec<_>>();
        ret.set_tx_hashes(RepeatedField::from_vec(tx_hashes));
        ret
    }
}

impl CompactBlockBody {
    pub fn transaction_hashes(&self) -> Vec<H256> {
        self.get_tx_hashes()
            .iter()
            .map(|h| H256::from_slice(h))
            .collect()
    }

    pub fn transactions_root(&self) -> H256 {
        *Tree::from_hashes(self.transaction_hashes(), merge)
            .get_root_hash()
            .unwrap_or(&HASH_NULL)
    }
}

impl VerifyBlockReq {
    pub fn check_txs(&self, stxs: &[SignedTransaction]) -> Result<(), Option<usize>> {
        self.get_block().check_txs(stxs)
    }

    pub fn reply(mut self, result: Result<Vec<SignedTransaction>, ()>) -> VerifyBlockResp {
        let mut ret = VerifyBlockResp::new();
        ret.set_height(self.get_height());
        ret.set_round(self.get_round());
        match result {
            Ok(stxs) => {
                ret.set_block(self.take_block().complete(stxs));
                ret.set_pass(true);
            }
            Err(()) => {
                ret.set_pass(false);
            }
        }
        ret
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn create_tx() {
        use super::{CreateKey, KeyPair, Transaction};
        let keypair = KeyPair::gen_keypair();
        let pv = keypair.privkey();

        let data = vec![1];
        let mut tx = Transaction::new();
        tx.set_data(data);
        tx.set_nonce("0".to_string());
        tx.set_to("123".to_string());
        tx.set_valid_until_block(99999);
        tx.set_quota(999999999);
        tx.set_value(vec![1]);
        tx.set_chain_id(0);
        tx.set_version(0);

        let signed_tx = tx.sign(*pv);
        assert_eq!(
            signed_tx.crypt_hash(),
            signed_tx.get_transaction_with_sig().crypt_hash()
        );
    }

    #[test]
    fn create_tx_v1() {
        use super::{CreateKey, KeyPair, Transaction};
        let keypair = KeyPair::gen_keypair();
        let pv = keypair.privkey();

        let data = vec![1];
        let mut tx = Transaction::new();
        tx.set_data(data);
        tx.set_nonce("0".to_string());
        tx.set_to_v1(vec![1, 2, 3]);
        tx.set_valid_until_block(99999);
        tx.set_quota(999999999);
        tx.set_value(vec![1]);
        tx.set_chain_id_v1(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0]);
        tx.set_version(1);

        let signed_tx = tx.sign(*pv);
        assert_eq!(
            signed_tx.crypt_hash(),
            signed_tx.get_transaction_with_sig().crypt_hash()
        );
    }
}
