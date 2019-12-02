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

use super::{Address, Error, PrivKey, PubKey, SECP256K1};
use crate::types::H160;
use cita_crypto_trait::CreateKey;
use hashable::Hashable;
use rand::thread_rng;
use rustc_serialize::hex::ToHex;
use secp256k1::key;
use std::fmt;

pub fn pubkey_to_address(pubkey: &PubKey) -> Address {
    H160::from(pubkey.crypt_hash())
}

/// key pair
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

    /// Create a pair from secret key
    fn from_privkey(privkey: Self::PrivKey) -> Result<Self, Self::Error> {
        let context = &SECP256K1;
        let s: key::SecretKey = key::SecretKey::from_slice(&privkey.0[..])?;
        let pubkey = key::PublicKey::from_secret_key(context, &s);
        let serialized = pubkey.serialize_uncompressed();

        let mut pubkey = PubKey::default();
        pubkey.0.copy_from_slice(&serialized[1..65]);

        let keypair = KeyPair { privkey, pubkey };

        Ok(keypair)
    }

    fn gen_keypair() -> Self {
        let context = &SECP256K1;
        let (s, p) = context.generate_keypair(&mut thread_rng());
        let serialized = p.serialize_uncompressed();
        let mut privkey = PrivKey::default();
        privkey.0.copy_from_slice(&s[0..32]);
        let mut pubkey = PubKey::default();
        pubkey.0.copy_from_slice(&serialized[1..65]);
        KeyPair { privkey, pubkey }
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
    use super::{KeyPair, PrivKey};
    use crate::types::H256;
    use cita_crypto_trait::CreateKey;
    use std::str::FromStr;

    #[test]
    fn from_privkey() {
        let privkey = PrivKey::from(
            H256::from_str("a100df7a048e50ed308ea696dc600215098141cb391e9527329df289f9383f65")
                .unwrap(),
        );
        let _ = KeyPair::from_privkey(privkey).unwrap();
    }
}
