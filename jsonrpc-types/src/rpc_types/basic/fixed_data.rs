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

use std::str::FromStr;

use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use cita_types::traits::LowerHex;
use cita_types::{H160, H256};

/// Fixed length bytes (wrapper structure around H256).
#[derive(Debug, PartialEq, Eq, Default, Hash, Clone)]
pub struct Data32(H256);

/// Fixed length bytes (wrapper structure around H160).
#[derive(Debug, PartialEq, Eq, Default, Hash, Clone)]
pub struct Data20(H160);

struct Data32Visitor;
struct Data20Visitor;

macro_rules! impl_for_fixed_type {
    ($outer:ident, $inner:ident, $outer_size:expr, $visitor:ident) => {
        impl $outer {
            pub fn new(data: $inner) -> $outer {
                $outer(data)
            }
        }

        impl Serialize for $outer {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                serializer.serialize_str(self.0.lower_hex_with_0x().as_ref())
            }
        }

        impl<'de> Deserialize<'de> for $outer {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                deserializer.deserialize_str($visitor)
            }
        }

        impl<'de> Visitor<'de> for $visitor {
            type Value = $outer;

            fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                formatter.write_str(stringify!($outtype))
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if value.len() == 2 + $outer_size * 2
                    && (&value[0..2] == "0x" || &value[0..2] == "0X")
                {
                    let data = $inner::from_str(&value[2..]).map_err(|_| {
                        if value.len() > 12 {
                            E::custom(format!(
                                "invalid hexadecimal string: [{}..(omit {})..{}]",
                                &value[..6],
                                value.len() - 12,
                                &value[value.len() - 6..value.len()]
                            ))
                        } else {
                            E::custom(format!("invalid hexadecimal string: [{}]", value))
                        }
                    })?;
                    Ok($outer::new(data))
                } else {
                    if value.len() > 12 {
                        Err(E::custom(format!(
                            "invalid format: [{}..(omit {})..{}]",
                            &value[..6],
                            value.len() - 12,
                            &value[value.len() - 6..value.len()]
                        )))
                    } else {
                        Err(E::custom(format!("invalid format: [{}]", value)))
                    }
                }
            }

            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                self.visit_str(value.as_ref())
            }
        }

        impl From<$inner> for $outer {
            fn from(data: $inner) -> $outer {
                $outer::new(data)
            }
        }

        impl Into<$inner> for $outer {
            fn into(self) -> $inner {
                self.0
            }
        }

        impl Into<Vec<u8>> for $outer {
            fn into(self) -> Vec<u8> {
                self.0.to_vec()
            }
        }
    };
}

impl_for_fixed_type!(Data32, H256, 32usize, Data32Visitor);
impl_for_fixed_type!(Data20, H160, 20usize, Data20Visitor);

macro_rules! test_for_fixed_type {
    ($test_name:ident, $outer:ident, $inner:ident, $outer_size:expr) => {
        #[cfg(test)]
        mod $test_name {

            use super::$outer;
            use cita_types::{pad_left0, $inner};
            use serde_json;
            use std::str::FromStr;

            #[test]
            fn serialize() {
                let testdata = vec!["0", "a", "123456789abcdef0"];
                for data in testdata.into_iter() {
                    let padded = pad_left0(data, $outer_size * 2);
                    let data = $outer::new($inner::from_str(&padded).unwrap());
                    let serialized = serde_json::to_string(&data).unwrap();
                    assert_eq!(serialized, format!(r#""0x{}""#, padded));
                }
            }

            #[test]
            fn deserialize() {
                let result: Result<$outer, serde_json::Error> = serde_json::from_str("");
                assert!(result.is_err());

                for sz in vec![
                    $outer_size * 2 - 2,
                    $outer_size * 2 - 1,
                    $outer_size * 2 + 1,
                    $outer_size * 2 + 2,
                ]
                .into_iter()
                {
                    let data = format!(r#""0x{}""#, pad_left0("0", sz));
                    let result: Result<$outer, serde_json::Error> = serde_json::from_str(&data);
                    assert!(result.is_err());
                }

                let testdata = vec![
                    ("g", None),
                    ("a", Some($outer::new($inner::from(10)))),
                    ("10", Some($outer::new($inner::from(16)))),
                    ("abcdef", Some($outer::new($inner::from(11259375)))),
                    ("ABCDEF", Some($outer::new($inner::from(11259375)))),
                ];
                for (data, expected_opt) in testdata.into_iter() {
                    let padded = format!(r#""0x{}""#, pad_left0(data, $outer_size * 2));
                    let result1: Result<$outer, serde_json::Error> = serde_json::from_str(&padded);
                    let padded = format!(r#""0X{}""#, pad_left0(data, $outer_size * 2));
                    let result2: Result<$outer, serde_json::Error> = serde_json::from_str(&padded);
                    if let Some(expected) = expected_opt {
                        assert_eq!(result1.unwrap(), expected);
                        assert_eq!(result2.unwrap(), expected);
                    } else {
                        assert!(result1.is_err());
                        assert!(result2.is_err());
                    }
                }
            }
        }
    };
}

test_for_fixed_type!(tests_data32, Data32, H256, 32usize);
test_for_fixed_type!(tests_data20, Data20, H160, 20usize);
