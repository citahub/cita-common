// Copyright 2015-2017 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// This software is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This software is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.
#![rustfmt_skip]

use super::{KeyPair, Generator, Secret};
use keccak::Keccak256;

/// Simple brainwallet.
pub struct Brain(String);

impl Brain {
    pub fn new(s: String) -> Self {
        Brain(s)
    }
}

impl Generator for Brain {
    type Error = ::Void;

    fn generate(self) -> Result<KeyPair, Self::Error> {
        let seed = self.0;
        let mut secret = seed.into_bytes().keccak256();

        let mut i = 0;
        loop {
            secret = secret.keccak256();

            match i > 16384 {
                false => i += 1,
                true => {
                    if let Ok(pair) = Secret::from_unsafe_slice(&secret).and_then(KeyPair::from_secret) {
                        if pair.address()[0] == 0 {
                            return Ok(pair);
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use {Brain, Generator};

    #[test]
    fn test_brain() {
        let words = "this is sparta!".to_owned();
        let first_keypair = Brain::new(words.clone()).generate().unwrap();
        let second_keypair = Brain::new(words.clone()).generate().unwrap();
        assert_eq!(first_keypair.secret(), second_keypair.secret());
    }
}
