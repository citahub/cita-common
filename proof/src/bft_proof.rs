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

use bincode::{deserialize, serialize, Infinite};
use cita_directories::DataPath;
use crypto::{pubkey_to_address, Sign, Signature};
use hashable::Hashable;
use libproto::blockchain::{Proof, ProofType};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::usize::MAX;
use types::{Address, H256};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Step {
    Propose,
    ProposeWait,
    Prevote,
    PrevoteWait,
    PrecommitAuth,
    Precommit,
    PrecommitWait,
    Commit,
    CommitWait,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct BftProof {
    pub proposal: H256,
    // Prev height
    pub height: usize,
    pub round: usize,
    pub commits: HashMap<Address, Signature>,
}

impl BftProof {
    pub fn new(
        height: usize,
        round: usize,
        proposal: H256,
        commits: HashMap<Address, Signature>,
    ) -> BftProof {
        BftProof {
            height,
            round,
            proposal,
            commits,
        }
    }

    pub fn default() -> Self {
        BftProof {
            height: MAX,
            round: MAX,
            proposal: H256::default(),
            commits: HashMap::new(),
        }
    }

    pub fn store(&self) {
        let proof_path = DataPath::proof_bin_path();
        let mut file = File::create(&proof_path).unwrap();
        let encoded_proof: Vec<u8> = serialize(&self, Infinite).unwrap();
        file.write_all(&encoded_proof).unwrap();
        let _ = file.sync_all();
    }

    pub fn load(&mut self) {
        let proof_path = DataPath::proof_bin_path();
        if let Ok(mut file) = File::open(&proof_path) {
            let mut content = Vec::new();
            if file.read_to_end(&mut content).is_ok() {
                if let Ok(decoded) = deserialize(&content[..]) {
                    //self.round = decoded.round;
                    //self.proposal = decoded.proposal;
                    //self.commits = decoded.commits;
                    *self = decoded;
                }
            }
        }
    }

    pub fn is_default(&self) -> bool {
        if self.round == MAX {
            return true;
        }
        false
    }

    // Check proof commits
    pub fn check(&self, h: usize, authorities: &[Address]) -> bool {
        if h == 0 {
            return true;
        }
        if h != self.height {
            return false;
        }
        if 2 * authorities.len() >= 3 * self.commits.len() {
            return false;
        }
        self.commits.iter().all(|(sender, sig)| {
            if authorities.contains(sender) {
                let msg = serialize(
                    &(h, self.round, Step::Precommit, sender, Some(self.proposal)),
                    Infinite,
                )
                .unwrap();
                let signature = Signature(sig.0);
                if let Ok(pubkey) = signature.recover(&msg.crypt_hash()) {
                    return pubkey_to_address(&pubkey) == *sender;
                }
            }
            false
        })
    }
}

impl From<Proof> for BftProof {
    fn from(p: Proof) -> Self {
        let decoded: BftProof = deserialize(&p.get_content()[..]).unwrap();
        decoded
    }
}

impl Into<Proof> for BftProof {
    fn into(self) -> Proof {
        let mut proof = Proof::new();
        let encoded_proof: Vec<u8> = serialize(&self, Infinite).unwrap();
        proof.set_content(encoded_proof);
        proof.set_field_type(ProofType::Bft);
        proof
    }
}

#[cfg(test)]
mod tests {
    use super::{BftProof, H256};
    use libproto::blockchain::Proof;
    use std::collections::HashMap;

    #[test]
    fn proof_convert() {
        let o_proof = BftProof::new(0, 1, H256::default(), HashMap::new());
        let proto_proof: Proof = o_proof.clone().into();
        let de_proof: BftProof = proto_proof.into();
        assert_eq!(o_proof, de_proof);
    }
}
