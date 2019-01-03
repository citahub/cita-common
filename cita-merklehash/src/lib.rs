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
// along with this program.  If not, see <http://www.gnu.org/licenses/>

extern crate cita_types;
extern crate hashable;
extern crate rlp;
#[macro_use]
extern crate rlp_derive;
extern crate static_merkel_tree;

use self::hashable::Hashable;
use cita_types::H256;
use rlp::RlpStream;
use static_merkel_tree::{Proof as MerkelProof, ProofNode as MerkelProofNode};

pub use self::hashable::HASH_NULL_RLP as HASH_NULL;
pub use static_merkel_tree::Tree;

#[derive(Debug, Clone, RlpEncodable, RlpDecodable)]
pub struct ProofNode {
    is_right: bool,
    hash: H256,
}

#[derive(Debug, Clone, RlpEncodableWrapper, RlpDecodableWrapper)]
pub struct Proof(Vec<ProofNode>);

pub fn merge(left: &H256, right: &H256) -> H256 {
    let mut stream = RlpStream::new();
    stream.append(left);
    stream.append(right);
    stream.out().crypt_hash()
}

impl From<MerkelProofNode<H256>> for ProofNode {
    fn from(node: MerkelProofNode<H256>) -> Self {
        ProofNode {
            is_right: node.is_right,
            hash: node.hash,
        }
    }
}

impl From<MerkelProof<H256>> for Proof {
    fn from(proof: MerkelProof<H256>) -> Self {
        Proof(proof.0.into_iter().map(|n| ProofNode::from(n)).collect())
    }
}
