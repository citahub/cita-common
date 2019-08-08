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
