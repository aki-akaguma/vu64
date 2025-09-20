#[cfg(test)]
mod test5 {
    use std::io::{self, Cursor, Read, Write};
    use vu64::io::{ReadVu64, WriteVu64};

    // A custom reader that reads data in small, fixed-size chunks.
    struct ChunkedReader<R: Read> {
        inner: R,
        chunk_size: usize,
    }

    impl<R: Read> Read for ChunkedReader<R> {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            let bytes_to_read = self.chunk_size.min(buf.len());
            self.inner.read(&mut buf[..bytes_to_read])
        }
    }

    impl<R: Read> ReadVu64 for ChunkedReader<R> {}

    // A simple writer that appends to a Vec<u8>.
    struct VecWriter {
        buf: Vec<u8>,
    }

    impl Write for VecWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.buf.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    impl WriteVu64 for VecWriter {}

    #[test]
    fn test_io_with_custom_reader_and_writer() {
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
            42,
            999999,
        ];

        // 1. Write values to a VecWriter.
        let mut writer = VecWriter { buf: Vec::new() };
        for &value in &values {
            let result = writer.encode_and_write_vu64(value);
            assert!(result.is_ok());
        }

        // The writer now contains all the encoded bytes.
        let encoded_data = writer.buf;

        // 2. Read values back with a ChunkedReader.
        // We will try various chunk sizes to test robustness.
        for chunk_size in 1..=10 {
            let cursor = Cursor::new(encoded_data.clone());
            let mut reader = ChunkedReader {
                inner: cursor,
                chunk_size,
            };

            let mut decoded_values = Vec::new();
            while let Ok(val) = reader.read_and_decode_vu64() {
                decoded_values.push(val);
            }

            assert_eq!(
                decoded_values, values,
                "Failed with chunk size: {}",
                chunk_size
            );
        }
    }

    #[test]
    fn test_signed_io_with_custom_reader_and_writer() {
        let values: Vec<i64> = vec![0, -1, 1, -100, 100, i64::MAX, i64::MIN, 12345, -54321];

        // 1. Write values to a VecWriter.
        let mut writer = VecWriter { buf: Vec::new() };
        for &value in &values {
            let result = writer.encode_and_write_vi64(value);
            assert!(result.is_ok());
        }

        let encoded_data = writer.buf;

        // 2. Read values back with a ChunkedReader.
        let cursor = Cursor::new(encoded_data);
        let mut reader = ChunkedReader {
            inner: cursor,
            chunk_size: 3, // Just pick one chunk size for this test
        };

        let mut decoded_values = Vec::new();
        while let Ok(val) = reader.read_and_decode_vi64() {
            decoded_values.push(val);
        }

        assert_eq!(decoded_values, values);
    }
}
