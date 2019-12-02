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

use std::collections::HashMap;

use bincode::deserialize;
use cita_types::Address;
use jsonrpc_types::{
    rpc_types::{BftProof, Proof},
    Error,
};
use libproto::blockchain::{Proof as ProtoProof, ProofType as ProtoProofType};

use crate::{error::ErrorExt, from_into::TryFromProto};

impl TryFromProto<libproto::Proof> for Proof {
    type Error = Error;

    fn try_from_proto(p: ProtoProof) -> Result<Self, Self::Error> {
        let proof = match p.get_field_type() {
            ProtoProofType::AuthorityRound | ProtoProofType::Bft => {
                Proof::Bft(BftProof::try_from_proto(p)?)
            }
            ProtoProofType::Raft => Proof::Raft,
        };

        Ok(proof)
    }
}

impl TryFromProto<ProtoProof> for BftProof {
    type Error = Error;

    fn try_from_proto(p: ProtoProof) -> Result<Self, Self::Error> {
        use crate::proof_srv::BftProof as SrvBftProof;

        let decoded: SrvBftProof = deserialize(&p.get_content()[..]) //
            .map_err(Error::bft_proof_decode_error)?;
        let mut commits: HashMap<Address, String> = HashMap::new();
        let str_0x = "0x".to_string();

        for (addr, sign) in decoded.commits {
            commits.insert(addr, str_0x.clone() + &String::from(sign));
        }

        Ok(BftProof {
            proposal: decoded.proposal,
            height: decoded.height,
            round: decoded.round,
            commits,
        })
    }
}
