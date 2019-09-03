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

extern crate bincode;
extern crate cita_crypto as crypto;
extern crate cita_types as types;
extern crate hashable;
extern crate libproto;
#[macro_use]
extern crate serde_derive;
extern crate cita_directories;

mod bft_proof;

pub use bft_proof::{BftProof, Step};
use libproto::blockchain::{Proof, ProofType};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CitaProof {
    Raft,
    Bft(BftProof),
}

impl From<Proof> for CitaProof {
    fn from(p: Proof) -> Self {
        match p.get_field_type() {
            ProofType::Raft => CitaProof::Raft,
            ProofType::Bft => CitaProof::Bft(BftProof::from(p)),
            _ => CitaProof::Bft(BftProof::from(p)),
        }
    }
}

impl Into<Proof> for CitaProof {
    fn into(self) -> Proof {
        match self {
            CitaProof::Raft => Proof::new(),
            CitaProof::Bft(proof) => proof.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::bft_proof::BftProof;
    use super::CitaProof;
    use libproto::blockchain::Proof;
    use std::collections::HashMap;
    use types::H256;

    #[test]
    fn bft_proof_convert() {
        let o_proof = CitaProof::Bft(BftProof::new(0, 1, H256::default(), HashMap::new()));
        let proto_proof: Proof = o_proof.clone().into();
        let de_proof: CitaProof = proto_proof.into();
        assert_eq!(o_proof, de_proof);
    }
}
