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

/// Semantic Versioning Specification (SemVer)
/// Check the detail about it: https://semver.org/spec/v2.0.0.html
pub struct SemVer {
    pub major: u8,
    pub minor: u8,
    pub patch: u8,
}

impl SemVer {
    pub fn new(major: u8, minor: u8, patch: u8) -> Self {
        SemVer {
            major,
            minor,
            patch,
        }
    }
}

impl Default for SemVer {
    /// Default: `v0.1.0`
    fn default() -> Self {
        SemVer {
            major: 0,
            minor: 1,
            patch: 0,
        }
    }
}
