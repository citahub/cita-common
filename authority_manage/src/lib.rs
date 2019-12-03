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

extern crate cita_types as types;

pub mod wal;

use crate::types::Address;
use crate::wal::Wal;
use bincode::{deserialize, serialize};

pub const DATA_PATH: &str = "DATA_PATH";
pub const LOG_TYPE_AUTHORITIES: u8 = 1;

#[derive(Debug)]
pub struct AuthorityManage {
    pub authorities: Vec<Address>,
    pub validators: Vec<Address>,
    pub authorities_log: Wal,
    pub authorities_old: Vec<Address>,
    pub validators_old: Vec<Address>,
    pub authority_h_old: usize,
}

impl Default for AuthorityManage {
    fn default() -> Self {
        Self::new()
    }
}

impl AuthorityManage {
    pub fn new() -> Self {
        let logpath = ::std::env::var(DATA_PATH)
            .unwrap_or_else(|_| panic!("{} must be set", DATA_PATH))
            + "/authorities";

        let mut authority_manage = AuthorityManage {
            authorities: Vec::new(),
            validators: Vec::new(),
            authorities_log: Wal::create(&*logpath).unwrap(),
            authorities_old: Vec::new(),
            validators_old: Vec::new(),
            authority_h_old: 0,
        };

        let vec_out = authority_manage.authorities_log.load();
        if !vec_out.is_empty() {
            if let Ok((h, authorities, validators_old, validators)) = deserialize(&(vec_out[0].1)) {
                let authorities: Vec<Address> = authorities;
                let validators_old: Vec<Address> = validators_old;
                let validators: Vec<Address> = validators;

                authority_manage.authorities.extend_from_slice(&authorities);
                authority_manage
                    .validators_old
                    .extend_from_slice(&validators_old);
                authority_manage.authority_h_old = h;
                authority_manage.validators.extend_from_slice(&validators);
            }
        }

        authority_manage
    }

    pub fn validator_n(&self) -> usize {
        self.validators.len()
    }

    pub fn receive_authorities_list(
        &mut self,
        height: usize,
        authorities: &[Address],
        validators: &[Address],
    ) {
        let flag = if self.validators != validators {
            2
        } else if self.authorities != authorities {
            1
        } else {
            0
        };

        if flag > 0 {
            self.authorities_old.clear();
            self.authorities_old.extend_from_slice(&self.authorities);
            self.validators_old.clear();
            self.validators_old.extend_from_slice(&self.validators);
            self.authority_h_old = height;

            self.authorities.clear();
            self.authorities.extend_from_slice(&authorities);
            self.validators.clear();
            self.validators.extend_from_slice(&validators);

            if flag == 2 {
                self.save();
            }
        }
    }

    pub fn save(&mut self) {
        let bmsg = serialize(&(
            self.authority_h_old,
            self.authorities.clone(),
            self.validators_old.clone(),
            self.validators.clone(),
        ))
        .unwrap();
        let _ = self.authorities_log.save(LOG_TYPE_AUTHORITIES, &bmsg);
    }
}
