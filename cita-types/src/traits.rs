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

use std::str::FromStr;

use super::{clean_0x, delete_left0, pad_left0};
use super::{Bloom, BloomInput};
use super::{H128, H160, H256, H264, H32, H512, H520, H64};
use super::{U128, U256, U512, U64};

pub trait LowerHex {
    fn lower_hex(&self) -> String;
    fn lower_hex_with_0x(&self) -> String;
}

pub trait ConvertType
where
    Self: ::std::marker::Sized,
{
    fn from_unaligned(_: &str) -> Option<Self>;
}

fn convert_u8_to_char(u: u8) -> char {
    match u {
        u if u < 10 => (b'0' + u) as char,
        u if (10..16).contains(&u) => (b'a' + (u - 10)) as char,
        _ => panic!("{} not bwtween 0 and 15", u),
    }
}

impl<'a> LowerHex for &'a [u8] {
    fn lower_hex(&self) -> String {
        let sz = self.len();
        let mut s = vec![0u8 as char; sz * 2];
        for i in 0..sz {
            s[i * 2] = convert_u8_to_char(self[i] / 16);
            s[i * 2 + 1] = convert_u8_to_char(self[i] % 16);
        }
        s.into_iter().collect()
    }

    fn lower_hex_with_0x(&self) -> String {
        let sz = self.len();
        let mut s = vec![0u8 as char; sz * 2 + 2];
        s[0] = '0';
        s[1] = 'x';
        for i in 0..sz {
            s[i * 2 + 2] = convert_u8_to_char(self[i] / 16);
            s[i * 2 + 3] = convert_u8_to_char(self[i] % 16);
        }
        s.into_iter().collect()
    }
}

impl LowerHex for Vec<u8> {
    fn lower_hex(&self) -> String {
        let s = self as &[u8];
        s.lower_hex()
    }
    fn lower_hex_with_0x(&self) -> String {
        let s = self as &[u8];
        s.lower_hex_with_0x()
    }
}

macro_rules! add_cita_funcs {
    ([$( ($name:ident, $bits:expr) ),+ ,]) => {
        add_cita_funcs!([ $( ($name, $bits) ),+ ]);
    };
    ([$( ($name:ident, $bits:expr) ),+]) => {
        $( add_cita_funcs!($name, $bits); )+
    };
    ($name:ident, $bits:expr) => {
        impl LowerHex for $name {
            #[inline]
            fn lower_hex(&self) -> String {
                format!("{:x}", self)
            }

            #[inline]
            fn lower_hex_with_0x(&self) -> String {
                // TODO: https://github.com/paritytech/primitives/pull/37
                format!("0x{:x}", self)
            }
        }

        impl ConvertType for $name {
            #[inline]
            fn from_unaligned(s: &str) -> Option<Self> {
                $name::from_str(
                    pad_left0(
                        delete_left0(clean_0x(s)), $bits/4usize).as_str())
                    .ok()
            }
        }
    }
}

add_cita_funcs!([
    (Bloom, 2048),
    (H32, 32),
    (H64, 64),
    (H128, 128),
    (H160, 160),
    (H256, 256),
    (H264, 264),
    (H512, 512),
    (H520, 520),
    (U64, 64),
    (U128, 128),
    (U256, 256),
    (U512, 512),
]);

pub trait BloomTools {
    fn from_raw(_: &[u8]) -> Self;
    fn contains_raw(&self, _: &[u8]) -> bool;
    fn accrue_raw(&mut self, _: &[u8]);
}

impl BloomTools for Bloom {
    fn from_raw(raw: &[u8]) -> Bloom {
        Bloom::from(BloomInput::Raw(raw))
    }

    fn contains_raw(&self, raw: &[u8]) -> bool {
        self.contains_input(BloomInput::Raw(raw))
    }

    fn accrue_raw(&mut self, raw: &[u8]) {
        self.accrue(BloomInput::Raw(raw));
    }
}

#[cfg(test)]
mod tests {
    use super::convert_u8_to_char;
    use super::U128;
    use super::{ConvertType, LowerHex};

    #[test]
    fn test_convert_u8_to_char() {
        let mut x = vec![0u8 as char; 2];
        for i in 0..256 {
            {
                let y = x.as_mut_slice();
                y[0] = convert_u8_to_char(i as u8 / 16);
                y[1] = convert_u8_to_char(i as u8 % 16);
            }
            let s: String = x.iter().collect();
            assert_eq!(format!("{:02x}", i), s);
        }
    }

    #[test]
    fn test_lower_hex_for_u8() {
        let x = vec![0u8, 127, 255];
        let x_str = "007fff";
        assert_eq!(x.lower_hex(), format!("{}", x_str));
        assert_eq!(x.lower_hex_with_0x(), format!("0x{}", x_str));
    }

    #[test]
    fn test_lower_hex_for_uint_and_hash() {
        let validation = vec![
            (0u64, "0"),
            (10u64, "a"),
            (2432902008176639999u64, "21c3677c82b3ffff"),
        ];
        for (x_uint, x_str) in validation.into_iter() {
            assert_eq!(U128::from(x_uint).lower_hex(), format!("{}", x_str));
            assert_eq!(
                U128::from(x_uint).lower_hex_with_0x(),
                format!("0x{}", x_str)
            );
        }
    }

    #[test]
    fn test_from_unaligned() {
        let validation = vec![
            (2432902008176639999u64, "0x21c3677c82b3ffff"),
            (2432902008176639999u64, "21c3677c82b3ffff"),
            (2432902008176639999u64, "00000000021c3677c82b3ffff"),
            (
                2432902008176639999u64,
                "000000000000000000000000000000000000000021c3677c82b3ffff",
            ),
        ];
        for (i, s) in validation.into_iter() {
            assert_eq!(U128::from(i), U128::from_unaligned(s).unwrap())
        }
    }
}
