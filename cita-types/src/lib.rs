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

extern crate ethereum_types;
extern crate plain_hasher;

use std::collections::{HashMap, HashSet};
use std::hash;

pub mod traits;

pub use ethereum_types::clean_0x;
pub use ethereum_types::{Bloom, BloomInput, BloomRef};
pub use ethereum_types::{H128, H160, H256, H264, H32, H512, H520, H64};
pub use ethereum_types::{U128, U256, U512, U64};
pub use plain_hasher::PlainHasher;

pub type Address = H160;

/// Specialized version of `HashMap` with H256 keys and fast hashing function.
pub type H256FastMap<T> = HashMap<H256, T, hash::BuildHasherDefault<PlainHasher>>;
/// Specialized version of `HashSet` with H256 keys and fast hashing function.
pub type H256FastSet = HashSet<H256, hash::BuildHasherDefault<PlainHasher>>;

#[inline]
pub fn pad_left0(hexstr: &str, width: usize) -> String {
    format!("{:0>width$}", hexstr, width = width)
}

#[inline]
pub fn delete_left0(s: &str) -> &str {
    let mut cnt = 0;
    for i in s.chars() {
        if i == '0' {
            cnt += 1;
        } else {
            break;
        }
    }
    &s[cnt..]
}
