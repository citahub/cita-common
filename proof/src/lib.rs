// CITA
// Copyright 2016-2017 Cryptape Technologies LLC.

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

extern crate bincode;
extern crate cita_crypto as crypto;
extern crate cita_types as types;
extern crate hashable;
extern crate libproto;
extern crate rustc_serialize;
#[macro_use]
extern crate serde_derive;
extern crate cita_directories;

mod bft_proof;

pub use bft_proof::BftProof;
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
