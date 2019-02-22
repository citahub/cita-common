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

extern crate ethereum_types;
extern crate plain_hasher;

use std::collections::{HashMap, HashSet};
use std::hash;

pub mod traits;

pub use ethereum_types::clean_0x;
pub use ethereum_types::{Bloom, BloomInput, BloomRef};
pub use ethereum_types::{H1024, H128, H160, H256, H264, H32, H512, H520, H64};
pub use ethereum_types::{U1024, U128, U256, U512, U64};
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
