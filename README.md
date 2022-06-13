# vu64

The variable length integer encoding of u64.
This is a simple and fast encoder/decoder.

## Features

- integer value length compaction
- minimum support rustc 1.56.1 (59eed8a2a 2021-11-01)

### format pattern

| Prefix     | Precision | Total Bytes |
|------------|-----------|-------------|
| `0xxxxxxx` | 7 bits    | 1 byte      |
| `10xxxxxx` | 14 bits   | 2 bytes     |
| `110xxxxx` | 21 bits   | 3 bytes     |
| `1110xxxx` | 28 bits   | 4 bytes     |
| `11110xxx` | 35 bits   | 5 bytes     |
| `111110xx` | 42 bits   | 6 bytes     |
| `1111110x` | 49 bits   | 7 bytes     |
| `11111110` | 56 bits   | 8 bytes     |
| `11111111` | 64 bits   | 9 bytes     |

This format is a like [`vint64`](https://crates.io/crates/vint64),
but 0x00 is represented by 0x00.

## Examples

### Encode

```rust
use vu64::encode;

assert_eq!(encode(0x0f0f).as_ref(), &[0x8F, 0x3c]);
```

### Decode

```rust
use vu64::decode;

let slice = [0x8F, 0x3c].as_ref();
assert_eq!(decode(slice).unwrap(), 0x0f0f);
```

### Encode and decode

```rust
use vu64::{encode, decode};

let val = 1234;
assert_eq!(decode(encode(val).as_ref()).unwrap(), val);
```


# Changelogs

[This crate's changelog here.](https://github.com/aki-akaguma/vu64/blob/main/CHANGELOG.md)

# References

+ [vint64](https://crates.io/crates/vint64)
+ [varu64](https://crates.io/crates/varu64)
+ [varbincode](https://crates.io/crates/varbincode)

# License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   https://opensource.org/licenses/MIT)

at your option.
