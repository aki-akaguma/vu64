#[cfg(test)]
mod test2 {
    use vu64::signed;
    use vu64::{self, Vu64};

    #[test]
    fn test_from_i64_for_vu64() {
        let pos_val: i64 = 67890;
        let vu64_pos: Vu64 = pos_val.into();
        let decoded_pos = signed::decode(vu64_pos.as_ref()).unwrap();
        assert_eq!(decoded_pos, pos_val);

        let neg_val: i64 = -12345;
        let vu64_neg: Vu64 = neg_val.into();
        let decoded_neg = signed::decode(vu64_neg.as_ref()).unwrap();
        assert_eq!(decoded_neg, neg_val);

        let zero: i64 = 0;
        let vu64_zero: Vu64 = zero.into();
        let decoded_zero = signed::decode(vu64_zero.as_ref()).unwrap();
        assert_eq!(decoded_zero, zero);

        let min: i64 = i64::MIN;
        let vu64_min: Vu64 = min.into();
        let decoded_min = signed::decode(vu64_min.as_ref()).unwrap();
        assert_eq!(decoded_min, min);

        let max: i64 = i64::MAX;
        let vu64_max: Vu64 = max.into();
        let decoded_max = signed::decode(vu64_max.as_ref()).unwrap();
        assert_eq!(decoded_max, max);
    }

    #[test]
    fn test_i64_edge_cases() {
        let values = [
            i64::MIN,
            i64::MAX,
            0,
            -1,
            1,
            -64,
            63,
            -65,
            64,
            i64::MIN + 1,
            i64::MAX - 1,
        ];
        for &value in &values {
            let encoded = signed::encode(value);
            let decoded = signed::decode(encoded.as_ref()).unwrap();
            assert_eq!(value, decoded);
        }
    }

    #[test]
    fn test_zigzag_properties() {
        // Small positive and negative numbers should result in small encoded values.
        for i in -10..=10 {
            let encoded = signed::zigzag::encode(i);
            assert!(
                encoded <= 20,
                "Zigzag encoding of {} is too large: {}",
                i,
                encoded
            );
            let decoded = signed::zigzag::decode(encoded);
            assert_eq!(i, decoded);
        }

        // Check boundary conditions
        let values = [i64::MIN, i64::MAX, 0, -1, 1];
        for &val in &values {
            assert_eq!(val, signed::zigzag::decode(signed::zigzag::encode(val)));
        }
    }

    #[test]
    fn test_from_i64_for_vu64_range() {
        for i in -1000..=1000 {
            let val: i64 = i;
            let vu64_val: Vu64 = val.into();
            let decoded = signed::decode(vu64_val.as_ref()).unwrap();
            assert_eq!(decoded, val);
        }
    }
}
