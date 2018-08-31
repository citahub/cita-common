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

mod basic;
mod block;
mod block_number;
mod call_request;
mod exchange;
mod filter;
mod log;
mod meta_data;
mod proof;
mod receipt;
mod specs;
mod transaction;
mod tx_response;

#[cfg(test)]
mod tests;

pub use self::basic::{
    BlockTag, Boolean, Data, Data20, Data32, Integer, OneItemTupleTrick, Quantity, VariadicValue,
};
pub use self::exchange::{BlockParamsByHash, BlockParamsByNumber, CountOrCode, RpcBlock};
pub use self::specs::{Id, Params, Version};

pub use self::block::{Block, BlockBody, BlockHeader};
pub use self::block_number::BlockNumber;
pub use self::call_request::CallRequest;
pub use self::filter::{Filter, FilterAddress, FilterChanges, Topic};
pub use self::log::Log;
pub use self::meta_data::MetaData;
pub use self::proof::{BftProof, Proof};
pub use self::receipt::Receipt;
pub use self::transaction::{BlockTransaction, FullTransaction, RpcTransaction, Transaction};
pub use self::tx_response::TxResponse;

use serde_json;

macro_rules! impl_from_jsonstr_for {
    ($type:ty) => {
        impl $type {
            pub fn from_jsonstr(s: &str) -> Result<Self, serde_json::Error> {
                serde_json::from_str(s)
            }
        }
    };
    ($( $type:ident ),+) => {
        $( impl_from_jsonstr_for!($type); )+
    };
    ($( $type:ident ),+ ,) => {
        impl_from_jsonstr_for!($($type),+);
    };
}

impl_from_jsonstr_for!(BlockTag, Boolean, Data, Data20, Data32, Quantity, Integer);
