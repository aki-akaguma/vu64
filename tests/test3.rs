#[cfg(test)]
mod test3 {
    use std::io::{self, Read};
    use vu64::{
        self,
        io::{ReadVu64, WriteVu64},
    };

    // A reader that returns one byte at a time.
    struct OneByteReader<R: Read> {
        inner: R,
    }

    impl<R: Read> Read for OneByteReader<R> {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            if buf.is_empty() {
                return Ok(0);
            }
            // Read one byte at a time
            self.inner.read(&mut buf[..1])
        }
    }

    impl<R: Read> ReadVu64 for OneByteReader<R> {}

    #[test]
    fn test_read_one_byte_at_a_time() {
        let values: Vec<u64> = vec![
            0,
            1,
            127,
            128,
            16383,
            16384,
            2097151,
            2097152,
            u64::MAX,
            u64::MAX / 2,
        ];

        let mut encoded_data = Vec::new();
        for &value in &values {
            let mut cursor = io::Cursor::new(Vec::new());
            cursor.encode_and_write_vu64(value).unwrap();
            encoded_data.extend_from_slice(cursor.get_ref());
        }

        let mut reader = OneByteReader {
            inner: io::Cursor::new(encoded_data),
        };

        for &value in &values {
            let decoded = reader.read_and_decode_vu64().unwrap();
            assert_eq!(decoded, value);
        }

        // Test EOF
        assert!(reader.read_and_decode_vu64().is_err());
    }

    #[test]
    fn test_read_one_byte_at_a_time_signed() {
        let values: Vec<i64> = vec![
            0,
            1,
            -1,
            63,
            -64,
            i64::MAX,
            i64::MIN,
            i64::MAX / 2,
            i64::MIN / 2,
        ];

        let mut encoded_data = Vec::new();
        for &value in &values {
            let mut cursor = io::Cursor::new(Vec::new());
            cursor.encode_and_write_vi64(value).unwrap();
            encoded_data.extend_from_slice(cursor.get_ref());
        }

        let mut reader = OneByteReader {
            inner: io::Cursor::new(encoded_data),
        };

        for &value in &values {
            let decoded = reader.read_and_decode_vi64().unwrap();
            assert_eq!(decoded, value);
        }

        // Test EOF
        assert!(reader.read_and_decode_vi64().is_err());
    }
}
