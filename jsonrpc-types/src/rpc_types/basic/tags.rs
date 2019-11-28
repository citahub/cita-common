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

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub enum BlockTag {
    #[serde(rename = "latest")]
    Latest,
    #[serde(rename = "earliest")]
    Earliest,
    #[serde(rename = "pending")]
    Pending,
}

/// Economical Model
enum_number!(
    EconomicalModel {
        Quota = 0,
        Charge = 1,
    }
);

#[cfg(test)]
mod tests_tags {
    use super::{BlockTag, EconomicalModel};
    use serde_json;

    #[test]
    fn blocktag_serialize() {
        let serialized = serde_json::to_string(&BlockTag::Latest).unwrap();
        assert_eq!(serialized, r#""latest""#);
        let serialized = serde_json::to_string(&BlockTag::Earliest).unwrap();
        assert_eq!(serialized, r#""earliest""#);
        let serialized = serde_json::to_string(&BlockTag::Pending).unwrap();
        assert_eq!(serialized, r#""pending""#);
    }

    #[test]
    fn blocktag_deserialize() {
        let testdata = vec![
            (r#""#, None),
            (r#""""#, None),
            (r#"latest"#, None),
            (r#"earliest"#, None),
            (r#""latest ""#, None),
            (r#"" latest""#, None),
            (r#""lat est""#, None),
            (r#""Latest""#, None),
            (r#""Pending""#, None),
            (r#""Earliest""#, None),
            (r#""LATEST""#, None),
            (r#""EARLIEST""#, None),
            (r#""latest""#, Some(BlockTag::Latest)),
            (r#""earliest""#, Some(BlockTag::Earliest)),
            (r#""pending""#, Some(BlockTag::Pending)),
        ];
        for (data, expected_opt) in testdata.into_iter() {
            let result: Result<BlockTag, serde_json::Error> = serde_json::from_str(data);
            if let Some(expected) = expected_opt {
                assert_eq!(result.unwrap(), expected);
            } else {
                assert!(result.is_err());
            }
        }
    }

    #[test]
    fn economicalmodel_serialize() {
        let serialized = serde_json::to_string(&EconomicalModel::Quota).unwrap();
        assert_eq!(serialized, r#"0"#);
        let serialized = serde_json::to_string(&EconomicalModel::Charge).unwrap();
        assert_eq!(serialized, r#"1"#);
    }

    #[test]
    fn economicalmodel_deserialize() {
        let testdata = vec![
            (r#""#, None),
            (r#""""#, None),
            (r#"quota"#, None),
            (r#"charge"#, None),
            (r#"-1"#, None),
            (r#"2"#, None),
            (r#""0""#, None),
            (r#""1""#, None),
            (r#"0"#, Some(EconomicalModel::Quota)),
            (r#"1"#, Some(EconomicalModel::Charge)),
        ];
        for (data, expected_opt) in testdata.into_iter() {
            let result: Result<EconomicalModel, serde_json::Error> = serde_json::from_str(data);
            if let Some(expected) = expected_opt {
                assert_eq!(result.unwrap(), expected);
            } else {
                assert!(result.is_err());
            }
        }
    }
}
