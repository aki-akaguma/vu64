/*!
The variable length integer encoding of u64.
This is a simple and fast encoder/decoder.

## format pattern

| Prefix     | Precision | Total Bytes |
|------------|-----------|-------------|
| `0xxxxxxx` | 7 bits    | 1 byte      |
| `10xxxxxx` | 14 bits   | 2 bytes     |
| `110xxxxx` | 21 bits   | 3 bytes     |
| `1110xxxx` | 28 bits   | 4 bytes     |
| `11110xxx` | 35 bits   | 5 bytes     |
| `111110xx` | 42 bits   | 6 bytes     |
| `1111110x` | 49 bits   | 7 bytes     |
| `11111110` | 56 bits   | 8 bytes     |
| `11111111` | 64 bits   | 9 bytes     |

This format is a like [`vint64`](https://crates.io/crates/vint64),
but 0x00 is represented by 0x00.

# Examples

## Encode

```rust
use vu64::encode;

assert_eq!(encode(0x0f0f).as_ref(), &[0x8F, 0x3c]);
```

## Decode

```rust
use vu64::decode;

let slice = [0x8F, 0x3c].as_ref();
assert_eq!(decode(slice).unwrap(), 0x0f0f);
```

## Encode and decode

```rust
use vu64::{encode, decode};

let val = 1234;
assert_eq!(decode(encode(val).as_ref()).unwrap(), val);
```

*/
use core::convert::{TryFrom, TryInto};
use core::fmt::{self, Debug, Display};

pub mod signed;

#[cfg(feature = "io")]
pub mod io;

/// Maximun integer whose length of `vu64` is 1 byte.
#[allow(dead_code)]
pub const MAX_LEN1: u64 = 0x7F;

/// Maximun integer whose length of `vu64` is 2 byte.
#[allow(dead_code)]
pub const MAX_LEN2: u64 = 0x3FFF;

/// Maximun integer whose length of `vu64` is 3 byte.
#[allow(dead_code)]
pub const MAX_LEN3: u64 = 0x1F_FFFF;

/// Maximun integer whose length of `vu64` is 4 byte.
#[allow(dead_code)]
pub const MAX_LEN4: u64 = 0x0FFF_FFFF;

/// Maximun integer whose length of `vu64` is 5 byte.
#[allow(dead_code)]
pub const MAX_LEN5: u64 = 0x07_FFFF_FFFF;

/// Maximun integer whose length of `vu64` is 6 byte.
#[allow(dead_code)]
pub const MAX_LEN6: u64 = 0x03FF_FFFF_FFFF;

/// Maximun integer whose length of `vu64` is 7 byte.
#[allow(dead_code)]
pub const MAX_LEN7: u64 = 0x01_FFFF_FFFF_FFFF;

/// Maximun integer whose length of `vu64` is 8 byte.
#[allow(dead_code)]
pub const MAX_LEN8: u64 = 0xFF_FFFF_FFFF_FFFF;

/// Maximun integer whose length of `vu64` is 9 byte.
#[allow(dead_code)]
pub const MAX_LEN9: u64 = core::u64::MAX;

/// Maximum length of a `vu64` in bytes
pub const MAX_BYTES: usize = 9;

/// `vu64`: serialized variable-length 64-bit integers.
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vu64 {
    /// Encoded length in bytes
    length: u8,
    /// Serialized variable-length integer
    bytes: [u8; MAX_BYTES],
}

impl AsRef<[u8]> for Vu64 {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        &self.bytes[..self.length as usize]
    }
}

impl Debug for Vu64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bytes_ref = self.as_ref();
        write!(f, "V64({})", decode(bytes_ref).unwrap())
    }
}

impl From<u64> for Vu64 {
    #[inline]
    fn from(value: u64) -> Vu64 {
        encode(value)
    }
}

impl From<i64> for Vu64 {
    #[inline]
    fn from(value: i64) -> Vu64 {
        signed::zigzag::encode(value).into()
    }
}

impl TryFrom<&[u8]> for Vu64 {
    type Error = Error;

    #[inline]
    fn try_from(slice: &[u8]) -> Result<Self, Error> {
        decode(slice).map(Vu64::from)
    }
}

/// Get the length of an encoded `vu64` for the given value in bytes.
#[inline]
pub fn encoded_len(value: u64) -> u8 {
    let ldz = value.leading_zeros();
    ENCODED_LEN_TBL[ldz as usize]
}

