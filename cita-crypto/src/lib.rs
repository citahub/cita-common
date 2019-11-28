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

#[cfg(feature = "ed25519")]
extern crate cita_ed25519;
#[cfg(feature = "secp256k1")]
extern crate cita_secp256k1;
#[cfg(feature = "sm2")]
extern crate cita_sm2;

pub use cita_crypto_trait::{CreateKey, Sign};
#[cfg(feature = "ed25519")]
pub use cita_ed25519::*;
#[cfg(feature = "secp256k1")]
pub use cita_secp256k1::*;
#[cfg(feature = "sm2")]
pub use cita_sm2::*;

#[cfg(feature = "ed25519")]
pub const SIGNATURE_NAME: &str = "ed25519";
#[cfg(feature = "secp256k1")]
pub const SIGNATURE_NAME: &str = "secp256k1";
#[cfg(feature = "sm2")]
pub const SIGNATURE_NAME: &str = "sm2";
