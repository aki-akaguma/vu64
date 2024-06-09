/*!
Supports of encoding signed integers as `Vu64`.
*/
use crate::{Error, Vu64};

/// Encode a signed integer as a zigzag-encoded `Vu64`.
#[inline]
pub fn encode(value: i64) -> Vu64 {
    value.into()
}

/// Decode a zigzag-encoded bytes as a signed integer.
#[inline]
pub fn decode(bytes: &[u8]) -> Result<i64, Error> {
    super::decode(bytes).map(zigzag::decode)
}

/// Get the length in bytes of a zigzag encoded `Vu64` from the given value.
#[inline]
pub fn encoded_len(value: i64) -> u8 {
    super::encoded_len(zigzag::encode(value))
}

/// The zigzag encoding for signed integers.
///
/// This module contains the raw zigzag encoding algorithm.
/// These are used in the function of the parent [`vu64::signed`](../index.html) module,
/// and encode the "signed integer" as `u64`.
///
/// Reference: [the __ZigZag__ encoding](https://protobuf.dev/programming-guides/encoding/#signed-ints)
///
pub mod zigzag {
    /// Encode a signed 64-bit integer to a zigzag encoded `u64`
    #[inline]
    pub fn encode(value: i64) -> u64 {
        ((value << 1) ^ (value >> 63)) as u64
    }

    /// Decode a zigzag encoded `u64` to a signed 64-bit integer
    #[inline]
    pub fn decode(encoded: u64) -> i64 {
        (encoded >> 1) as i64 ^ -((encoded & 1) as i64)
    }
}

