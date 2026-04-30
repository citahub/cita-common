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

use cita_types::{H160, H256, U256};

use crate::rpc_types::{BlockTag, Boolean, Data, Data20, Data32, Integer, Quantity};

macro_rules! test_from_jsonstr {
    ($type:tt, $jsonstr:expr, $expected_opt:expr) => {
        let result = $type::from_jsonstr($jsonstr);
        if let Some(expected) = $expected_opt {
            assert_eq!(result.unwrap(), expected);
        } else {
            assert!(result.is_err());
        }
    };
}

#[allow(clippy::cognitive_complexity)]
#[test]
fn from_jsonstr() {
    test_from_jsonstr!(BlockTag, r#""latest""#, Some(BlockTag::Latest));
    test_from_jsonstr!(BlockTag, r#""earliest""#, Some(BlockTag::Earliest));
    test_from_jsonstr!(Boolean, r#"true"#, Some(Boolean::new(true)));
    test_from_jsonstr!(Boolean, r#"false"#, Some(Boolean::new(false)));
    test_from_jsonstr!(
        Data,
        r#""0xabcdef""#,
        Some(Data::new(vec![0xab, 0xcd, 0xef]))
    );
    let auint = 11_259_375u64;
    test_from_jsonstr!(
        Data20,
        format!(r#""{:#042x}""#, auint).as_ref(),
        Some(Data20::new(H160::from_low_u64_be(auint)))
    );
    test_from_jsonstr!(
        Data32,
        format!(r#""{:#066x}""#, auint).as_ref(),
        Some(Data32::new(H256::from_low_u64_be(auint)))
    );
    test_from_jsonstr!(
        Integer,
        format!(r#"{}"#, auint).as_ref(),
        Some(Integer::new(auint))
    );
    test_from_jsonstr!(
        Quantity,
        format!(r#""{:#x}""#, auint).as_ref(),
        Some(Quantity::new(U256::from(auint)))
    );
}
