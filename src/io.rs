use super::signed::zigzag;
use super::{decode_with_first_and_follow, decoded_len, encode, MAX_BYTES};
use std::fs::File;
use std::io::{Cursor, Read, Result, Write};

/// io read interface of `vu64` and `vi64`
pub trait ReadVu64: std::io::Read {
    /// you can write over this by a fast routine.
    #[inline]
    fn read_one_byte(&mut self) -> Result<u8> {
        let mut buf = [0u8; 1];
        let _ = self.read_exact(&mut buf)?;
        Ok(buf[0])
    }
    /// you can write over this by a fast routine.
    #[inline]
    fn read_exact_max8byte(&mut self, buf: &mut [u8]) -> Result<()> {
        debug_assert!(buf.len() <= 8, "buf.len(): {} <= 8", buf.len());
        self.read_exact(buf)?;
        Ok(())
    }
    /// reads `vu64` bytes and decods it to `u64`
    fn read_and_decode_vu64(&mut self) -> Result<u64> {
        let mut buf = [0u8; MAX_BYTES - 1];
        let byte_1st = self.read_one_byte()?;
        let len = decoded_len(byte_1st);
        if len > 1 {
            self.read_exact_max8byte(&mut buf[..len as usize - 1])?;
        }
        match decode_with_first_and_follow(len, byte_1st, &buf[..len as usize - 1]) {
            Ok(i) => Ok(i),
            Err(err) => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("{}", err),
            )),
        }
    }
    /// reads `vi64` bytes and decods it to `i64`
    #[inline]
    fn read_and_decode_vi64(&mut self) -> Result<i64> {
        self.read_and_decode_vu64().map(zigzag::decode)
    }
}

/// io write interface of `vu64` and `vi64`
pub trait WriteVu64: std::io::Write {
    /// encods `u64` to `vu64` bytes and writes it.
    #[inline]
    fn encode_and_write_vu64(&mut self, value: u64) -> Result<()> {
        self.write_all(encode(value).as_ref())
    }
    /// encods `i64` to `vi64` bytes and writes it.
    #[inline]
    fn encode_and_write_vi64(&mut self, value: i64) -> Result<()> {
        self.encode_and_write_vu64(zigzag::encode(value))
    }
}

impl ReadVu64 for File {}
impl WriteVu64 for File {}
impl<T> ReadVu64 for Cursor<T> where Cursor<T>: Read {}
impl<T> WriteVu64 for Cursor<T> where Cursor<T>: Write {}
