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

#![feature(try_from)]

extern crate cita_crypto as crypto;
#[macro_use]
extern crate log as rlog;
extern crate protobuf;
extern crate rlp;
extern crate rustc_serialize;
#[macro_use]
extern crate serde_derive;
extern crate util;

pub mod protos;

pub use protos::*;
pub use protos::auth;
pub use protos::blockchain;
pub use protos::communication;
pub use protos::consensus;
pub use protos::executor;
pub use protos::request;
pub use protos::response;
pub use protos::sync;

use crypto::{CreateKey, KeyPair, Message as SignMessage, PrivKey, PubKey, Sign, Signature, SIGNATURE_BYTES_LEN};
use protobuf::{parse_from_bytes, Message as MessageTrait, RepeatedField};
use rlp::*;
use rustc_serialize::hex::ToHex;
use std::ops::Deref;
use std::result::Result::Err;
use util::{merklehash, snappy, H256, Hashable};

use std::convert::{From, Into, TryFrom, TryInto};

//TODO respone contain error
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct TxResponse {
    pub hash: H256,
    pub status: String,
}

impl TxResponse {
    pub fn new(hash: H256, status: String) -> Self {
        TxResponse {
            hash: hash,
            status: status,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct State(pub Vec<Vec<u8>>);

pub mod submodules {
    pub const JSON_RPC: u32 = 1;
    pub const NET: u32 = 2;
    pub const CHAIN: u32 = 3;
    pub const CONSENSUS: u32 = 4;
    pub const CONSENSUS_CMD: u32 = 5;
    pub const AUTH: u32 = 6;
    pub const EXECUTOR: u32 = 7;
}

// TODO: 这里要不要修改下，使topics和MsgClass对应起来
pub mod topics {
    pub const DEFAULT: u16 = 0;
    pub const REQUEST: u16 = 1;
    pub const NEW_BLK: u16 = 2;
    pub const NEW_STATUS: u16 = 3;
    pub const SYNC_BLK: u16 = 4;
    pub const RESPONSE: u16 = 5;
    pub const CONSENSUS_MSG: u16 = 6;
    pub const NEW_PROPOSAL: u16 = 7;
    pub const VERIFY_TX_REQ: u16 = 8;
    pub const VERIFY_TX_RESP: u16 = 9;
    pub const VERIFY_BLK_REQ: u16 = 10;
    pub const VERIFY_BLK_RESP: u16 = 11;
    pub const BLOCK_TXHASHES: u16 = 12;
    pub const BLOCK_TXHASHES_REQ: u16 = 13;
    pub const NEW_PROOF_BLOCK: u16 = 14;
    pub const BLOCK_TXS: u16 = 15;
    pub const RICH_STATUS: u16 = 16;
    pub const EXECUTED_RESULT: u16 = 17;
}

#[derive(Debug)]
pub enum MsgClass {
    EMPTY,
    REQUEST(Request),
    RESPONSE(Response),
    HEADER(BlockHeader),
    BLOCK(Block),
    STATUS(Status),
    VERIFYTXREQ(VerifyTxReq),
    VERIFYTXRESP(VerifyTxResp),
    VERIFYBLKREQ(VerifyBlockReq),
    VERIFYBLKRESP(VerifyBlockResp),
    BLOCKTXHASHES(BlockTxHashes),
    BLOCKTXHASHESREQ(BlockTxHashesReq),
    BLOCKWITHPROOF(BlockWithProof),
    BLOCKTXS(BlockTxs),
    MSG(Vec<u8>),
    RICHSTATUS(RichStatus),
    SYNCREQUEST(SyncRequest),
    SYNCRESPONSE(SyncResponse),
    EXECUTED(ExecutedResult),
}

pub fn key_to_id(key: &str) -> u32 {
    if key.starts_with("jsonrpc") {
        submodules::JSON_RPC
    } else if key.starts_with("net") {
        submodules::NET
    } else if key.starts_with("chain") {
        submodules::CHAIN
    } else if key.starts_with("consensus_cmd") {
        submodules::CONSENSUS_CMD
    } else if key.starts_with("consensus") {
        submodules::CONSENSUS
    } else if key.starts_with("auth") {
        submodules::AUTH
    } else if key.starts_with("executor") {
        submodules::EXECUTOR
    } else {
        0
    }
}

pub fn cmd_id(submodule: u32, topic: u16) -> u32 {
    (submodule << 16) + topic as u32
}

const ZERO_ORIGIN: u32 = 99999;

impl From<Message_oneof_content> for MsgClass {
    fn from(content: Message_oneof_content) -> Self {
        match content {
            Message_oneof_content::RawBytes(data) => {
                let mut content = Vec::new();
                content.extend_from_slice(&snappy::cita_decompress(data));
                MsgClass::MSG(content)
            }
            Message_oneof_content::Request(data) => MsgClass::REQUEST(data),
            Message_oneof_content::Response(data) => MsgClass::RESPONSE(data),
            Message_oneof_content::SyncRequest(data) => MsgClass::SYNCREQUEST(data),
            Message_oneof_content::SyncResponse(data) => MsgClass::SYNCRESPONSE(data),
            Message_oneof_content::Status(data) => MsgClass::STATUS(data),
            Message_oneof_content::RichStatus(data) => MsgClass::RICHSTATUS(data),
            Message_oneof_content::Block(data) => MsgClass::BLOCK(data),
            Message_oneof_content::BlockWithProof(data) => MsgClass::BLOCKWITHPROOF(data),
            Message_oneof_content::BlockHeader(data) => MsgClass::HEADER(data),
            Message_oneof_content::BlockTxs(data) => MsgClass::BLOCKTXS(data),
            Message_oneof_content::BlockTxHashes(data) => MsgClass::BLOCKTXHASHES(data),
            Message_oneof_content::BlockTxHashesReq(data) => MsgClass::BLOCKTXHASHESREQ(data),
            Message_oneof_content::VerifyTxReq(data) => MsgClass::VERIFYTXREQ(data),
            Message_oneof_content::VerifyTxResp(data) => MsgClass::VERIFYTXRESP(data),
            Message_oneof_content::VerifyBlockReq(data) => MsgClass::VERIFYBLKREQ(data),
            Message_oneof_content::VerifyBlockResp(data) => MsgClass::VERIFYBLKRESP(data),
            Message_oneof_content::ExecutedResult(data) => MsgClass::EXECUTED(data),
        }
    }
}

impl From<Option<Message_oneof_content>> for MsgClass {
    fn from(content: Option<Message_oneof_content>) -> Self {
        match content {
            Some(inner) => inner.into(),
            None => MsgClass::EMPTY,
        }
    }
}

impl Message {
    pub fn clear_content(&mut self) {
        self.content = None;
    }

    pub fn has_content(&self) -> bool {
        self.content.is_some()
    }

    // Param is passed by value, moved
    pub fn set_content(&mut self, v: MsgClass) {
        match v {
            MsgClass::MSG(data) => self.set_RawBytes(snappy::cita_compress(data)),
            MsgClass::REQUEST(data) => self.set_Request(data),
            MsgClass::RESPONSE(data) => self.set_Response(data),
            MsgClass::SYNCREQUEST(data) => self.set_SyncRequest(data),
            MsgClass::SYNCRESPONSE(data) => self.set_SyncResponse(data),
            MsgClass::STATUS(data) => self.set_Status(data),
            MsgClass::RICHSTATUS(data) => self.set_RichStatus(data),
            MsgClass::BLOCK(data) => self.set_Block(data),
            MsgClass::BLOCKWITHPROOF(data) => self.set_BlockWithProof(data),
            MsgClass::HEADER(data) => self.set_BlockHeader(data),
            MsgClass::BLOCKTXS(data) => self.set_BlockTxs(data),
            MsgClass::BLOCKTXHASHES(data) => self.set_BlockTxHashes(data),
            MsgClass::BLOCKTXHASHESREQ(data) => self.set_BlockTxHashesReq(data),
            MsgClass::VERIFYTXREQ(data) => self.set_VerifyTxReq(data),
            MsgClass::VERIFYTXRESP(data) => self.set_VerifyTxResp(data),
            MsgClass::VERIFYBLKREQ(data) => self.set_VerifyBlockReq(data),
            MsgClass::VERIFYBLKRESP(data) => self.set_VerifyBlockResp(data),
            MsgClass::EXECUTED(data) => self.set_ExecutedResult(data),
            MsgClass::EMPTY => self.clear_content(),
        };
    }

    pub fn get_content(&self) -> MsgClass {
        self.content.clone().into()
    }

    pub fn take_content(&mut self) -> MsgClass {
        self.content.take().into()
    }

    pub fn init(sub: u32, top: u16, operate: OperateType, origin: u32, mc: MsgClass) -> Self {
        let mut msg = Message::new();
        msg.set_cmd_id(cmd_id(sub, top));
        msg.set_origin(origin);
        msg.set_operate(operate);
        msg.set_content(mc);
        msg
    }

    pub fn init_default(sub: u32, top: u16, mc: MsgClass) -> Self {
        Message::init(sub, top, OperateType::BROADCAST, ZERO_ORIGIN, mc)
    }
}

#[derive(Debug)]
pub struct TryFromConvertError(());

#[derive(Debug)]
pub struct TryIntoConvertError(());

macro_rules! impl_convert_for_struct {
    ($( $struct:ident, )+) => {
        $(
            impl_convert_for_struct!($struct);
        )+
    };
    ($struct:ident) => {

        impl<'a> TryFrom<&'a [u8]> for $struct {
            type Error = TryFromConvertError;
            fn try_from(b: &[u8]) -> Result<Self, TryFromConvertError> {
                parse_from_bytes::<$struct>(b).map_err(|_| { TryFromConvertError(()) })
            }
        }

        impl<'a> TryFrom<&'a Vec<u8>> for $struct {
            type Error = TryFromConvertError;
            fn try_from(v: &Vec<u8>) -> Result<Self, TryFromConvertError> {
                Self::try_from(v.as_slice())
            }
        }

       /* Comment for preventing confusion: references is better than entities.
       impl TryFrom<Vec<u8>> for $struct {
            type Error = TryFromConvertError;
            fn try_from(v: Vec<u8>) -> Result<Self, TryFromConvertError> {
                Self::try_from(v.as_slice())
            }
        }
        */

       impl<'a> TryInto<Vec<u8>> for &'a $struct {
           type Error = TryIntoConvertError;
           fn try_into(self) -> Result<Vec<u8>, TryIntoConvertError> {
               self.write_to_bytes().map_err(|_| { TryIntoConvertError(()) })
           }
       }

       impl TryInto<Vec<u8>> for $struct {
           type Error = TryIntoConvertError;
           fn try_into(self) -> Result<Vec<u8>, TryIntoConvertError> {
               self.write_to_bytes().map_err(|_| { TryIntoConvertError(()) })
           }
       }
    };
}

// grep "^pub struct .* {$" protos/*.rs | sort | awk '{ print "            "$3"," }' | uniq
macro_rules! loop_macro_for_structs {
    ($macro:ident) => {
        $macro!(
            BlockTxHashes,
            BlockTxHashesReq,
            VerifyBlockReq,
            VerifyBlockResp,
            VerifyTxReq,
            VerifyTxResp,
            AccountGasLimit,
            Block,
            BlockBody,
            BlockHeader,
            BlockTxs,
            BlockWithProof,
            Proof,
            RichStatus,
            SignedTransaction,
            Status,
            Transaction,
            UnverifiedTransaction,
            Message,
            Proposal,
            SignedProposal,
            Vote,
            ConsensusConfig,
            ExecutedHeader,
            ExecutedInfo,
            ExecutedResult,
            LogEntry,
            Receipt,
            ReceiptErrorWithOption,
            ReceiptWithOption,
            StateRoot,
            TransAddr,
            BatchRequest,
            Call,
            Request,
            FullTransaction,
            Response,
            SyncRequest,
            SyncResponse,
        );
    }
}

loop_macro_for_structs!(impl_convert_for_struct);

impl Into<Message> for Request {
    fn into(self) -> Message {
        Message::init_default(
            submodules::JSON_RPC,
            topics::REQUEST,
            MsgClass::REQUEST(self),
        )
    }
}

impl Into<Message> for Response {
    fn into(self) -> Message {
        Message::init_default(
            submodules::CHAIN,
            topics::RESPONSE,
            MsgClass::RESPONSE(self),
        )
    }
}

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
        signed_tx.set_signer(pubkey.to_vec());
        let bytes: Vec<u8> = (&unverified_tx).try_into().unwrap();
        signed_tx.set_tx_hash(bytes.crypt_hash().to_vec());
        signed_tx.set_transaction_with_sig(unverified_tx);
        signed_tx
    }

    /// Build UnverifiedTransaction
    pub fn build_unverified(&self, sk: PrivKey) -> UnverifiedTransaction {
        let mut unverified_tx = UnverifiedTransaction::new();
        let bytes: Vec<u8> = self.try_into().unwrap();
        let hash = bytes.crypt_hash();
        unverified_tx.set_transaction(self.clone());
        let signature = Signature::sign(&sk, &SignMessage::from(hash)).unwrap();
        unverified_tx.set_signature(signature.to_vec());
        unverified_tx.set_crypto(Crypto::SECP);
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
                Crypto::SECP => {
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
        let bytes: Vec<u8> = self.get_transaction().try_into().unwrap();
        let hash = bytes.crypt_hash();
        let mut verify_tx_req = VerifyTxReq::new();
        verify_tx_req.set_valid_until_block(self.get_transaction().get_valid_until_block());
        // tx hash
        verify_tx_req.set_hash(hash.to_vec());
        verify_tx_req.set_crypto(self.get_crypto());
        verify_tx_req.set_signature(self.get_signature().to_vec());
        verify_tx_req.set_nonce(self.get_transaction().get_nonce().to_string());
        // unverified tx hash
        let tx_hash = self.crypt_hash();
        verify_tx_req.set_tx_hash(tx_hash.to_vec());
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
        signed_tx.set_signer(public.to_vec());
        signed_tx.set_tx_hash(tx_hash.to_vec());
        signed_tx.set_transaction_with_sig(transaction);
        Ok(signed_tx)
    }

    pub fn crypt_hash(&self) -> H256 {
        H256::from(self.tx_hash.as_slice())
    }
}

impl Eq for Proof {}

impl Decodable for Proof {
    fn decode(rlp: &UntrustedRlp) -> Result<Self, DecoderError> {
        rlp.decoder()
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

    pub fn block_verify_req(&self, request_id: u64) -> VerifyBlockReq {
        let mut reqs: Vec<VerifyTxReq> = Vec::new();
        let signed_txs = self.get_body().get_transactions();
        for signed_tx in signed_txs {
            let signer = signed_tx.get_signer();
            let unverified_tx = signed_tx.get_transaction_with_sig();
            let mut verify_tx_req = unverified_tx.tx_verify_req_msg();
            verify_tx_req.set_signer(signer.to_vec());
            reqs.push(verify_tx_req);
        }
        let mut verify_blk_req = VerifyBlockReq::new();
        verify_blk_req.set_id(request_id);
        verify_blk_req.set_reqs(RepeatedField::from_vec(reqs));
        verify_blk_req
    }
}

impl BlockHeader {
    pub fn crypt_hash(&self) -> H256 {
        let bytes: Vec<u8> = self.try_into().unwrap();
        bytes.crypt_hash()
    }

    pub fn crypt_hash_hex(&self) -> String {
        let bytes: Vec<u8> = self.try_into().unwrap();
        bytes.crypt_hash().to_hex()
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
        merklehash::complete_merkle_root_raw(self.transaction_hashes().clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cmd_id_works() {
        assert_eq!(cmd_id(submodules::JSON_RPC, topics::REQUEST), 0x10001);
        assert_eq!(cmd_id(submodules::CHAIN, topics::RESPONSE), 0x30005);
    }

    #[test]
    fn create_tx() {
        let keypair = KeyPair::gen_keypair();
        let pv = keypair.privkey();

        let data = vec![1];
        let mut tx = Transaction::new();
        tx.set_data(data);
        tx.set_nonce("0".to_string());
        tx.set_to("123".to_string());
        tx.set_valid_until_block(99999);
        tx.set_quota(999999999);

        let signed_tx = tx.sign(*pv);
        assert_eq!(
            signed_tx.crypt_hash(),
            signed_tx.get_transaction_with_sig().crypt_hash()
        );
    }
}
