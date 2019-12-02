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

extern crate hashable;

use self::hashable::Hashable;
use cita_types::H256;
use rlp::{Decodable, DecoderError, Encodable, RlpStream, Rlp};
pub use static_merkle_tree::{Proof as MerkleProof, ProofNode as MerkleProofNode};

pub use self::hashable::HASH_NULL_RLP as HASH_NULL;
pub use static_merkle_tree::Tree;

#[derive(Debug, Clone)]
pub struct ProofNode {
    is_right: bool,
    hash: H256,
}

impl Encodable for ProofNode {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.begin_list(2);
        s.append(&self.is_right);
        s.append(&self.hash);
    }
}

impl Decodable for ProofNode {
    fn decode(r: &Rlp) -> Result<Self, DecoderError> {
        if r.item_count()? != 2 {
            return Err(DecoderError::RlpIncorrectListLen);
        }
        let proof_node = ProofNode {
            is_right: r.val_at(0)?,
            hash: r.val_at(1)?,
        };
        Ok(proof_node)
    }
}

#[derive(Debug, Clone)]
pub struct Proof(Vec<ProofNode>);

impl Encodable for Proof {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.append_list(&self.0);
    }
}

impl Decodable for Proof {
    fn decode(r: &Rlp) -> Result<Self, DecoderError> {
        let proof = Proof(r.as_list()?);

        Ok(proof)
    }
}

pub fn merge(left: &H256, right: &H256) -> H256 {
    let mut stream = RlpStream::new();
    stream.append(left);
    stream.append(right);
    stream.out().crypt_hash()
}

impl From<MerkleProofNode<H256>> for ProofNode {
    fn from(node: MerkleProofNode<H256>) -> Self {
        ProofNode {
            is_right: node.is_right,
            hash: node.hash,
        }
    }
}

impl From<MerkleProof<H256>> for Proof {
    fn from(proof: MerkleProof<H256>) -> Self {
        Proof(proof.0.into_iter().map(ProofNode::from).collect())
    }
}

impl From<ProofNode> for MerkleProofNode<H256> {
    fn from(node: ProofNode) -> Self {
        MerkleProofNode {
            is_right: node.is_right,
            hash: node.hash,
        }
    }
}

impl From<Proof> for MerkleProof<H256> {
    fn from(proof: Proof) -> Self {
        MerkleProof(
            proof
                .0
                .into_iter()
                .map(MerkleProofNode::<H256>::from)
                .collect(),
        )
    }
}
