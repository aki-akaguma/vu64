#[cfg(test)]
mod test1 {
    use vu64::{
        self, check_result_with_length, decode, decode_with_length, decoded_len, encode,
        encoded_len, Error, Vu64, MAX_BYTES,
    };

    #[test]
    fn test_memory_layout() {
        use std::mem::{align_of, size_of};
        // Assert the size of the Vu64 struct.
        // It contains a u8 length and a [u8; 9] byte array.
        // The size should be 1 (length) + 9 (bytes) = 10 bytes, plus any padding.
        // On most architectures, this should be packed tightly.
        let expected_size = 1 + MAX_BYTES;
        assert!(
            size_of::<Vu64>() <= expected_size + 7, // Allow for some padding, e.g., up to the size of a u64
            "Size of Vu64 should be around {}, but was {}",
            expected_size,
            size_of::<Vu64>()
        );

        // Assert the alignment of Vu64.
        // Since it's composed of u8, its alignment requirement should be 1.
        assert_eq!(align_of::<Vu64>(), 1, "Alignment of Vu64 should be 1");

        // Assert the size of the Error enum.
        // It's a simple C-like enum, so it should be the size of a u8.
        assert_eq!(size_of::<Error>(), 1, "Size of Error enum should be 1 byte");

        // Assert the alignment of the Error enum.
        assert_eq!(
            align_of::<Error>(),
            1,
            "Alignment of Error enum should be 1"
        );
    }

    #[test]
    fn test_decode_truncated() {
        // Test for truncated input at every possible length
        let enc_max = vu64::encode(u64::MAX);
        let bytes = enc_max.as_ref();
        assert_eq!(bytes.len(), 9);

        for i in 1..bytes.len() {
            let slice = &bytes[..i];
            let err = vu64::decode(slice).unwrap_err();
            assert_eq!(err, Error::Truncated, "Failed for slice of length {}", i);
        }

        // Test with a 4-byte encoding
        let val = vu64::MAX_LEN4;
        let enc_val = vu64::encode(val);
        let bytes_val = enc_val.as_ref();
        assert_eq!(bytes_val.len(), 4);

        for i in 1..bytes_val.len() {
            let slice = &bytes_val[..i];
            let err = vu64::decode(slice).unwrap_err();
            assert_eq!(
                err,
                Error::Truncated,
                "Failed for slice of length {} on a 4-byte value",
                i
            );
        }
    }

    #[test]
    fn test_redundant_encodings() {
        // Values that should be encoded in 1 byte but are encoded in 2
        // 0x80 0x01 represents 64, which fits in 1 byte (0x40)
        let redundant_2_byte = &[0x81, 0x00]; // Represents 1, should be 0x01
        assert_eq!(vu64::decode(redundant_2_byte), Err(Error::RedundantEncode));

        // Value that should be encoded in 2 bytes but is in 3
        let redundant_3_byte = &[0xC0, 0x80, 0x00]; // Represents 128, should be 0x80 0x02
        assert_eq!(vu64::decode(redundant_3_byte), Err(Error::RedundantEncode));
    }

    #[test]
    fn test_from_u64_for_vu64() {
        let val: u64 = 12345;
        let vu64_val: Vu64 = val.into();
        let decoded = vu64::decode(vu64_val.as_ref()).unwrap();
        assert_eq!(decoded, val);

        let zero: u64 = 0;
        let vu64_zero: Vu64 = zero.into();
        let decoded_zero = vu64::decode(vu64_zero.as_ref()).unwrap();
        assert_eq!(decoded_zero, zero);

        let max: u64 = u64::MAX;
        let vu64_max: Vu64 = max.into();
        let decoded_max = vu64::decode(vu64_max.as_ref()).unwrap();
        assert_eq!(decoded_max, max);
    }

    #[test]
    fn test_decode_with_length_function() {
        // 1. Test with correct length and correct slice.
        let value = 123456;
        let encoded = encode(value);
        let slice = encoded.as_ref();
        let len = encoded_len(value);
        assert_eq!(decode_with_length(len, slice).unwrap(), value);

        // 2. Test with a `length` parameter that is longer than the slice.
        let result = decode_with_length(len, &slice[..len as usize - 1]);
        assert_eq!(result, Err(Error::Truncated));
    }

