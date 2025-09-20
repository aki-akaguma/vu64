#[cfg(test)]
mod test4 {
    use std::io::{self, Write};
    use vu64::io::WriteVu64;
    use vu64::{self};

    // A writer that will fail after a certain number of bytes are written.
    struct FailingWriter {
        written: usize,
        fail_after: usize,
    }

    impl Write for FailingWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            let bytes_to_write = buf.len();
            if self.written + bytes_to_write > self.fail_after {
                return Err(io::Error::new(io::ErrorKind::Other, "write limit reached"));
            }
            self.written += bytes_to_write;
            Ok(bytes_to_write)
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    impl WriteVu64 for FailingWriter {}

    #[test]
    fn test_write_error_handling() {
        let value = u64::MAX;
        let encoded = vu64::encode(value);
        let len = encoded.as_ref().len();

        for i in 0..len {
            let mut writer = FailingWriter {
                written: 0,
                fail_after: i,
            };
            let result = writer.encode_and_write_vu64(value);
            assert!(
                result.is_err(),
                "Should fail when writer fails after {} bytes",
                i
            );
        }

        let mut writer = FailingWriter {
            written: 0,
            fail_after: len,
        };
        let result = writer.encode_and_write_vu64(value);
        assert!(
            result.is_ok(),
            "Should succeed when writer allows all bytes"
        );
    }
}
