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

use super::{Address, KeyPair, PrivKey};
use cita_crypto_trait::CreateKey;

#[derive(Default)]
pub struct Signer {
    pub keypair: KeyPair,
    pub address: Address,
}

impl From<PrivKey> for Signer {
    fn from(privkey: PrivKey) -> Self {
        let keypair = KeyPair::from_privkey(privkey).unwrap();
        Signer {
            address: keypair.address(),
            keypair,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cita_crypto_trait::CreateKey;

    #[test]
    fn test_signer() {
        let keypair = KeyPair::gen_keypair();
        let signer = Signer::from(keypair.privkey().clone());
        assert_eq!(signer.keypair.privkey(), keypair.privkey());
        assert_eq!(signer.keypair.pubkey(), keypair.pubkey());
        assert_eq!(signer.address, keypair.address());
    }
}
