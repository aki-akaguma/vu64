#[cfg(test)]
mod test_redundant {
    // the redundant encoding test
    #[test]
    fn decode_bug_1() {
        // at 2023-02-09: Redundant encoding
        // 0xDD: 0b1101_1101
        let buf = vec![0xDD, 0, 0];
        let r = vu64::decode(&buf);
        assert!(
            r.is_err(),
            "assertion failed: r.is_err() with 1st byte: 0x{:02X}",
            buf[0]
        );
        assert_eq!(r.unwrap_err(), vu64::Error::RedundantEncode);
    }
    #[test]
    fn decode_bug_2() {
        // at 2023-02-09: Redundant encoding
        // 0xDD: 0b1101_1101
        let buf = vec![0xDD, 0, 0];
        let r = vu64::decode2(buf[0], &buf[1..]);
        assert!(
            r.is_err(),
            "assertion failed: r.is_err() with 1st byte: 0x{:02X}",
            buf[0]
        );
        assert_eq!(r.unwrap_err(), vu64::Error::RedundantEncode);
    }
    #[test]
    fn decode_bug_3() {
        // at 2023-02-09: Redundant encoding
        // 0xDD: 0b1101_1101
        let buf = vec![0xDD, 0, 0, 0, 0, 0, 0, 0, 0];
        let mut byte8 = [0u8; 8];
        byte8.copy_from_slice(&buf[1..]);
        let follow_le: u64 = u64::from_le_bytes(byte8);
        let r = vu64::decode3(buf[0], follow_le);
        assert!(
            r.is_err(),
            "assertion failed: r.is_err() with 1st byte: 0x{:02X}",
            buf[0]
        );
        assert_eq!(r.unwrap_err(), vu64::Error::RedundantEncode);
    }
    #[test]
    fn decode_dataset() {
        #[rustfmt::skip]
        let data = vec![
            [0x8B, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // 1000_1011
            [0x92, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // 1001_0010
            [0xA0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // 1010_0000
            [0xD8, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // 1101_1000
            [0xDD, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // 1101_1101
            [0xE0, 0xE4, 0x51, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // 1110_0000
            [0xE0, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // 1110_0000
        ];
        for a in data.iter() {
            do_check_redundant_1(a);
            do_check_redundant_2(a);
            do_check_redundant_3(a);
        }
    }
    fn do_check_redundant_1(ary: &[u8; 9]) {
        let r = vu64::decode(&ary[0..]);
        assert!(
            r.is_err(),
            "assertion failed: r.is_err() with 1st byte: 0x{:02X}",
            ary[0]
        );
        assert_eq!(r.unwrap_err(), vu64::Error::RedundantEncode);
    }
    fn do_check_redundant_2(ary: &[u8; 9]) {
        let r = vu64::decode2(ary[0], &ary[1..]);
        assert!(
            r.is_err(),
            "assertion failed: r.is_err() with 1st byte: 0x{:02X}",
            ary[0]
        );
        assert_eq!(r.unwrap_err(), vu64::Error::RedundantEncode);
    }
    fn do_check_redundant_3(ary: &[u8; 9]) {
        let mut byte8 = [0u8; 8];
        byte8.copy_from_slice(&ary[1..]);
        let follow_le: u64 = u64::from_le_bytes(byte8);
        let r = vu64::decode3(ary[0], follow_le);
        assert!(
            r.is_err(),
            "assertion failed: r.is_err() with 1st byte: 0x{:02X}",
            ary[0]
        );
        assert_eq!(r.unwrap_err(), vu64::Error::RedundantEncode);
    }
}
