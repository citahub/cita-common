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

use super::{
    pubkey_to_address, Address, Error, KeyPair, Message, PrivKey, PubKey, SIGNATURE_BYTES_LEN,
};
use cita_crypto_trait::{CreateKey, Sign};
use ring::signature;
use ring::signature::{EcdsaKeyPair, ECDSA_SM2P256_SM3_ASN1_SIGNING};
use rlp::*;
use rustc_serialize::hex::ToHex;
use serde::de::{Error as SerdeError, SeqAccess, Visitor};
use serde::ser::SerializeSeq;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::{Deref, DerefMut};

pub struct Signature(pub [u8; 128]);

impl Signature {
    #[inline]
    fn r(&self) -> &[u8] {
        &self.0[0..32]
    }

    #[inline]
    fn s(&self) -> &[u8] {
        &self.0[32..64]
    }

    #[inline]
    fn pk(&self) -> &[u8] {
        &self.0[64..]
    }
}

impl PartialEq for Signature {
    fn eq(&self, other: &Self) -> bool {
        self.0[..] == other.0[..]
    }
}

impl Decodable for Signature {
    fn decode(rlp: &UntrustedRlp) -> Result<Self, DecoderError> {
        rlp.decoder().decode_value(|bytes| {
            let mut sig = [0u8; 128];
            sig.copy_from_slice(bytes);
            Ok(Signature(sig))
        })
    }
}

impl Encodable for Signature {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.encoder().encode_value(&self.0[0..SIGNATURE_BYTES_LEN]);
    }
}

impl<'de> Deserialize<'de> for Signature {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SignatureVisitor;

        impl<'de> Visitor<'de> for SignatureVisitor {
            type Value = Signature;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("sm2 signature")
            }

            fn visit_seq<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let mut signature = Signature([0u8; SIGNATURE_BYTES_LEN]);
                for i in 0..SIGNATURE_BYTES_LEN {
                    signature.0[i] = match visitor.next_element()? {
                        Some(val) => val,
                        None => return Err(SerdeError::invalid_length(SIGNATURE_BYTES_LEN, &self)),
                    }
                }
                Ok(signature)
            }
        }

        let visitor = SignatureVisitor;
        deserializer.deserialize_seq(visitor)
    }
}

impl Serialize for Signature {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(SIGNATURE_BYTES_LEN))?;
        for i in 0..SIGNATURE_BYTES_LEN {
            seq.serialize_element(&self.0[i])?;
        }
        seq.end()
    }
}

// manual implementation required in Rust 1.13+, see `std::cmp::AssertParamIsEq`.
impl Eq for Signature {}

// also manual for the same reason, but the pretty printing might be useful.
impl fmt::Debug for Signature {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.debug_struct("Signature")
            .field("r", &self.r().to_hex())
            .field("s", &self.s().to_hex())
            .field("pk", &self.pk().to_hex())
            .finish()
    }
}

impl fmt::Display for Signature {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.to_hex())
    }
}

impl Default for Signature {
    fn default() -> Self {
        Signature([0; 128])
    }
}

impl Hash for Signature {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl Clone for Signature {
    fn clone(&self) -> Self {
        Signature(self.0)
    }
}

impl From<[u8; 128]> for Signature {
    fn from(s: [u8; 128]) -> Self {
        Signature(s)
    }
}

impl Into<[u8; 128]> for Signature {
    fn into(self) -> [u8; 128] {
        self.0
    }
}

impl<'a> From<&'a [u8]> for Signature {
    fn from(slice: &'a [u8]) -> Signature {
        assert_eq!(slice.len(), SIGNATURE_BYTES_LEN);
        let mut bytes = [0u8; SIGNATURE_BYTES_LEN];
        bytes.copy_from_slice(&slice[..]);
        Signature(bytes)
    }
}

impl<'a> Into<&'a [u8]> for &'a Signature {
    fn into(self) -> &'a [u8] {
        &self.0[..]
    }
}

impl fmt::LowerHex for Signature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in &self.0[..] {
            write!(f, "{:02x}", i)?;
        }
        Ok(())
    }
}

impl From<Signature> for String {
    fn from(s: Signature) -> Self {
        format!("{:x}", s)
    }
}

