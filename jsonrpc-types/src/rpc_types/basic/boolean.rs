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

use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// A unsigned integer (wrapper structure around bool).
#[derive(Debug, PartialEq, Eq, Default, Hash, Clone)]
pub struct Boolean(bool);

impl Boolean {
    pub fn new(data: bool) -> Boolean {
        Boolean(data)
    }
}

impl Serialize for Boolean {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bool(self.0)
    }
}

impl<'de> Deserialize<'de> for Boolean {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_bool(BooleanVisitor)
    }
}

struct BooleanVisitor;

impl<'de> Visitor<'de> for BooleanVisitor {
    type Value = Boolean;

    fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        formatter.write_str("Boolean")
    }

    fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Boolean::new(value))
    }
}

impl From<bool> for Boolean {
    fn from(data: bool) -> Boolean {
        Boolean::new(data)
    }
}

impl Into<bool> for Boolean {
    fn into(self) -> bool {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::Boolean;
    use serde_json;

    #[test]
    fn serialize() {
        let data = Boolean::new(true);
        let serialized = serde_json::to_string(&data).unwrap();
        assert_eq!(serialized, r#"true"#);
        let data = Boolean::new(false);
        let serialized = serde_json::to_string(&data).unwrap();
        assert_eq!(serialized, r#"false"#);
    }

    #[test]
    fn deserialize() {
        let testdata = vec![
            (r#""""#, None),
            (r#""0""#, None),
            (r#""10""#, None),
            (r#""#, None),
            (r#"a"#, None),
            (r#""True""#, None),
            (r#""False""#, None),
            (r#""TRUE""#, None),
            (r#""FALSE""#, None),
            (r#"true"#, Some(true)),
            (r#"false"#, Some(false)),
        ];
        for (data, expected_opt) in testdata.into_iter() {
            let result: Result<Boolean, serde_json::Error> = serde_json::from_str(data);
            if let Some(expected) = expected_opt {
                assert_eq!(result.unwrap(), Boolean::new(expected));
            } else {
                assert!(result.is_err());
            }
        }
    }
}