#[rustfmt::skip]
static ENCODED_LEN_TBL: &[u8; 65] = &[
    9,
    9, 9, 9, 9, 9, 9, 9,
    8, 8, 8, 8, 8, 8, 8,
    7, 7, 7, 7, 7, 7, 7,
    6, 6, 6, 6, 6, 6, 6,
    5, 5, 5, 5, 5, 5, 5,
    4, 4, 4, 4, 4, 4, 4,
    3, 3, 3, 3, 3, 3, 3,
    2, 2, 2, 2, 2, 2, 2,
    1, 1, 1, 1, 1, 1, 1,
    1,
];

/*
pub fn encoded_len(value: u64) -> u8 {
    match value.leading_zeros() {
        57..=64 => 1,
        50..=56 => 2,
        43..=49 => 3,
        36..=42 => 4,

        29..=35 => 5,
        22..=28 => 6,
        15..=21 => 7,
        8..=14 => 8,
        0..=7 => 9,
        _ => {
            // SAFETY:
            //
            // The `leading_zeros` intrinsic returns the number of bits that
            // contain a zero bit. The result will always be in the range of
            // 0..=64 for a `u64`, so the above pattern is exhaustive, however
            // it is not exhaustive over the return type of `u32`. Because of
            // this, we mark the "uncovered" part of the match as unreachable
            // for performance reasons.
            #[allow(unsafe_code)]
            unsafe {
                core::hint::unreachable_unchecked()
            }
        }
    }
}
*/

/// Get the length of a `vu64` from the first byte.
///
/// NOTE: The returned value is inclusive of the first byte itself.
#[inline]
pub fn decoded_len(byte: u8) -> u8 {
    byte.leading_ones() as u8 + 1
}

/// Encode an unsigned 64-bit integer as `vu64`.
//#[inline]
pub fn encode(value: u64) -> Vu64 {
    let mut bytes = [0u8; MAX_BYTES];
    let length = encoded_len(value);
    let follow_len = length - 1;
    //
    if follow_len == 0 {
        // 1-byte special case
        bytes[0] = value as u8;
    } else if follow_len < 7 {
        let encoded = value << length as u64;
        bytes[..8].copy_from_slice(&encoded.to_le_bytes());
        let b1st = bytes[0];
        bytes[0] = !((!(b1st >> 1)) >> follow_len);
        /*
        let mut encoded = value << length as u64;
        bytes[0] = !((!((encoded & 0xFF) as u8 >> 1)) >> follow_len);
        encoded >>= 8;
        for i in 1..(length as usize) {
            bytes[i] = (encoded & 0xFF) as u8;
            encoded >>= 8;
        }
        */
    } else {
        bytes[1..].copy_from_slice(&value.to_le_bytes());
        if follow_len == 7 {
            // 8-byte special case
            bytes[0] = 0xFE;
        } else if follow_len == 8 {
            // 9-byte special case
            bytes[0] = 0xFF;
        } else {
            #[allow(unsafe_code)]
            unsafe {
                core::hint::unreachable_unchecked()
            }
        }
    }
    //
    Vu64 { bytes, length }
}

/// Decode a `v64`-encoded unsigned 64-bit integer.
///
/// Accepts a mutable reference to a slice containing the `v64`.
/// Upon success, the reference is updated to begin at the byte immediately
/// after the encoded `v64`.
#[inline]
pub fn decode(bytes: &[u8]) -> Result<u64, Error> {
    if bytes.is_empty() {
        return Err(Error::Truncated);
    }
    let length = decoded_len(bytes[0]);
    let result = decode_with_length(length, bytes)?;
    Ok(result)
    //check_result_with_length(length, result)
}

#[inline]
pub fn check_result_with_length(length: u8, result: u64) -> Result<u64, Error> {
    if length == 1 || result >= (1 << (7 * (length - 1))) {
        Ok(result)
    } else {
        Err(Error::LeadingOnes)
    }
}

//#[inline]
pub fn decode_with_length(length: u8, bytes: &[u8]) -> Result<u64, Error> {
    if bytes.len() < length as usize {
        return Err(Error::Truncated);
    }
    let follow_len = length - 1;
    //
    let result = if follow_len == 0 {
        // 1-byte special case
        bytes[0] as u64
    } else if follow_len < 7 {
        /*
        let mut encoded = [0u8; 8];
        encoded[..length as usize].copy_from_slice(&bytes[..length as usize]);
        let lsb = encoded[0] << length;
        let val = u64::from_le_bytes(encoded) & !0xFFu64;
        (val | lsb as u64) >> length
        */
        let mut val = 0u64;
        let mut i = length as usize - 1;
        while i > 0 {
            val = val << 8 | bytes[i] as u64;
            i -= 1;
        }
        let lsb = bytes[0] << length;
        ((val << 8) | lsb as u64) >> length
    } else if follow_len == 7 {
        // 8-byte special case
        /*
        let mut encoded = [0u8; 8];
        encoded[..7].copy_from_slice(&bytes[1..8]);
        u64::from_le_bytes(encoded)
        */
        let mut val = 0u64;
        let mut i = length as usize - 1;
        while i > 0 {
            val = val << 8 | bytes[i] as u64;
            i -= 1;
        }
        val
    } else if follow_len == 8 {
        // 9-byte special case
        u64::from_le_bytes(bytes[1..9].try_into().unwrap())
    } else {
        #[allow(unsafe_code)]
        unsafe {
            core::hint::unreachable_unchecked()
        }
    };
    //
    debug_assert!(length == 1 || result >= (1 << (7 * (length - 1))));
    Ok(result)
}

