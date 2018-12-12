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

use super::{Address, Error, PrivKey, PubKey};
use hashable::Hashable;
use libsm::sm2::signature::SigCtx;
use rustc_serialize::hex::ToHex;
use std::fmt;
use types::H160;
use util::crypto::CreateKey;

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
        let ctx = SigCtx::new();
        ctx.load_seckey(&privkey.0)
            .map_err(|_| Error::RecoverError)
            .map(|sk| {
                let pk = ctx.pk_from_sk(&sk);
                let pubkey = PubKey::from(&ctx.serialize_pubkey(&pk, false)[1..]);
                KeyPair { privkey, pubkey }
            })
    }

    fn gen_keypair() -> Self {
        let ctx = SigCtx::new();
        let (pk, sk) = ctx.new_keypair();
        let pubkey = PubKey::from(&ctx.serialize_pubkey(&pk, false)[1..]);
        let privkey = PrivKey::from(&ctx.serialize_seckey(&sk)[..]);
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
    use super::KeyPair;
    use util::crypto::CreateKey;

    #[test]
    fn test_gen_keypair() {
        let keypair = KeyPair::gen_keypair();
        let privkey = keypair.privkey().clone();
        let new_keypair = KeyPair::from_privkey(privkey).unwrap();
        assert_eq!(keypair.pubkey(), new_keypair.pubkey());
    }
}
