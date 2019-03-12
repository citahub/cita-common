// Copyright 2015-2017 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// This software is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This software is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

//! Semantic version formatting and comparing.

/// A version value with strict meaning. Use `as_u32` to convert to a simple integer.
///
/// # Example
/// ```
/// extern crate util;
/// use util::semantic_version::*;
///
/// fn main() {
///   assert_eq!(SemanticVersion::new(1, 2, 3).as_u32(), 0x010203);
/// }
/// ```
pub struct SemanticVersion {
    /// Major version - API/feature removals & breaking changes.
    pub major: u8,
    /// Minor version - API/feature additions.
    pub minor: u8,
    /// Tiny version - bug fixes.
    pub tiny: u8,
}

impl SemanticVersion {
    /// Create a new object.
    pub fn new(major: u8, minor: u8, tiny: u8) -> SemanticVersion {
        SemanticVersion { major, minor, tiny }
    }

    /// Convert to a `u32` representation.
    pub fn as_u32(&self) -> u32 {
        (u32::from(self.major) << 16) + (u32::from(self.minor) << 8) + u32::from(self.tiny)
    }
}

// TODO: implement Eq, Comparison and Debug/Display for SemanticVersion.
