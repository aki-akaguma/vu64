#[cfg(test)]
mod test_integration {
    use std::convert::TryFrom;
    use std::io::Cursor;
    use vu64::{
        self, encoded_len,
        io::{ReadVu64, WriteVu64},
        signed, Vu64,
    };

    #[test]
    fn kitchen_sink_integration_test() {
        // 1. Create a list of interesting i64 and u64 values.
        let i64_values: Vec<i64> = vec![
            0,
            -1,
            1,
            i64::MAX,
            i64::MIN,
            12345,
            -54321,
            63,
            -64,
            64,
            -65,
        ];
        let u64_values: Vec<u64> = vec![0, 1, u64::MAX, 127, 128, 16383, 16384, 9876543210];

        // 2. Zigzag encode the i64 values.
        let zigzag_encoded_values: Vec<u64> = i64_values
            .iter()
            .map(|&v| signed::zigzag::encode(v))
            .collect();

        // 3. Create a mixed list of u64 values.
        let mut all_values = u64_values.clone();
        all_values.extend_from_slice(&zigzag_encoded_values);
        all_values.sort(); // Sort to have a predictable order
        all_values.dedup();

        let mut encoded_bytes_buffer: Vec<u8> = Vec::new();

        // 4. Encode each value and write it to the buffer.
        for &value in &all_values {
            // 4a. Encode to a Vu64 struct.
            let vu64_val = vu64::encode(value);

            // 4b. Check the Debug representation.
            assert_eq!(format!("{:?}", vu64_val), format!("V64({})", value));

            // Check encoded_len
            assert_eq!(vu64_val.as_ref().len(), encoded_len(value) as usize);

            // 4c. Write to buffer.
            let mut cursor = Cursor::new(Vec::new());
            let write_res = cursor.encode_and_write_vu64(value);
            assert!(write_res.is_ok());
            encoded_bytes_buffer.extend_from_slice(cursor.get_ref());

            // Also test TryFrom
            let vu64_from_slice = Vu64::try_from(cursor.get_ref().as_slice()).unwrap();
            assert_eq!(vu64_from_slice, vu64_val);
        }

        // 5. Create a reader for the buffer.
        let mut reader = Cursor::new(encoded_bytes_buffer);
        let mut decoded_values: Vec<u64> = Vec::new();

        // 6. Read values back one by one.
        while let Ok(decoded_val) = reader.read_and_decode_vu64() {
            decoded_values.push(decoded_val);
        }

        // 8. Compare the final list with the original.
        assert_eq!(
            decoded_values, all_values,
            "The final decoded list of values should match the original sorted list."
        );

        // 7. (Inside the loop was too complex, let's do it here)
        // Re-read the buffer and test all decode variants.
        let mut full_buffer_reader = Cursor::new(reader.into_inner());
        for &original_value in &all_values {
            let current_pos = full_buffer_reader.position() as usize;
            let remaining_bytes = &full_buffer_reader.get_ref()[current_pos..];

            let len = vu64::decoded_len(remaining_bytes[0]) as usize;

            // Test decode
            let val1 = vu64::decode(&remaining_bytes[..len]).unwrap();
            assert_eq!(val1, original_value);

            // Test decode2
            let val2 = vu64::decode2(remaining_bytes[0], &remaining_bytes[1..len]).unwrap();
            assert_eq!(val2, original_value);

            // Test decode3
            let mut buf3 = [0u8; 8];
            if len > 1 {
                buf3[..len - 1].copy_from_slice(&remaining_bytes[1..len]);
            }
            let follow_le = u64::from_le_bytes(buf3);
            let val3 = vu64::decode3(remaining_bytes[0], follow_le).unwrap();
            assert_eq!(val3, original_value);

            full_buffer_reader.set_position(full_buffer_reader.position() + len as u64);
        }

        // Check we are at the end of the buffer
        assert_eq!(
            full_buffer_reader.position() as usize,
            full_buffer_reader.get_ref().len()
        );
    }
}
