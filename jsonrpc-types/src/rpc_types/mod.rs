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

mod basic;
mod block;
mod block_number;
mod call_request;
mod exchange;
mod filter;
mod log;
mod meta_data;
mod peers_info;
mod proof;
mod receipt;
mod software_version;
mod specs;
mod transaction;
mod tx_response;

#[cfg(test)]
mod tests;

pub use self::basic::{
    BlockTag, Boolean, Data, Data20, Data32, EconomicalModel, Integer, OneItemTupleTrick, Quantity,
    VariadicValue,
};
pub use self::exchange::{BlockParamsByHash, BlockParamsByNumber, CountOrCode, RpcBlock};
pub use self::specs::{Id, Params, Version};

pub use self::block::{Block, BlockBody, BlockHeader};
pub use self::block_number::BlockNumber;
pub use self::call_request::CallRequest;
pub use self::filter::{Filter, FilterAddress, FilterChanges, Topic};
pub use self::log::Log;
pub use self::meta_data::MetaData;
pub use self::peers_info::PeersInfo;
pub use self::proof::{BftProof, Proof};
pub use self::receipt::Receipt;
pub use self::software_version::SoftwareVersion;
pub use self::transaction::{BlockTransaction, FullTransaction, RpcTransaction};
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