/// Error type
#[derive(Copy, Clone, Debug)]
pub enum Error {
    /// Value contains unnecessary leading ones
    LeadingOnes,

    /// Value is truncated / malformed
    Truncated,
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Error::LeadingOnes => "leading ones in vu64 value",
            Error::Truncated => "truncated vu64 value",
        })
    }
}

impl std::error::Error for Error {}

#[cfg(test)]
mod test_u64 {
    use super::{decode, encode, encoded_len};
    use super::{MAX_LEN1, MAX_LEN2, MAX_LEN3, MAX_LEN4, MAX_LEN5, MAX_LEN6, MAX_LEN7, MAX_LEN8};
    //
    #[test]
    fn encode_bit_pattern_examples() {
        assert_eq!(encode(0x0f0f).as_ref(), &[0x8F, 0x3c]);
        assert_eq!(encode(0x0f0f_f0f0).as_ref(), &[0xE0, 0x0f, 0xff, 0xf0]);
        assert_eq!(
            encode(0x0f0f_f0f0_0f0f).as_ref(),
            &[0xFD, 0x87, 0x07, 0x78, 0xf8, 0x87, 0x07]
        );
        assert_eq!(
            encode(0x0f0f_f0f0_0f0f_f0f0).as_ref(),
            &[0xFF, 0xf0, 0xf0, 0x0f, 0x0f, 0xf0, 0xf0, 0x0f, 0x0f]
        );
    }
    #[test]
    fn decode_bit_pattern_examples() {
        let slice = [0x8F, 0x3c].as_ref();
        assert_eq!(decode(slice).unwrap(), 0x0f0f);

        let slice = [0xE0, 0x0f, 0xff, 0xf0].as_ref();
        assert_eq!(decode(slice).unwrap(), 0x0f0f_f0f0);

        let slice = [0xFD, 0x87, 0x07, 0x78, 0xf8, 0x87, 0x07].as_ref();
        assert_eq!(decode(slice).unwrap(), 0x0f0f_f0f0_0f0f);

        let slice = [0xFF, 0xf0, 0xf0, 0x0f, 0x0f, 0xf0, 0xf0, 0x0f, 0x0f].as_ref();
        assert_eq!(decode(slice).unwrap(), 0x0f0f_f0f0_0f0f_f0f0);
    }
    #[test]
    fn encode_decode() {
        let val = 1234;
        assert_eq!(decode(encode(val).as_ref()).unwrap(), val);
        let val = 123456789;
        assert_eq!(decode(encode(val).as_ref()).unwrap(), val);
    }
    #[test]
    fn enc_dec_max_u64() {
        let val = core::u64::MAX;
        assert_eq!(encoded_len(val), 9);
        assert_eq!(
            encode(val).as_ref(),
            &[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff]
        );
        assert_eq!(decode(encode(val).as_ref()).unwrap(), val);
    }
    #[test]
    fn enc_dec_max_u32() {
        let val = core::u32::MAX.into();
        assert_eq!(encoded_len(val), 5);
        assert_eq!(encode(val).as_ref(), &[0xf7, 0xff, 0xff, 0xff, 0x1f]);
        assert_eq!(decode(encode(val).as_ref()).unwrap(), val);
    }
    #[test]
    fn enc_dec_max_u16() {
        let val = core::u16::MAX.into();
        assert_eq!(encoded_len(val), 3);
        assert_eq!(encode(val).as_ref(), &[0xdf, 0xff, 0x07]);
        assert_eq!(decode(encode(val).as_ref()).unwrap(), val);
    }
    #[test]
    fn enc_dec_max_u8() {
        let val = core::u8::MAX.into();
        assert_eq!(encoded_len(val), 2);
        assert_eq!(encode(val).as_ref(), &[0xbf, 0x03]);
        assert_eq!(decode(encode(val).as_ref()).unwrap(), val);
    }
    #[test]
    fn enc_dec_zero() {
        let val = 0u64;
        assert_eq!(encoded_len(val), 1);
        assert_eq!(encode(val).as_ref(), &[0x00]);
        assert_eq!(decode(encode(val).as_ref()).unwrap(), val);
    }
    #[test]
    fn enc_dec_max_len1() {
        let val = MAX_LEN1 as u64;
        assert_eq!(encoded_len(val), 1);
        assert_eq!(encode(val).as_ref(), &[0x7F]);
        assert_eq!(decode(encode(val).as_ref()).unwrap(), val);
        let val = val + 1;
        assert_eq!(encoded_len(val), 2);
        assert_eq!(encode(val).as_ref(), &[0x80, 0x02]);
        assert_eq!(decode(encode(val).as_ref()).unwrap(), val);
    }
    #[test]
    fn enc_dec_max_len2() {
        let val = MAX_LEN2 as u64;
        assert_eq!(encoded_len(val), 2);
        assert_eq!(encode(val).as_ref(), &[0xBF, 0xFF]);
        assert_eq!(decode(encode(val).as_ref()).unwrap(), val);
        let val = val + 1;
        assert_eq!(encoded_len(val), 3);
        assert_eq!(encode(val).as_ref(), &[0xC0, 0x00, 0x02]);
        assert_eq!(decode(encode(val).as_ref()).unwrap(), val);
    }
    #[test]
    fn enc_dec_max_len3() {
        let val = MAX_LEN3 as u64;
        assert_eq!(encoded_len(val), 3);
        assert_eq!(encode(val).as_ref(), &[0xDF, 0xFF, 0xFF]);
        assert_eq!(decode(encode(val).as_ref()).unwrap(), val);
        let val = val + 1;
        assert_eq!(encoded_len(val), 4);
        assert_eq!(encode(val).as_ref(), &[0xE0, 0x00, 0x00, 0x02]);
        assert_eq!(decode(encode(val).as_ref()).unwrap(), val);
    }
    #[test]
    fn enc_dec_max_len4() {
        let val = MAX_LEN4 as u64;
        assert_eq!(encoded_len(val), 4);
        assert_eq!(encode(val).as_ref(), &[0xEF, 0xFF, 0xFF, 0xFF]);
        assert_eq!(decode(encode(val).as_ref()).unwrap(), val);
        let val = val + 1;
        assert_eq!(encoded_len(val), 5);
        assert_eq!(encode(val).as_ref(), &[0xF0, 0x00, 0x00, 0x00, 0x02]);
        assert_eq!(decode(encode(val).as_ref()).unwrap(), val);
    }
    #[test]
    fn enc_dec_max_len5() {
        let val = MAX_LEN5 as u64;
        assert_eq!(encoded_len(val), 5);
        assert_eq!(encode(val).as_ref(), &[0xF7, 0xFF, 0xFF, 0xFF, 0xFF]);
        assert_eq!(decode(encode(val).as_ref()).unwrap(), val);
        let val = val + 1;
        assert_eq!(encoded_len(val), 6);
        assert_eq!(encode(val).as_ref(), &[0xF8, 0x00, 0x00, 0x00, 0x00, 0x02]);
        assert_eq!(decode(encode(val).as_ref()).unwrap(), val);
    }
    #[test]
    fn enc_dec_max_len6() {
        let val = MAX_LEN6 as u64;
        assert_eq!(encoded_len(val), 6);
        assert_eq!(encode(val).as_ref(), &[0xFB, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
        assert_eq!(decode(encode(val).as_ref()).unwrap(), val);
        let val = val + 1;
        assert_eq!(encoded_len(val), 7);
        assert_eq!(
            encode(val).as_ref(),
            &[0xFC, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02]
        );
        assert_eq!(decode(encode(val).as_ref()).unwrap(), val);
    }
    #[test]
    fn enc_dec_max_len7() {
        let val = MAX_LEN7 as u64;
        assert_eq!(encoded_len(val), 7);
        assert_eq!(
            encode(val).as_ref(),
            &[0xFD, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]
        );
        assert_eq!(decode(encode(val).as_ref()).unwrap(), val);
        let val = val + 1;
        assert_eq!(encoded_len(val), 8);
        assert_eq!(
            encode(val).as_ref(),
            &[0xFE, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02]
        );
        assert_eq!(decode(encode(val).as_ref()).unwrap(), val);
    }
    #[test]
    fn enc_dec_max_len8() {
        let val = MAX_LEN8 as u64;
        assert_eq!(encoded_len(val), 8);
        assert_eq!(
            encode(val).as_ref(),
            &[0xFE, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]
        );
        assert_eq!(decode(encode(val).as_ref()).unwrap(), val);
        let val = val + 1;
        assert_eq!(encoded_len(val), 9);
        assert_eq!(
            encode(val).as_ref(),
            &[0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01]
        );
        assert_eq!(decode(encode(val).as_ref()).unwrap(), val);
    }
    #[test]
    fn enc_dec_len_6() {
        let val = 0x0300_0000_0000;
        assert_eq!(encoded_len(val), 6);
        assert_eq!(encode(val).as_ref(), &[0xf8, 0x00, 0x00, 0x00, 0x00, 0xc0]);
        assert_eq!(decode(encode(val).as_ref()).unwrap(), val);
    }
    #[test]
    fn enc_dec_len_7() {
        let val = 0x1_0000_0000_0000;
        assert_eq!(encoded_len(val), 7);
        assert_eq!(
            encode(val).as_ref(),
            &[0xfc, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80]
        );
        assert_eq!(decode(encode(val).as_ref()).unwrap(), val);
    }
    #[test]
    fn enc_dec_len_8() {
        let val = 0x00f0_0000_0000_0000;
        assert_eq!(encoded_len(val), 8);
        assert_eq!(
            encode(val).as_ref(),
            &[0xfe, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf0]
        );
        assert_eq!(decode(encode(val).as_ref()).unwrap(), val);
    }
    #[test]
    fn enc_dec_len_9() {
        let val = 0x0100_0000_0000_0000;
        assert_eq!(encoded_len(val), 9);
        assert_eq!(
            encode(val).as_ref(),
            &[0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01]
        );
        assert_eq!(decode(encode(val).as_ref()).unwrap(), val);
    }
    #[test]
    fn enc_dec_all() {
        let mut val: u64 = 1;
        for _i in 0..64 {
            val = (val << 1) | 0x01;
            assert_eq!(decode(encode(val).as_ref()).unwrap(), val);
        }
    }
    #[test]
    fn enc_dec_all_1byte() {
        for val in 0..128 {
            assert_eq!(encoded_len(val), 1);
            assert_eq!(decode(encode(val).as_ref()).unwrap(), val);
        }
    }
    #[test]
    fn enc_dec_all_2byte() {
        for val in 128..16384 {
            assert_eq!(encoded_len(val), 2);
            assert_eq!(decode(encode(val).as_ref()).unwrap(), val);
        }
    }
    #[test]
    fn enc_dec_all_3byte() {
        for val in 16384..2097152 {
            assert_eq!(encoded_len(val), 3);
            assert_eq!(decode(encode(val).as_ref()).unwrap(), val);
        }
    }
    #[test]
    fn enc_dec_sp1() {
        let val = 0x06_6C80;
        assert_eq!(encoded_len(val), 3);
        assert_eq!(decode(encode(val).as_ref()).unwrap(), val);
    }
    #[test]
    fn enc_dec_sp2() {
        let val = 136;
        assert_eq!(encoded_len(val), 2);
        assert_eq!(decode(encode(val).as_ref()).unwrap(), val);
        //
        let val = 288;
        assert_eq!(encoded_len(val), 2);
        assert_eq!(decode(encode(val).as_ref()).unwrap(), val);
        //
        let val = 569;
        assert_eq!(encoded_len(val), 2);
        assert_eq!(decode(encode(val).as_ref()).unwrap(), val);
        //
        let val = 1144;
        assert_eq!(encoded_len(val), 2);
        assert_eq!(decode(encode(val).as_ref()).unwrap(), val);
        //
        let val = 2296;
        assert_eq!(encoded_len(val), 2);
        assert_eq!(decode(encode(val).as_ref()).unwrap(), val);
        //
        let val = 4601;
        assert_eq!(encoded_len(val), 2);
        assert_eq!(decode(encode(val).as_ref()).unwrap(), val);
        //
        let val = 9209;
        assert_eq!(encoded_len(val), 2);
        assert_eq!(decode(encode(val).as_ref()).unwrap(), val);
    }
    #[test]
    fn decode_err_truncated() {
        let slice = [0xF0].as_ref();
        assert!(decode(slice).is_err());
        let slice = [0xF8, 0x0f, 0xff].as_ref();
        assert!(decode(slice).is_err());
    }
    #[test]
    fn decode_err_leading_ones() {
        let slice = [0xF8, 0x00, 0x00, 0x00].as_ref();
        assert!(decode(slice).is_err());
    }
}