impl Deref for Signature {
    type Target = [u8; 128];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Signature {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Sign for Signature {
    type PrivKey = PrivKey;
    type PubKey = PubKey;
    type Message = Message;
    type Error = Error;

    fn sign(privkey: &Self::PrivKey, message: &Self::Message) -> Result<Self, Error> {
        let rng = ring::rand::SystemRandom::new();
        let key_pair = KeyPair::from_privkey(*privkey)?;

        let sig = key_pair
            .inner
            .sign(&rng, &message)
            .map_err(|_| Error::SignError)?;
        let signature = untrusted::Input::from(sig.as_ref());
        let (r, s) = key_pair
            .inner
            .split_rs(&signature::ECDSA_SM2P256_SM3_ASN1, &signature)
            .map_err(|_| Error::SignError)?;
        let mut sig_bytes = [0u8; SIGNATURE_BYTES_LEN];
        let r_bytes = r.as_slice_less_safe();
        let s_bytes = s.as_slice_less_safe();
        sig_bytes[32 - r_bytes.len()..32].copy_from_slice(&r_bytes);
        sig_bytes[64 - s_bytes.len()..64].copy_from_slice(&s_bytes);
        sig_bytes[64..].copy_from_slice(key_pair.pubkey());
        Ok(Signature(sig_bytes))
    }

    fn recover(&self, message: &Message) -> Result<Self::PubKey, Error> {
        let mut pubkey = [0u8; 65];
        pubkey[0] = 4;
        pubkey[1..].copy_from_slice(self.pk());

        let pk =
            signature::UnparsedPublicKey::new(&signature::ECDSA_SM2P256_SM3_ASN1, pubkey.as_ref());

        let r = self.r();
        let s = self.s();
        let signature = EcdsaKeyPair::format_rs(&ECDSA_SM2P256_SM3_ASN1_SIGNING, r, s)
            .map_err(|_| Error::RecoverError)?;

        if pk.verify(&message, signature.as_ref()).is_ok() {
            Ok(PubKey::from(self.pk()))
        } else {
            Err(Error::RecoverError)
        }
    }

    fn verify_public(&self, pubkey: &Self::PubKey, message: &Self::Message) -> Result<bool, Error> {
        let mut pubkey_sig = [0u8; 65];
        pubkey_sig[0] = 4;
        pubkey_sig[1..].copy_from_slice(self.pk());

        let pk = signature::UnparsedPublicKey::new(
            &signature::ECDSA_SM2P256_SM3_ASN1,
            pubkey_sig.as_ref(),
        );

        let r = self.r();
        let s = self.s();
        let signature = EcdsaKeyPair::format_rs(&ECDSA_SM2P256_SM3_ASN1_SIGNING, r, s)
            .map_err(|_| Error::SignError)?;

        if PubKey::from(self.pk()) == *pubkey {
            if pk.verify(&message, signature.as_ref()).is_ok() {
                Ok(true)
            } else {
                Ok(false)
            }
        } else {
            Ok(false)
        }
    }

    fn verify_address(&self, address: &Address, message: &Self::Message) -> Result<bool, Error> {
        let pubkey = self.recover(message)?;
        let recover_address = pubkey_to_address(&pubkey);
        Ok(address == &recover_address)
    }
}

#[cfg(test)]
mod tests {
    use super::{Message, Signature};
    use crate::keypair::KeyPair;
    use crate::PrivKey;
    use cita_crypto_trait::{CreateKey, Sign};
    use hashable::Hashable;

    #[test]
    fn test_sign_verify() {
        let keypair = KeyPair::gen_keypair();
        let msg = Message::default();
        let sig = Signature::sign(keypair.privkey(), &msg).unwrap();
        assert!(sig.verify_public(keypair.pubkey(), &msg).unwrap());
    }

    #[test]
    fn test_verify_address() {
        let keypair = KeyPair::gen_keypair();
        let msg = Message::default();
        let sig = Signature::sign(keypair.privkey(), &msg).unwrap();
        assert_eq!(keypair.pubkey(), &sig.recover(&msg).unwrap());
    }

    #[test]
    fn test_recover() {
        let keypair = KeyPair::gen_keypair();
        let msg = Message::default();
        let sig = Signature::sign(keypair.privkey(), &msg).unwrap();
        assert_eq!(keypair.pubkey(), &sig.recover(&msg).unwrap());
    }

    #[test]
    fn test_into_slice() {
        let keypair = KeyPair::gen_keypair();
        let msg = Message::default();
        let sig = Signature::sign(keypair.privkey(), &msg).unwrap();
        let sig = &sig;
        let slice: &[u8] = sig.into();
        assert_eq!(Signature::from(slice), *sig);
    }

    #[test]
    fn test_sign_ring_libsm() {
        let pri = hex::decode("fffffc4d0000064efffffb8c00000324fffffdc600000543fffff8950000053b")
            .unwrap();
        let keypair = KeyPair::from_privkey(PrivKey::from(pri.as_slice())).unwrap();
        let pk = keypair.pubkey();
        println!("{}", hex::encode(&pk));

        let mut pubkey = [0u8; 65];
        pubkey[0] = 4;
        pubkey[1..].copy_from_slice(pk);
        let message = "hello world".as_bytes();

        let message = message.crypt_hash();
        println!("real msg: {}", hex::encode(&message));

        let sig = Signature::sign(keypair.privkey(), &message).unwrap();
        println!("sig: {}", hex::encode(sig.as_ref()));
    }
}
