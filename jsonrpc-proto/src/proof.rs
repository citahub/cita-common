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
