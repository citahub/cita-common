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

use crypto::{CreateKey, KeyPair, Message as SignMessage, PrivKey, PubKey, Sign, Signature, SIGNATURE_BYTES_LEN};
use protobuf::{parse_from_bytes, Message as MessageTrait, RepeatedField};
use rlp::{Decodable, DecoderError, Encodable, RlpStream, UntrustedRlp};
use rustc_serialize::hex::ToHex;
use std::fmt;
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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SubModules {
    Jsonrpc,
    Net,
    Chain,
    Consensus,
    Auth,
    Executor,
    Unknown,
}

pub const SUBMODULES_UNKNOWN: SubModules = SubModules::Unknown;

impl fmt::Display for SubModules {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                // Please use the same rules to name the string
                &SubModules::Jsonrpc => "jsonrpc",
                &SubModules::Net => "net",
                &SubModules::Chain => "chain",
                &SubModules::Consensus => "consensus",
                &SubModules::Auth => "auth",
                &SubModules::Executor => "executor",
                &SubModules::Unknown => "unknown",
            }
        )
    }
}

impl<'a> From<&'a str> for SubModules {
    fn from(s: &'a str) -> SubModules {
        let parts: Vec<&str> = s.split('.').collect();
        if parts.len() > 1 {
            match parts[0] {
                "jsonrpc" => SubModules::Jsonrpc,
                "net" => SubModules::Net,
                "chain" => SubModules::Chain,
                "consensus" => SubModules::Consensus,
                "auth" => SubModules::Auth,
                "executor" => SubModules::Executor,
                _ => SubModules::Unknown,
            }
        } else {
            SUBMODULES_UNKNOWN
        }
    }
}

#[derive(Debug)]
pub enum MsgClass {
    RawBytes(Vec<u8>),
    Request(Request),
    Response(Response),
    SyncRequest(SyncRequest),
    SyncResponse(SyncResponse),
    Status(Status),
    RichStatus(RichStatus),
    SignedProposal(SignedProposal),
    Block(Block),
    BlockWithProof(BlockWithProof),
    BlockHeader(BlockHeader),
    BlockTxs(BlockTxs),
    BlockTxHashes(BlockTxHashes),
    BlockTxHashesReq(BlockTxHashesReq),
    VerifyTxReq(VerifyTxReq),
    VerifyTxResp(VerifyTxResp),
    VerifyBlockReq(VerifyBlockReq),
    VerifyBlockResp(VerifyBlockResp),
    ExecutedResult(ExecutedResult),
    Empty,
}

const ZERO_ORIGIN: u32 = 99999;

impl From<Message_oneof_content> for MsgClass {
    fn from(content: Message_oneof_content) -> Self {
        match content {
            Message_oneof_content::RawBytes(data) => {
                let mut content = Vec::new();
                content.extend_from_slice(&snappy::cita_decompress(data));
                content.into()
            }
            // Generate MSG-PROTOS from_content automatically begin:
            Message_oneof_content::Request(data) => data.into(),
            Message_oneof_content::Response(data) => data.into(),
            Message_oneof_content::SyncRequest(data) => data.into(),
            Message_oneof_content::SyncResponse(data) => data.into(),
            Message_oneof_content::Status(data) => data.into(),
            Message_oneof_content::RichStatus(data) => data.into(),
            Message_oneof_content::SignedProposal(data) => data.into(),
            Message_oneof_content::Block(data) => data.into(),
            Message_oneof_content::BlockWithProof(data) => data.into(),
            Message_oneof_content::BlockHeader(data) => data.into(),
            Message_oneof_content::BlockTxs(data) => data.into(),
            Message_oneof_content::BlockTxHashes(data) => data.into(),
            Message_oneof_content::BlockTxHashesReq(data) => data.into(),
            Message_oneof_content::VerifyTxReq(data) => data.into(),
            Message_oneof_content::VerifyTxResp(data) => data.into(),
            Message_oneof_content::VerifyBlockReq(data) => data.into(),
            Message_oneof_content::VerifyBlockResp(data) => data.into(),
            Message_oneof_content::ExecutedResult(data) => data.into(),
            // Generate MSG-PROTOS from_content automatically end.
        }
    }
}

impl From<Option<Message_oneof_content>> for MsgClass {
    fn from(content: Option<Message_oneof_content>) -> Self {
        match content {
            Some(inner) => inner.into(),
            None => MsgClass::Empty,
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
            MsgClass::RawBytes(data) => self.set_RawBytes(snappy::cita_compress(data)),
            // Generate MSG-PROTOS set_content automatically begin:
            MsgClass::Request(data) => self.set_Request(data),
            MsgClass::Response(data) => self.set_Response(data),
            MsgClass::SyncRequest(data) => self.set_SyncRequest(data),
            MsgClass::SyncResponse(data) => self.set_SyncResponse(data),
            MsgClass::Status(data) => self.set_Status(data),
            MsgClass::RichStatus(data) => self.set_RichStatus(data),
            MsgClass::SignedProposal(data) => self.set_SignedProposal(data),
            MsgClass::Block(data) => self.set_Block(data),
            MsgClass::BlockWithProof(data) => self.set_BlockWithProof(data),
            MsgClass::BlockHeader(data) => self.set_BlockHeader(data),
            MsgClass::BlockTxs(data) => self.set_BlockTxs(data),
            MsgClass::BlockTxHashes(data) => self.set_BlockTxHashes(data),
            MsgClass::BlockTxHashesReq(data) => self.set_BlockTxHashesReq(data),
            MsgClass::VerifyTxReq(data) => self.set_VerifyTxReq(data),
            MsgClass::VerifyTxResp(data) => self.set_VerifyTxResp(data),
            MsgClass::VerifyBlockReq(data) => self.set_VerifyBlockReq(data),
            MsgClass::VerifyBlockResp(data) => self.set_VerifyBlockResp(data),
            MsgClass::ExecutedResult(data) => self.set_ExecutedResult(data),
            // Generate MSG-PROTOS set_content automatically end.
            MsgClass::Empty => self.clear_content(),
        };
    }

