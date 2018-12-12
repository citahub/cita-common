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

#![allow(unused_extern_crates)]
#![feature(custom_attribute)]
#![allow(unused_attributes)]
extern crate ansi_term;
extern crate cita_types as types;
extern crate elastic_array;
extern crate git2;
extern crate hashable;
extern crate heapsize;
extern crate itertools;
extern crate libc;
extern crate lru_cache;
extern crate parking_lot;
extern crate regex;
extern crate rlp;
extern crate parity_rocksdb;
extern crate rustc_hex;
extern crate rustc_version;
extern crate target_info;
pub extern crate tiny_keccak as sha3;

#[macro_use]
extern crate logger;
extern crate panic_hook;
extern crate serde;
extern crate toml;
extern crate uuid;

pub mod avl;
pub mod build_info;
pub mod bytes;
pub mod cache;
pub mod common;
pub mod crypto;
pub mod datapath;
pub mod error;
pub mod hashdb;
pub mod instrument;
pub mod journaldb;
pub mod kvdb;
pub mod memorydb;
pub mod nibbleslice;
pub mod nibblevec;
pub mod overlaydb;
pub mod semantic_version;
pub mod snappy;
pub mod trie;
pub mod triehash;
pub mod vector;
#[macro_use]
pub mod init;

pub use ansi_term::{Colour, Style};
pub use bytes::*;
pub use datapath::*;
// pub use timer::*;
pub use error::*;
pub use hashdb::*;
pub use heapsize::HeapSizeOf;
pub use init::*;
pub use instrument::*;
pub use itertools::Itertools;
pub use journaldb::JournalDB;
pub use kvdb::*;
pub use memorydb::MemoryDB;
pub use overlaydb::*;
pub use panic_hook::set_panic_handler;
pub use parking_lot::{Condvar, Mutex, MutexGuard, RwLock, RwLockReadGuard, RwLockWriteGuard};
pub use semantic_version::*;
pub use trie::{SecTrieDB, SecTrieDBMut, Trie, TrieDB, TrieDBMut, TrieError, TrieFactory, TrieMut};
pub use triehash::*;
pub use vector::*;

pub const BLOCKLIMIT: u64 = 100;

/// Boolean type for clean/dirty status.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Filth {
    /// Data has not been changed.
    Clean,
    /// Data has been changed.
    Dirty,
}
