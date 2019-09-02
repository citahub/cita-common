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

use super::{Address, PrivKey, PubKey};
use crate::error::Error;
use crate::types::H160;
use cita_crypto_trait::CreateKey;
use hashable::Hashable;
use rustc_serialize::hex::ToHex;
use sodiumoxide::crypto::sign::gen_keypair;
use std::fmt;

pub fn pubkey_to_address(pubkey: &PubKey) -> Address {
    H160::from(pubkey.crypt_hash())
}

#[derive(Default)]
pub struct KeyPair {
    privkey: PrivKey,
    pubkey: PubKey,
}

impl fmt::Display for KeyPair {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        writeln!(f, "privkey:  {}", self.privkey.0.to_hex())?;
        writeln!(f, "pubkey:  {}", self.pubkey.0.to_hex())?;
        write!(f, "address:  {}", self.address().0.to_hex())
    }
}

impl CreateKey for KeyPair {
    type PrivKey = PrivKey;
    type PubKey = PubKey;
    type Error = Error;

    fn from_privkey(privkey: Self::PrivKey) -> Result<Self, Self::Error> {
        let pubkey = PubKey::from(&privkey.0[32..]);
        Ok(KeyPair { privkey, pubkey })
    }

    fn gen_keypair() -> Self {
        let (pk, sk) = gen_keypair();
        KeyPair {
            privkey: PrivKey::from(sk.0),
            pubkey: PubKey::from(pk.0),
        }
    }

    fn privkey(&self) -> &Self::PrivKey {
        &self.privkey
    }

    fn pubkey(&self) -> &Self::PubKey {
        &self.pubkey
    }

    fn address(&self) -> Address {
        pubkey_to_address(&self.pubkey)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cita_crypto_trait::CreateKey;

    #[test]
    fn test_from_privkey() {
        let keypair1 = KeyPair::gen_keypair();
        let keypair2 = KeyPair::from_privkey(keypair1.privkey).unwrap();
        assert_eq!(keypair1.pubkey, keypair2.pubkey);
        assert_eq!(keypair1.privkey, keypair2.privkey);
    }
}
