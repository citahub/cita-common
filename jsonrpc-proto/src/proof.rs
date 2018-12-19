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
use jsonrpc_types::rpctypes::{BftProof, Proof};
use libproto::blockchain::{Proof as ProtoProof, ProofType as ProtoProofType};

use crate::from_into::FromProto;

impl FromProto<libproto::Proof> for Proof {
    fn from_proto(p: ProtoProof) -> Self {
        match p.get_field_type() {
            ProtoProofType::AuthorityRound => Proof::Bft(BftProof::from_proto(p)),
            ProtoProofType::Raft => Proof::Raft,
            ProtoProofType::Bft => Proof::Bft(BftProof::from_proto(p)),
        }
    }
}

impl FromProto<ProtoProof> for BftProof {
    fn from_proto(p: ProtoProof) -> Self {
        use common_proof::BftProof as CommonBftProof;

        // FIXME: remove unwrap!!!!
        let decoded: CommonBftProof = deserialize(&p.get_content()[..]).unwrap();
        let mut commits: HashMap<Address, String> = HashMap::new();
        let str_0x = "0x".to_string();

        for (addr, sign) in decoded.commits {
            commits.insert(addr, str_0x.clone() + &String::from(sign));
        }

        BftProof {
            proposal: decoded.proposal,
            height: decoded.height,
            round: decoded.round,
            commits,
        }
    }
}
