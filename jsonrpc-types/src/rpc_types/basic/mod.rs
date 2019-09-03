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

mod arbitrary_data;
mod boolean;
mod fixed_data;
mod integer;
mod quantity;
mod tags;
mod variadic;

pub use self::arbitrary_data::Data;
pub use self::boolean::Boolean;
pub use self::fixed_data::{Data20, Data32};
pub use self::integer::Integer;
pub use self::quantity::Quantity;
pub use self::tags::{BlockTag, EconomicalModel};
pub use self::variadic::VariadicValue;

// serde: Tuple enums with single element should not be a json-array
// https://github.com/serde-rs/serde/pull/111
#[derive(Debug, Clone, PartialEq)]
pub struct OneItemTupleTrick {}

impl Default for OneItemTupleTrick {
    fn default() -> Self {
        OneItemTupleTrick {}
    }
}
