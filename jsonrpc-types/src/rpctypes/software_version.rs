// CITA
// Copyright 2016-2019 Cryptape Technologies LLC.

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