    #[test]
    fn test_check_result_with_length_function() {
        // Valid: 1-byte value
        assert_eq!(check_result_with_length(1, 100).unwrap(), 100);

        // Valid: 2-byte value, just at the boundary
        let val = 1 << 7;
        assert_eq!(check_result_with_length(2, val).unwrap(), val);

        // Invalid: 2-byte value, but could fit in 1 byte (redundant)
        let val = (1 << 7) - 1;
        assert_eq!(check_result_with_length(2, val), Err(Error::LeadingOnes));

        // Valid: 9-byte value
        let val = u64::MAX;
        assert_eq!(check_result_with_length(9, val).unwrap(), val);

        // Invalid: 9-byte value, but could fit in 8 bytes
        let val = (1 << 56) - 1;
        assert_eq!(check_result_with_length(9, val), Err(Error::LeadingOnes));
    }

    #[test]
    fn test_vu64_struct_impls() {
        let value = 9876543210;
        let vu64: Vu64 = value.into();

        // Test AsRef<[u8]>
        let encoded_ref = vu64.as_ref();
        let decoded_from_ref = decode(encoded_ref).unwrap();
        assert_eq!(decoded_from_ref, value);

        // Test Debug format
        let debug_str = format!("{:?}", vu64);
        assert_eq!(debug_str, format!("V64({})", value));

        // Test Clone
        let cloned_vu64 = vu64;
        assert_eq!(vu64.as_ref(), cloned_vu64.as_ref());

        // Test PartialEq
        let another_vu64: Vu64 = value.into();
        assert_eq!(vu64, another_vu64);

        let different_value: u64 = 123;
        let different_vu64: Vu64 = different_value.into();
        assert_ne!(vu64, different_vu64);
    }

    #[test]
    fn test_decoded_len_exhaustive() {
        // 1 byte
        for i in 0..=0x7F {
            assert_eq!(decoded_len(i), 1);
        }
        // 2 bytes
        for i in 0x80..=0xBF {
            assert_eq!(decoded_len(i), 2);
        }
        // 3 bytes
        for i in 0xC0..=0xDF {
            assert_eq!(decoded_len(i), 3);
        }
        // 4 bytes
        for i in 0xE0..=0xEF {
            assert_eq!(decoded_len(i), 4);
        }
        // 5 bytes
        for i in 0xF0..=0xF7 {
            assert_eq!(decoded_len(i), 5);
        }
        // 6 bytes
        for i in 0xF8..=0xFB {
            assert_eq!(decoded_len(i), 6);
        }
        // 7 bytes
        for i in 0xFC..=0xFD {
            assert_eq!(decoded_len(i), 7);
        }
        // 8 bytes
        assert_eq!(decoded_len(0xFE), 8);
        // 9 bytes
        assert_eq!(decoded_len(0xFF), 9);
    }

    #[test]
    fn test_encoded_len_boundaries() {
        assert_eq!(encoded_len(0), 1);
        assert_eq!(encoded_len(vu64::MAX_LEN1), 1);
        assert_eq!(encoded_len(vu64::MAX_LEN1 + 1), 2);
        assert_eq!(encoded_len(vu64::MAX_LEN2), 2);
        assert_eq!(encoded_len(vu64::MAX_LEN2 + 1), 3);
        assert_eq!(encoded_len(vu64::MAX_LEN3), 3);
        assert_eq!(encoded_len(vu64::MAX_LEN3 + 1), 4);
        assert_eq!(encoded_len(vu64::MAX_LEN4), 4);
        assert_eq!(encoded_len(vu64::MAX_LEN4 + 1), 5);
        assert_eq!(encoded_len(vu64::MAX_LEN5), 5);
        assert_eq!(encoded_len(vu64::MAX_LEN5 + 1), 6);
        assert_eq!(encoded_len(vu64::MAX_LEN6), 6);
        assert_eq!(encoded_len(vu64::MAX_LEN6 + 1), 7);
        assert_eq!(encoded_len(vu64::MAX_LEN7), 7);
        assert_eq!(encoded_len(vu64::MAX_LEN7 + 1), 8);
        assert_eq!(encoded_len(vu64::MAX_LEN8), 8);
        assert_eq!(encoded_len(vu64::MAX_LEN8 + 1), 9);
        assert_eq!(encoded_len(u64::MAX), 9);
    }

