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
