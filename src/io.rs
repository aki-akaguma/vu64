use super::{decode_with_length, decoded_len, encode, MAX_BYTES};
use std::fs::File;
use std::io::{Cursor, Read, Result, Write};

/// io read interface of `vu64`
pub trait ReadVu64: std::io::Read {
    /// you can write over this by a fast routine.
    fn read_one_byte(&mut self) -> Result<u8> {
        let mut buf = [0u8; 1];
        let _ = self.read_exact(&mut buf)?;
        Ok(buf[0])
    }
    /// you can write over this by a fast routine.
    fn read_exact_max8byte(&mut self, buf: &mut [u8]) -> Result<()> {
        debug_assert!(buf.len() <= 8, "buf.len(): {} <= 8", buf.len());
        self.read_exact(buf)?;
        Ok(())
    }
    /// reads `vu64` bytes and decods it to `u64`
    fn read_and_decode_vu64(&mut self) -> Result<u64> {
        let mut buf = [0u8; MAX_BYTES];
        let byte_1st = self.read_one_byte()?;
        buf[0] = byte_1st;
        let len = decoded_len(byte_1st);
        if len > 1 {
            self.read_exact_max8byte(&mut buf[1..len as usize])?;
        }
        match decode_with_length(len, &buf[0..len as usize]) {
            Ok(i) => Ok(i),
            Err(err) => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("{}", err),
            )),
        }
    }
}

/// io write interface of `vu64`
pub trait WriteVu64: std::io::Write {
    /// encods `u64` to `vu64` bytes and writes it.
    fn encode_and_write_vu64(&mut self, value: u64) -> Result<()> {
        self.write_all(encode(value).as_ref())
    }
}

impl ReadVu64 for File {}
impl WriteVu64 for File {}
impl<T> ReadVu64 for Cursor<T> where Cursor<T>: Read {}
impl<T> WriteVu64 for Cursor<T> where Cursor<T>: Write {}
