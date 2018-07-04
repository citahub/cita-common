// CITA
// Copyright 2016-2018 Cryptape Technologies LLC.

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

use std::str::FromStr;

use super::{clean_0x, delete_left0, pad_left0};
use super::{Bloom, BloomInput};
use super::{H1024, H128, H160, H256, H264, H32, H512, H520, H64};
use super::{U1024, U128, U256, U512, U64};

pub trait LowerHex {
    fn lower_hex(&self) -> String;
    fn lower_hex_with_0x(&self) -> String;
}

pub trait ConvertType
where
    Self: ::std::marker::Sized,
{
    fn from_unaligned(&str) -> Option<Self>;
}

fn convert_u8_to_char(u: u8) -> char {
    match u {
        u if u < 10 => ('0' as u8 + u) as char,
        u if 10 <= u && u < 16 => ('a' as u8 + (u - 10)) as char,
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
                format!("{:#x}", self)
            }
        }

        impl ConvertType for $name {
            #[inline]
            fn from_unaligned<'a>(s: &'a str) -> Option<Self> {
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
    (H1024, 1024),
    (U64, 64),
    (U128, 128),
    (U256, 256),
    (U512, 512),
    (U1024, 1024),
]);

pub trait BloomTools {
    fn from_raw<'a>(&'a [u8]) -> Self;
    fn contains_raw<'a>(&self, &'a [u8]) -> bool;
    fn accrue_raw<'a>(&mut self, &'a [u8]);
}

impl BloomTools for Bloom {
    fn from_raw<'a>(raw: &'a [u8]) -> Bloom {
        Bloom::from(BloomInput::Raw(raw))
    }

    fn contains_raw<'a>(&self, raw: &'a [u8]) -> bool {
        self.contains_input(BloomInput::Raw(raw))
    }

    fn accrue_raw<'a>(&mut self, raw: &'a [u8]) {
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
