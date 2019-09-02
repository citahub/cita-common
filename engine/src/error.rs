// Copyright Cryptape Technologies LLC.
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

use crate::crypto::Signature;
use crate::types::Address;
use std::fmt;

#[derive(Debug)]
pub enum EngineError {
    NotAuthorized(Address),
    NotProposer(Mismatch<Address>),
    DoubleVote(Address),
    FutureBlock(u64),
    NotAboveThreshold(usize),
    BadSignature(Signature),
    InvalidProof,
    InvalidTimeInterval,
    /// Message was not expected.
    UnexpectedMessage,
    VoteMsgDelay(usize),
    VoteMsgForth(usize),
    InvalidSignature,
    InvalidTxInProposal,
}

impl fmt::Display for EngineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::EngineError::*;
        let msg = match *self {
            NotProposer(ref mis) => format!("Author is not a current proposer: {}", mis),
            NotAuthorized(ref address) => format!("Signer {} is not authorized.", address),
            DoubleVote(ref address) => format!("Author {} issued too many blocks.", address),
            BadSignature(ref signature) => format!("bad signature {}", signature),
            FutureBlock(time) => format!("Block from future: {}", time),
            InvalidTimeInterval => "Invalid Time Interval".into(),
            InvalidProof => "Invalid proof.".into(),
            NotAboveThreshold(vote) => format!("Vote is not above threshold: {}", vote),
            UnexpectedMessage => "This Engine should not be fed messages.".into(),
            VoteMsgDelay(height) => format!(
                "The vote message is delayed and missed the current height:{}",
                height
            ),
            VoteMsgForth(height) => format!("The vote message is fulture height :{}", height),
            InvalidSignature => "Invalid Signature.".into(),
            InvalidTxInProposal => "Invalid Tx In Proposal.".into(),
        };
        f.write_fmt(format_args!("Engine error ({})", msg))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
/// Error indicating an expected value was not found.
pub struct Mismatch<T: fmt::Debug> {
    /// Value expected.
    pub expected: T,
    /// Value found.
    pub found: T,
}

impl<T: fmt::Debug + fmt::Display> fmt::Display for Mismatch<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_fmt(format_args!(
            "Expected {}, found {}",
            self.expected, self.found
        ))
    }
}
