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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum Id {
    /// No id (notification)
    Null,
    /// String id
    Str(String),
    /// Numeric id
    Num(u64),
}

impl Default for Id {
    fn default() -> Self {
        Id::Null
    }
}

impl Id {
    pub fn is_null(&self) -> bool {
        *self == Id::Null
    }
}