#[cfg(test)]
mod test_i64 {
    use super::super::signed::{decode, encode, encoded_len};
    #[test]
    fn encode_bit_pattern_examples() {
        assert_eq!(encode(0x0f0f).as_ref(), &[0x9E, 0x78]);
        assert_eq!(
            encode(0x0f0f_f0f0).as_ref(),
            &[0xF0, 0x3C, 0xFC, 0xC3, 0x03]
        );
        assert_eq!(
            encode(0x0f0f_f0f0_0f0f).as_ref(),
            &[0xFC, 0x0F, 0x0F, 0xF0, 0xF0, 0x0F, 0x0F]
        );
        assert_eq!(
            encode(0x0f0f_f0f0_0f0f_f0f0).as_ref(),
            &[0xFF, 0xE0, 0xE1, 0x1F, 0x1E, 0xE0, 0xE1, 0x1F, 0x1E]
        );
        //
        assert_eq!(encode(-0x0f0f).as_ref(), &[0x9D, 0x78]);
        assert_eq!(
            encode(-0x0f0f_f0f0).as_ref(),
            &[0xF7, 0x3B, 0xFC, 0xC3, 0x03]
        );
        assert_eq!(
            encode(-0x0f0f_f0f0_0f0f).as_ref(),
            &[0xFD, 0x0E, 0x0F, 0xF0, 0xF0, 0x0F, 0x0F]
        );
        assert_eq!(
            encode(-0x0f0f_f0f0_0f0f_f0f0).as_ref(),
            &[0xFF, 0xDF, 0xE1, 0x1F, 0x1E, 0xE0, 0xE1, 0x1F, 0x1E]
        );
    }
    #[test]
    fn decode_bit_pattern_examples() {
        assert_eq!(decode(&[0x9E, 0x78]).unwrap(), 0x0f0f);
        assert_eq!(
            decode(&[0xF0, 0x3C, 0xFC, 0xC3, 0x03]).unwrap(),
            0x0f0f_f0f0
        );
        assert_eq!(
            decode(&[0xFC, 0x0F, 0x0F, 0xF0, 0xF0, 0x0F, 0x0F]).unwrap(),
            0x0f0f_f0f0_0f0f
        );
        assert_eq!(
            decode(&[0xFF, 0xE0, 0xE1, 0x1F, 0x1E, 0xE0, 0xE1, 0x1F, 0x1E]).unwrap(),
            0x0f0f_f0f0_0f0f_f0f0
        );
        //
        assert_eq!(decode(&[0x9D, 0x78]).unwrap(), -0x0f0f);
        assert_eq!(
            decode(&[0xF7, 0x3B, 0xFC, 0xC3, 0x03]).unwrap(),
            -0x0f0f_f0f0
        );
        assert_eq!(
            decode(&[0xFD, 0x0E, 0x0F, 0xF0, 0xF0, 0x0F, 0x0F]).unwrap(),
            -0x0f0f_f0f0_0f0f
        );
        assert_eq!(
            decode(&[0xFF, 0xDF, 0xE1, 0x1F, 0x1E, 0xE0, 0xE1, 0x1F, 0x1E]).unwrap(),
            -0x0f0f_f0f0_0f0f_f0f0
        );
    }
    #[test]
    fn encode_decode() {
        let val = 1234;
        assert_eq!(decode(encode(val).as_ref()).unwrap(), val);
        let val = 123456789;
        assert_eq!(decode(encode(val).as_ref()).unwrap(), val);
        //
        let val = -1234;
        assert_eq!(decode(encode(val).as_ref()).unwrap(), val);
        let val = -123456789;
        assert_eq!(decode(encode(val).as_ref()).unwrap(), val);
    }
    #[test]
    fn enc_dec_max_i64() {
        let val = i64::MAX;
        assert_eq!(encoded_len(val), 9);
        assert_eq!(
            encode(val).as_ref(),
            &[0xFF, 0xFE, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]
        );
        assert_eq!(decode(encode(val).as_ref()).unwrap(), val);
    }
    #[test]
    fn enc_dec_min_i64() {
        let val = i64::MIN;
        assert_eq!(encoded_len(val), 9);
        assert_eq!(
            encode(val).as_ref(),
            &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]
        );
        assert_eq!(decode(encode(val).as_ref()).unwrap(), val);
    }
    #[test]
    fn enc_dec_max_i32() {
        let val = i32::MAX.into();
        assert_eq!(encoded_len(val), 5);
        assert_eq!(encode(val).as_ref(), &[0xF6, 0xFF, 0xFF, 0xFF, 0x1F]);
        assert_eq!(decode(encode(val).as_ref()).unwrap(), val);
    }
    #[test]
    fn enc_dec_min_i32() {
        let val = i32::MIN.into();
        assert_eq!(encoded_len(val), 5);
        assert_eq!(encode(val).as_ref(), &[0xF7, 0xFF, 0xFF, 0xFF, 0x1F]);
        assert_eq!(decode(encode(val).as_ref()).unwrap(), val);
    }
    #[test]
    fn enc_dec_max_i16() {
        let val = i16::MAX.into();
        assert_eq!(encoded_len(val), 3);
        assert_eq!(encode(val).as_ref(), &[0xDE, 0xFF, 0x07]);
        assert_eq!(decode(encode(val).as_ref()).unwrap(), val);
    }
    #[test]
    fn enc_dec_min_i16() {
        let val = i16::MIN.into();
        assert_eq!(encoded_len(val), 3);
        assert_eq!(encode(val).as_ref(), &[0xDF, 0xFF, 0x07]);
        assert_eq!(decode(encode(val).as_ref()).unwrap(), val);
    }
    #[test]
    fn enc_dec_max_i8() {
        let val = i8::MAX.into();
        assert_eq!(encoded_len(val), 2);
        assert_eq!(encode(val).as_ref(), &[0xBE, 0x03]);
        assert_eq!(decode(encode(val).as_ref()).unwrap(), val);
    }
    #[test]
    fn enc_dec_min_i8() {
        let val = i8::MIN.into();
        assert_eq!(encoded_len(val), 2);
        assert_eq!(encode(val).as_ref(), &[0xBF, 0x03]);
        assert_eq!(decode(encode(val).as_ref()).unwrap(), val);
    }
    #[test]
    fn enc_dec_zero() {
        let val = 0i64;
        assert_eq!(encoded_len(val), 1);
        assert_eq!(encode(val).as_ref(), &[0x00]);
        assert_eq!(decode(encode(val).as_ref()).unwrap(), val);
    }
    #[test]
    fn enc_dec_len_6() {
        let val = 0x0180_0000_0000;
        assert_eq!(encoded_len(val), 6);
        assert_eq!(encode(val).as_ref(), &[0xf8, 0x00, 0x00, 0x00, 0x00, 0xc0]);
        assert_eq!(decode(encode(val).as_ref()).unwrap(), val);
    }
    #[test]
    fn enc_dec_len_7() {
        let val = 0xFF00_0000_0000;
        assert_eq!(encoded_len(val), 7);
        assert_eq!(
            encode(val).as_ref(),
            &[0xfc, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF]
        );
        assert_eq!(decode(encode(val).as_ref()).unwrap(), val);
    }
    #[test]
    fn enc_dec_len_8() {
        let val = 0x007F_0000_0000_0000;
        assert_eq!(encoded_len(val), 8);
        assert_eq!(
            encode(val).as_ref(),
            &[0xFE, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFE]
        );
        assert_eq!(decode(encode(val).as_ref()).unwrap(), val);
    }
    #[test]
    fn enc_dec_len_9() {
        let val = 0x0100_0000_0000_0000;
        assert_eq!(encoded_len(val), 9);
        assert_eq!(
            encode(val).as_ref(),
            &[0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02]
        );
        assert_eq!(decode(encode(val).as_ref()).unwrap(), val);
    }
    #[test]
    fn enc_dec_all() {
        let mut val: i64 = 1;
        for _i in 0..64 {
            val = (val << 1) | 0x01;
            assert_eq!(decode(encode(val).as_ref()).unwrap(), val);
            assert_eq!(decode(encode(-val).as_ref()).unwrap(), -val);
        }
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
