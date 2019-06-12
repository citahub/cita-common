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
    let auint = 11259375u64;
    test_from_jsonstr!(
        Data20,
        format!(r#""{:#042x}""#, auint).as_ref(),
        Some(Data20::new(H160::from(auint)))
    );
    test_from_jsonstr!(
        Data32,
        format!(r#""{:#066x}""#, auint).as_ref(),
        Some(Data32::new(H256::from(auint)))
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
