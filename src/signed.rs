/*!
Support for encoding signed integers as `vu64`.
*/
use crate::{Error, Vu64};

/// Encode a signed integer as a zigzag-encoded `vint64`.
#[inline]
pub fn encode(value: i64) -> Vu64 {
    value.into()
}

/// Decode a zigzag-encoded `vint64` as a signed integer.
#[inline]
pub fn decode(input: &mut &[u8]) -> Result<i64, Error> {
    super::decode(input).map(zigzag::decode)
}

/// Get the length of a zigzag encoded `vint64` for the given value in bytes.
#[inline]
pub fn encoded_len(value: i64) -> u8 {
    super::encoded_len(zigzag::encode(value))
}

/// Zigzag encoding for signed integers.
///
/// This module contains the raw zigzag encoding algorithm.
///
/// For encoding signed integers as `vu64`, use the functions located in
/// the parent [`vu64::signed`](../index.html) module.
///
/// Reference: [](https://developers.google.com/protocol-buffers/docs/encoding#types)
///
pub mod zigzag {
    /// Encode a signed 64-bit integer in zigzag encoding
    #[inline]
    pub fn encode(value: i64) -> u64 {
        ((value << 1) ^ (value >> 63)) as u64
    }

    /// Decode a signed 64-bit integer from zigzag encoding
    #[inline]
    pub fn decode(encoded: u64) -> i64 {
        (encoded >> 1) as i64 ^ -((encoded & 1) as i64)
    }
}

#[cfg(test)]
mod test_i64 {
    //use super::super::{decode, encode, encoded_len, signed};
    use super::super::signed;
    //use super::{MAX_LEN1, MAX_LEN2, MAX_LEN3, MAX_LEN4, MAX_LEN5, MAX_LEN6, MAX_LEN7, MAX_LEN8};
    #[test]
    fn encode_signed_values() {
        let val = 0x0f0f_f0f0;
        let slice = [0xF0, 0x3C, 0xFC, 0xC3, 0x03].as_ref();
        assert_eq!(signed::encode(val).as_ref(), slice);

        let val = -0x0f0f_f0f0;
        let slice = [0xF7, 0x3B, 0xFC, 0xC3, 0x03].as_ref();
        assert_eq!(signed::encode(val).as_ref(), slice);
    }
    #[test]
    fn decode_signed_values() {
        let mut slice = [0xF0, 0x3C, 0xFC, 0xC3, 0x03].as_ref();
        assert_eq!(signed::decode(&mut slice).unwrap(), 0x0f0f_f0f0);

        let mut slice = [0xF7, 0x3B, 0xFC, 0xC3, 0x03].as_ref();
        assert_eq!(signed::decode(&mut slice).unwrap(), -0x0f0f_f0f0);
    }
}
