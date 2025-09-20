// A simple pseudo-random number generator for deterministic tests.
// https://en.wikipedia.org/wiki/Linear_congruential_generator
struct Lcg {
    state: u64,
}

impl Lcg {
    fn new(seed: u64) -> Self {
        Lcg { state: seed }
    }

    #[allow(dead_code)]
    fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1);
        self.state
    }

    #[allow(dead_code)]
    fn next_u8(&mut self) -> u8 {
        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1);
        (self.state >> 32) as u8
    }
}

#[cfg(test)]
mod test_fuzz {
    use super::Lcg;
    use vu64::{self, decode, decode2, decode3, encode, signed, Error};

    #[test]
    fn fuzz_u64_encode_decode() {
        let mut rng = Lcg::new(0xdeadbeef);
        for _ in 0..100_000 {
            let value = rng.next_u64();
            let encoded = vu64::encode(value);
            let decoded = vu64::decode(encoded.as_ref()).unwrap();
            assert_eq!(value, decoded);
        }
    }

    #[test]
    fn fuzz_i64_encode_decode() {
        let mut rng = Lcg::new(0xbadc0ffee);
        for _ in 0..100_000 {
            let value = rng.next_u64() as i64;
            let encoded = vu64::signed::encode(value);
            let decoded = vu64::signed::decode(encoded.as_ref()).unwrap();
            assert_eq!(value, decoded);
        }
    }

    #[test]
    fn fuzz_decode_with_random_bytes() {
        let mut rng = Lcg::new(0x12345678);
        let mut bytes = [0u8; 10];

        for _ in 0..100_000 {
            for a in &mut bytes {
                *a = rng.next_u8();
            }

            // We don't care about the result, just that it doesn't panic.
            // The decode function should either return a valid number or an error.
            let _ = vu64::decode(&bytes);
            let _ = signed::decode(&bytes);
        }
    }

    #[test]
    fn test_decode_equivalency() {
        let mut rng = Lcg::new(0xabcdef1234567890);
        for _ in 0..100_000 {
            let value = rng.next_u64();
            let encoded = encode(value);
            let slice = encoded.as_ref();

            // decode
            let r1 = decode(slice).unwrap();

            // decode2
            let r2 = decode2(slice[0], &slice[1..]).unwrap();

            // decode3
            let mut buf = [0u8; 8];
            if slice.len() > 1 {
                buf[..slice.len() - 1].copy_from_slice(&slice[1..]);
            }
            let follow_le = u64::from_le_bytes(buf);
            let r3 = decode3(slice[0], follow_le).unwrap();

            assert_eq!(value, r1, "decode failed for {}", value);
            assert_eq!(value, r2, "decode2 failed for {}", value);
            assert_eq!(value, r3, "decode3 failed for {}", value);
        }
    }

    #[test]
    fn test_decode2_thoroughly() {
        let mut rng = Lcg::new(0xdeadbeefbadc0fee);

        for _ in 0..100_000 {
            let value = rng.next_u64();
            let encoded = encode(value);
            let slice = encoded.as_ref();

            // Test success case
            let result = decode2(slice[0], &slice[1..]);
            assert_eq!(result.unwrap(), value);

            // Test truncated error
            if slice.len() > 1 {
                let result = decode2(slice[0], &slice[1..slice.len() - 1]);
                assert_eq!(result, Err(Error::Truncated));
            }
        }

        // Test redundant encoding
        let redundant_slice = &[0x81, 0x00]; // Redundant encoding of 1
        let result = decode2(redundant_slice[0], &redundant_slice[1..]);
        assert_eq!(result, Err(Error::RedundantEncode));
    }

    #[test]
    fn test_decode3_thoroughly() {
        let mut rng = Lcg::new(0x1234567890abcdef);

        for _ in 0..100_000 {
            let value = rng.next_u64();
            let encoded = encode(value);
            let slice = encoded.as_ref();

            let mut buf = [0u8; 8];
            if slice.len() > 1 {
                buf[..slice.len() - 1].copy_from_slice(&slice[1..]);
            }
            let follow_le = u64::from_le_bytes(buf);

            // Test success case
            let result = decode3(slice[0], follow_le);
            assert_eq!(result.unwrap(), value);
        }

        // Test redundant encoding
        // Note: decode3 cannot fail on truncation because it takes a full u64.
        // It can only fail on redundancy.
        let redundant_slice = &[0x81, 0x00]; // Redundant encoding of 1
        let mut buf = [0u8; 8];
        buf[..1].copy_from_slice(&redundant_slice[1..]);
        let follow_le = u64::from_le_bytes(buf);
        let result = decode3(redundant_slice[0], follow_le);
        assert_eq!(result, Err(Error::RedundantEncode));
    }

    #[test]
    fn test_interoperability() {
        let mut rng = Lcg::new(0xf0f0f0f0f0f0f0f0);

        for _ in 0..100_000 {
            let value = rng.next_u64();

            // 1 -> 2
            let enc1 = encode(value);
            let dec1 = decode2(enc1.as_ref()[0], &enc1.as_ref()[1..]).unwrap();
            let enc2 = encode(dec1);
            let dec2 = vu64::decode(enc2.as_ref()).unwrap();
            assert_eq!(value, dec2);

            // 2 -> 3
            let enc3 = encode(value);
            let mut buf = [0u8; 8];
            if enc3.as_ref().len() > 1 {
                buf[..enc3.as_ref().len() - 1].copy_from_slice(&enc3.as_ref()[1..]);
            }
            let follow_le = u64::from_le_bytes(buf);
            let dec3 = decode3(enc3.as_ref()[0], follow_le).unwrap();
            let enc4 = encode(dec3);
            let dec4 = decode2(enc4.as_ref()[0], &enc4.as_ref()[1..]).unwrap();
            assert_eq!(value, dec4);
        }
    }
}
