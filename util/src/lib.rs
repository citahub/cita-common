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

extern crate ansi_term;
extern crate cita_types as types;
extern crate elastic_array;
extern crate git2;
extern crate heapsize;
extern crate lru_cache;
// TODO: remove rlp
extern crate parking_lot;
extern crate rlp;
extern crate rustc_hex;
extern crate rustc_version;
extern crate snappy;
pub extern crate tiny_keccak as sha3;

extern crate panic_hook;
extern crate serde;
extern crate toml;

pub mod build_info;
pub mod instrument;
pub mod semantic_version;
#[macro_use]
pub mod init;

pub use crate::init::*;
pub use crate::instrument::*;
pub use crate::semantic_version::*;
pub use ansi_term::{Colour, Style};
pub use heapsize::HeapSizeOf;
pub use panic_hook::set_panic_handler;
pub use parking_lot::{Condvar, Mutex, MutexGuard, RwLock, RwLockReadGuard, RwLockWriteGuard};

pub const BLOCKLIMIT: u64 = 100;

/// Boolean type for clean/dirty status.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Filth {
    /// Data has not been changed.
    Clean,
    /// Data has been changed.
    Dirty,
}
