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

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SoftwareVersion {
    #[serde(rename = "softwareVersion")]
    pub software_version: String,
}

impl SoftwareVersion {
    pub fn new(version: String) -> SoftwareVersion {
        SoftwareVersion {
            software_version: version,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SoftwareVersion;
    use serde_json;

    #[test]
    fn software_version_serialization() {
        let value = json!({
        "softwareVersion": "0.22.0"
        });
        let software_version = SoftwareVersion::new("0.22.0".to_owned());
        assert_eq!(serde_json::to_value(software_version).unwrap(), value);
    }
}