    fn calculate_encoded_len(value: u64) -> u8 {
        if value <= 0x7F {
            1
        } else if value <= 0x3FFF {
            2
        } else if value <= 0x1FFFFF {
            3
        } else if value <= 0xFFFFFFF {
            4
        } else if value <= 0x7FFFFFFFF {
            5
        } else if value <= 0x3FFFFFFFFFF {
            6
        } else if value <= 0x1FFFFFFFFFFFF {
            7
        } else if value <= 0xFFFFFFFFFFFFFF {
            8
        } else {
            9
        }
    }

    #[test]
    fn test_encoded_len_table_correctness() {
        // Check boundaries
        let boundaries = [
            0,
            1,
            0x7F,
            0x80,
            0x3FFF,
            0x4000,
            0x1FFFFF,
            0x200000,
            0xFFFFFFF,
            0x10000000,
            0x7FFFFFFFF,
            0x800000000,
            0x3FFFFFFFFFF,
            0x40000000000,
            0x1FFFFFFFFFFFF,
            0x2000000000000,
            0xFFFFFFFFFFFFFF,
            0x100000000000000,
            u64::MAX,
        ];

        for &val in &boundaries {
            assert_eq!(
                encoded_len(val),
                calculate_encoded_len(val),
                "Failed for value: {:#x}",
                val
            );
        }

        // Check leading zero counts
        for i in 0..=64 {
            // Create a number with `i` leading zeros
            let value = if i == 64 { 0 } else { u64::MAX >> i };
            let expected_len = calculate_encoded_len(value);
            assert_eq!(
                encoded_len(value),
                expected_len,
                "Failed for leading zero count: {}",
                i
            );
        }
    }

    #[test]
    fn test_error_enum_impls() {
        use std::error::Error as _;

        let err = Error::Truncated;
        assert_eq!(format!("{}", err), "truncated vu64 value");
        assert!(err.source().is_none());

        let err = Error::RedundantEncode;
        assert_eq!(format!("{}", err), "redundant encoded vu64 value");
        assert!(err.source().is_none());

        let err = Error::LeadingOnes;
        assert_eq!(format!("{}", err), "leading ones in vu64 value");
        assert!(err.source().is_none());
    }

    #[test]
    fn test_vu64_try_from_slice() {
        use std::convert::TryFrom;

        // Success case
        let val: u64 = 123456789;
        let encoded = vu64::encode(val);
        let slice = encoded.as_ref();
        let vu64_res = Vu64::try_from(slice);
        assert!(vu64_res.is_ok());
        let vu64_val = vu64_res.unwrap();
        assert_eq!(vu64_val.as_ref(), slice);
        assert_eq!(vu64::decode(vu64_val.as_ref()).unwrap(), val);

        // Error case: truncated
        let truncated_slice = &slice[..slice.len() - 1];
        let truncated_res = Vu64::try_from(truncated_slice);
        assert!(truncated_res.is_err());
        assert_eq!(truncated_res.unwrap_err(), Error::Truncated);

        // Error case: redundant
        let redundant_slice: &[u8] = &[0x81, 0x00]; // Redundant encoding of 1
        let redundant_res = Vu64::try_from(redundant_slice);
        assert!(redundant_res.is_err());
        assert_eq!(redundant_res.unwrap_err(), Error::RedundantEncode);

        // Error case: empty slice
        let empty_slice: &[u8] = &[];
        let empty_res = Vu64::try_from(empty_slice);
        assert!(empty_res.is_err());
        assert_eq!(empty_res.unwrap_err(), Error::Truncated);
    }
}
