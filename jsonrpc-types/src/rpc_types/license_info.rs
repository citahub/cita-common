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

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct LicenseInfo {
    #[serde(rename = "licenseType")]
    pub license_type: String,

    #[serde(rename = "fingerPrint")]
    pub finger_print: Option<String>,

    #[serde(rename = "expirationDate")]
    pub expiration_date: Option<String>,
    pub issuer: Option<String>,

    #[serde(rename = "errorMessage")]
    pub error_message: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::LicenseInfo;
    use serde_json;

    #[test]
    fn peers_info_serialization_without_error_msg() {
        let value = json!({
            "licenseType": "Free Trial",
            "fingerPrint": "0x1c29a8b5a8b5c0fb2f8d06fb70796ec4af6c70f6c6fbee9df922e6f45148e211",
            "expirationDate": "2020-03-05T14:24:19Z",
            "issuer": "0xd4565c152aa8f786920846e49f08782e161e3254",
            "errorMessage": serde_json::Value::Null,
        });

        let peers_info = LicenseInfo {
            license_type: "Free Trial".to_owned(),
            finger_print: Some(
                "0x1c29a8b5a8b5c0fb2f8d06fb70796ec4af6c70f6c6fbee9df922e6f45148e211".to_owned(),
            ),
            expiration_date: Some("2020-03-05T14:24:19Z".to_owned()),
            issuer: Some("0xd4565c152aa8f786920846e49f08782e161e3254".to_owned()),
            error_message: None,
        };

        assert_eq!(serde_json::to_value(peers_info).unwrap(), value);
    }

    #[test]
    fn peers_info_serialization_with_error_msg() {
        let value = json!({
            "licenseType": "Free Trial",
            "fingerPrint": serde_json::Value::Null,
            "expirationDate": serde_json::Value::Null,
            "issuer": serde_json::Value::Null,
            "errorMessage": "Time out".to_owned(),
        });

        let peers_info = LicenseInfo {
            license_type: "Free Trial".to_owned(),
            finger_print: None,
            expiration_date: None,
            issuer: None,
            error_message: Some("Time out".to_owned()),
        };

        assert_eq!(serde_json::to_value(peers_info).unwrap(), value);
    }
}
