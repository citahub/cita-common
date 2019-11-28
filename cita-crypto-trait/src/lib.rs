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

use cita_types::Address;

pub trait Sign
where
    Self: ::std::marker::Sized,
{
    type PrivKey;
    type PubKey;
    type Message;
    type Error;

    fn sign(privkey: &Self::PrivKey, message: &Self::Message) -> Result<Self, Self::Error>;
    fn recover(&self, message: &Self::Message) -> Result<Self::PubKey, Self::Error>;
    fn verify_public(
        &self,
        pubkey: &Self::PubKey,
        message: &Self::Message,
    ) -> Result<bool, Self::Error>;
    fn verify_address(
        &self,
        address: &Address,
        message: &Self::Message,
    ) -> Result<bool, Self::Error>;
}

pub trait CreateKey
where
    Self: ::std::marker::Sized,
{
    type PrivKey;
    type PubKey;
    type Error;

    fn from_privkey(privkey: Self::PrivKey) -> Result<Self, Self::Error>;
    fn gen_keypair() -> Self;
    fn privkey(&self) -> &Self::PrivKey;
    fn pubkey(&self) -> &Self::PubKey;
    fn address(&self) -> Address;
}
