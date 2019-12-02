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

use crate::types::{Address, H256, H512};

mod error;
mod keypair;
mod signature;
mod signer;

pub use self::error::*;
pub use self::keypair::*;
pub use self::signature::*;
pub use self::signer::*;

pub type PrivKey = H256;
pub type PubKey = H512;
pub type Message = H256;

pub const ADDR_BYTES_LEN: usize = 20;
pub const PUBKEY_BYTES_LEN: usize = 64;
pub const PRIVKEY_BYTES_LEN: usize = 32;
pub const SIGNATURE_BYTES_LEN: usize = 128;
pub const HASH_BYTES_LEN: usize = 32;