    pub fn get_content(&self) -> MsgClass {
        self.content.clone().into()
    }

    pub fn take_content(&mut self) -> MsgClass {
        self.content.take().into()
    }

    pub fn init(operate: OperateType, origin: u32, mc: MsgClass) -> Self {
        let mut msg = Message::new();
        msg.set_origin(origin);
        msg.set_operate(operate);
        msg.set_content(mc);
        msg
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

impl Into<MsgClass> for Vec<u8> {
    fn into(self) -> MsgClass {
        MsgClass::RawBytes(self)
    }
}

impl Into<Message> for Vec<u8> {
    fn into(self) -> Message {
        Message::init(OperateType::BROADCAST, ZERO_ORIGIN, self.into())
    }
}

macro_rules! impl_convert_for_struct_in_msg {
    ($( $struct:ident, )+) => {
        $(
            impl_convert_for_struct_in_msg!($struct);
        )+
    };
    ($struct:ident) => {

       impl Into<MsgClass> for $struct {
           fn into(self) -> MsgClass {
               MsgClass::$struct(self)
           }
       }

        impl Into<Message> for $struct {
            fn into(self) -> Message {
                Message::init(OperateType::BROADCAST, ZERO_ORIGIN, self.into())
            }
        }
    };
}

macro_rules! loop_macro_for_structs {
    ($macro:ident) => {
        $macro!(
            // Generate ALL-PROTOS automatically begin:
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
            BatchRequest,
            Call,
            Request,
            FullTransaction,
            Response,
            SyncRequest,
            SyncResponse,
            // Generate ALL-PROTOS automatically end.
        );
    }
}

macro_rules! loop_macro_for_structs_in_msg {
    ($macro:ident) => {
        $macro!(
            // Generate MSG-PROTOS automatically begin:
            Request,
            Response,
            SyncRequest,
            SyncResponse,
            Status,
            RichStatus,
            SignedProposal,
            Block,
            BlockWithProof,
            BlockHeader,
            BlockTxs,
            BlockTxHashes,
            BlockTxHashesReq,
            VerifyTxReq,
            VerifyTxResp,
            VerifyBlockReq,
            VerifyBlockResp,
            ExecutedResult,
            // Generate MSG-PROTOS automatically end.
        );
    }
}

loop_macro_for_structs!(impl_convert_for_struct);
loop_macro_for_structs_in_msg!(impl_convert_for_struct_in_msg);

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

        let signed_tx = tx.sign(*pv);
        assert_eq!(
            signed_tx.crypt_hash(),
            signed_tx.get_transaction_with_sig().crypt_hash()
        );
    }

    #[test]
    fn sub_modules_works() {
        use super::SubModules;
        use std::convert::From;
        let s = "jsonrpc.anything-is-ok";
        let sm = SubModules::from(s);
        assert_eq!(sm, SubModules::Jsonrpc);
    }

    #[test]
    fn class_funcs_for_message_works() {
        use super::{Message, MsgClass, Request, Response};
        let req_origin = Request::new();
        let resp_origin = Response::new();
        let mut msg: Message = req_origin.into();

        let req_clone = msg.get_content();
        assert!(msg.has_content());
        assert!(if let MsgClass::Request(_) = req_clone {
            true
        } else {
            false
        });

        let req_take = msg.take_content();
        assert!(!msg.has_content());
        assert!(if let MsgClass::Request(_) = req_take {
            true
        } else {
            false
        });

        msg.set_content(resp_origin.into());
        assert!(msg.has_content());

        let resp_clone = msg.get_content();
        assert!(msg.has_content());
        assert!(if let MsgClass::Response(_) = resp_clone {
            true
        } else {
            false
        });

        msg.clear_content();
        assert!(!msg.has_content());
    }

    #[test]
    fn traits_for_protos_works() {
        use super::blockchain;
        use std::convert::{TryFrom, TryInto};
        let height: u64 = 13579;
        let mut status = blockchain::Status::new();
        status.set_height(height);
        let status_bytes: Vec<u8> = status.try_into().unwrap();
        let status_new = blockchain::Status::try_from(&status_bytes).unwrap();
        let height_new = status_new.get_height();
        assert_eq!(height, height_new);
    }
}
