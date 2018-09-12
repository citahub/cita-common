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

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub enum BlockTag {
    #[serde(rename = "latest")]
    Latest,
    #[serde(rename = "earliest")]
    Earliest,
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
            (r#""Earliest""#, None),
            (r#""LATEST""#, None),
            (r#""EARLIEST""#, None),
            (r#""latest""#, Some(BlockTag::Latest)),
            (r#""earliest""#, Some(BlockTag::Earliest)),
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
