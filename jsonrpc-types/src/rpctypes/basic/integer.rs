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

use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::Error;

pub trait TryInto<T> {
    fn try_into(self) -> Result<T, Error>;
}

/// A unsigned integer (wrapper structure around u64).
#[derive(Debug, PartialEq, Eq, Default, Hash, Clone)]
pub struct Integer(u64);

impl Integer {
    pub fn new(data: u64) -> Integer {
        Integer(data)
    }
}

impl Serialize for Integer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(self.0)
    }
}

impl<'de> Deserialize<'de> for Integer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_u64(IntegerVisitor)
    }
}

struct IntegerVisitor;

impl<'de> Visitor<'de> for IntegerVisitor {
    type Value = Integer;

    fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        formatter.write_str("Integer")
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Integer::new(value))
    }
}

impl From<u64> for Integer {
    fn from(data: u64) -> Integer {
        Integer::new(data)
    }
}

impl Into<u64> for Integer {
    fn into(self) -> u64 {
        self.0
    }
}

macro_rules! impl_convert_with_uint {
    ($uint:ident) => {
        impl From<$uint> for Integer {
            fn from(data: $uint) -> Integer {
                Integer::new(data as u64)
            }
        }

        impl TryInto<$uint> for Integer {
            fn try_into(self) -> Result<$uint, Error> {
                if self.0 > u64::from($uint::max_value()) {
                    Err(Error::parse_error_with_message("Integer truncated"))
                } else {
                    Ok(self.0 as $uint)
                }
            }
        }
    };
}

impl_convert_with_uint!(u32);
impl_convert_with_uint!(u16);
impl_convert_with_uint!(u8);

#[cfg(test)]
mod tests {
    use super::{Integer, TryInto};
    use serde_json;
    use std::convert::Into;

    #[test]
    fn serialize() {
        let data = Integer::new(1311768467463790320u64);
        let serialized = serde_json::to_string(&data).unwrap();
        assert_eq!(serialized, r#"1311768467463790320"#);
    }

    #[test]
    fn deserialize() {
        let testdata = vec![
            (r#""""#, None),
            (r#""0""#, None),
            (r#""10""#, None),
            (r#""#, None),
            (r#"a"#, None),
            (r#"0"#, Some(0u64)),
            (r#"10"#, Some(10u64)),
        ];
        for (data, expected_opt) in testdata.into_iter() {
            let result: Result<Integer, serde_json::Error> = serde_json::from_str(data);
            if let Some(expected) = expected_opt {
                assert_eq!(result.unwrap(), Integer::new(expected));
            } else {
                assert!(result.is_err());
            }
        }
    }

    #[test]
    fn convert() {
        let auint: u8 = 123;
        let data = Integer::new(auint as u64);
        assert_eq!(data, (auint as u32).into());
        assert_eq!(data, (auint as u16).into());
        assert_eq!(data, (auint as u8).into());
        let expected: u32 = data.clone().try_into().unwrap();
        assert_eq!(expected, 123);
        let expected: u16 = data.clone().try_into().unwrap();
        assert_eq!(expected, 123);
        let expected: u8 = data.clone().try_into().unwrap();
        assert_eq!(expected, 123);
    }
}
