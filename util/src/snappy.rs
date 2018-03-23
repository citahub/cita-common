// CITA
// Copyright 2016-2017 Cryptape Technologies LLC.

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

use libc::{c_char, c_int, size_t};
use std::convert::{From, Into};

pub const CITA_SKIP_COMPRESS_SIZE: usize = 40 * 1024;

// https://github.com/google/snappy/blob/ca37ab7/snappy-c.h#L46-L50
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SnappyError {
    InvalidInput,
    BufferTooSmall,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SnappyStatus {
    Ok = 0,
    InvalidInput = 1,
    BufferTooSmall = 2,
    Unknown = !0,
}

impl ::std::fmt::Display for SnappyError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ::std::fmt::Display for SnappyStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<c_int> for SnappyStatus {
    fn from(s: c_int) -> SnappyStatus {
        match s {
            0 => SnappyStatus::Ok,
            1 => SnappyStatus::InvalidInput,
            2 => SnappyStatus::BufferTooSmall,
            _ => SnappyStatus::Unknown,
        }
    }
}

impl From<SnappyStatus> for SnappyError {
    fn from(s: SnappyStatus) -> SnappyError {
        match s {
            SnappyStatus::InvalidInput => SnappyError::InvalidInput,
            SnappyStatus::BufferTooSmall => SnappyError::BufferTooSmall,
            _ => SnappyError::Unknown,
        }
    }
}

#[link(name = "snappy")]
extern "C" {
    fn snappy_compress(
        input: *const c_char,
        input_len: size_t,
        compressed: *mut c_char,
        compressed_len: *mut size_t,
    ) -> c_int;

    fn snappy_max_compressed_length(source_len: size_t) -> size_t;

    fn snappy_uncompress(
        compressed: *const c_char,
        compressed_len: size_t,
        uncompressed: *mut c_char,
        uncompressed_len: *mut size_t,
    ) -> c_int;

    fn snappy_uncompressed_length(compressed: *const c_char, compressed_len: size_t, result: *mut size_t) -> c_int;
}

/// Compress a buffer using snappy, write the result append to
/// the given output buffer, growing it if necessary.
/// Returns the length of the compressed data.
/// Otherwise, raise an error if compress failed.
pub fn compress_to(input: &[u8], output: &mut Vec<u8>) -> Result<usize, SnappyError> {
    let input_len = input.len();
    let output_len = output.len();
    // Try to get the maximum compressed length possibly.
    let mut compressed_len: size_t = unsafe { snappy_max_compressed_length(input_len as size_t) };
    // Reserves capacity for compressed data to be inserted.
    output.reserve(compressed_len as usize);

    let status = unsafe {
        snappy_compress(
            input.as_ptr() as *const c_char,
            input.len() as size_t,
            output[output_len..].as_mut_ptr() as *mut c_char,
            &mut compressed_len,
        )
    };
    // Since we call snappy_max_compressed_length before snappy_compress,
    // the return status should not be SNAPPY_BUFFER_TOO_SMALL.
    match status.into() {
        SnappyStatus::Ok => {
            unsafe { output.set_len(output_len + compressed_len) };
            Ok(compressed_len as usize)
        }
        s @ _ => {
            unsafe { output.set_len(output_len) };
            Err(s.into())
        }
    }
}

/// Decompress a buffer using snappy, write the result append to
/// the given output buffer, growing it if necessary.
/// Returns the length of the decompressed data.
/// Otherwise, raise an error if uncompress failed.
pub fn decompress_to(input: &[u8], output: &mut Vec<u8>) -> Result<usize, SnappyError> {
    let input_len = input.len();
    let output_len = output.len();
    let mut uncompressed_len: size_t = 0;
    // Try to get the uncompressed length
    let status = unsafe {
        snappy_uncompressed_length(
            input.as_ptr() as *const c_char,
            input_len as size_t,
            &mut uncompressed_len,
        )
    };
    let s: SnappyStatus = status.into();
    if s != SnappyStatus::Ok {
        return Err(s.into());
    }
    // Reserves capacity for uncompressed data to be inserted.
    output.reserve(uncompressed_len as usize);
    // Uncompress.
    let status = unsafe {
        snappy_uncompress(
            input.as_ptr() as *const c_char,
            input.len() as size_t,
            output[output_len..].as_mut_ptr() as *mut c_char,
            &mut uncompressed_len,
        )
    };
    match status.into() {
        SnappyStatus::Ok => {
            unsafe { output.set_len(output_len + uncompressed_len) };
            Ok(uncompressed_len as usize)
        }
        s @ _ => {
            unsafe { output.set_len(output_len) };
            Err(s.into())
        }
    }
}

pub fn cita_compress_to(input: &[u8], output: &mut Vec<u8>) -> Result<bool, SnappyError> {
    if input.len() > CITA_SKIP_COMPRESS_SIZE {
        compress_to(input, output).map(|_| true)
    } else {
        Ok(false)
    }
}

pub fn cita_decompress_to(input: &[u8], output: &mut Vec<u8>) -> Result<(), SnappyError> {
    decompress_to(input, output).map(|_| ())
}

#[cfg(test)]
mod tests {
    use super::{cita_compress_to, cita_decompress_to, compress_to, decompress_to};

    #[test]
    fn valid() {
        let d = vec![0xde, 0xad, 0xd0, 0x0d];
        let mut cd: Vec<u8> = Vec::new();
        assert!(compress_to(&d, &mut cd).is_ok());
        let mut dd: Vec<u8> = Vec::new();
        assert!(decompress_to(&cd, &mut dd).is_ok());
        assert!(&d == &dd);
    }

    #[test]
    fn invalid() {
        let v = vec![0, 0, 0, 0];
        let mut d: Vec<u8> = Vec::new();
        assert!(decompress_to(&v, &mut d).is_err());
    }

    #[test]
    fn empty() {
        let v = vec![];
        let mut d: Vec<u8> = Vec::new();
        assert!(decompress_to(&v, &mut d).is_err());
    }

    #[test]
    fn compress_and_decompress() {
        let v = vec![1, 2, 3];
        let mut d = vec![4, 5, 6];
        assert!(compress_to(&v, &mut d).is_ok());
        let u: Vec<_> = d.drain(3..).collect();
        assert_eq!(d, &[4, 5, 6]);
        let mut c = vec![7, 8, 9];
        assert!(decompress_to(&u, &mut c).is_ok());
        let w: Vec<_> = c.drain(3..).collect();
        assert_eq!(c, &[7, 8, 9]);
        assert_eq!(w, v);
    }

    #[test]
    fn decompress_error() {
        let u: Vec<u8> = vec![
            18, 20, 10, 16, 224, 43, 102, 80, 16, 58, 75, 248, 165, 208, 24, 6, 22, 217, 65, 135, 16, 1
        ];
        let mut c: Vec<u8> = Vec::new();
        let mut d: Vec<u8> = Vec::new();
        // no compress
        assert_eq!(cita_compress_to(&u, &mut c), Ok(false));
        // why uncompress
        assert_eq!(cita_decompress_to(&u, &mut d), Ok(()));
        // data is not same
        assert_ne!(d, u);
    }
}
