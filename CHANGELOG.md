# Changelog: vu64
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.0] - 2026-05-30
### Added
- Code review report: `docs/reviews/2026-05-30_code_review.2.md`.
- Support for `#![no_std]` (via `default-features = false`).
- `std` feature to enable standard library dependence.
- `check` target in `Makefile` to verify all feature combinations.
- Optimization for `TryFrom<&[u8]> for Vu64` using direct byte copying.
- Blanket `ReadVu64` and `WriteVu64` traits for all types implementing `std::io::Read`/`Write`.
- Optimization for `decode_with_length` and `decode_with_first_and_follow` using bulk memory loads.
- Refactor test suite to accommodate blanket I/O trait implementations.
- Robustness for `Vu64` Debug implementation to handle invalid internal states.
- Documentation for `Vu64` memory layout and fixed-size buffer storage.
- Detailed explanatory comments for bit manipulation logic in `encode`.

### Changed
- Improve robustness of `decode_with_first_and_follow_le` by masking input.
- Consolidate redundant encoding checks into `check_result_with_length`.
- Consolidate `Error::LeadingOnes` into `Error::RedundantEncode`.
- Improve safety of `unsafe` blocks with `// SAFETY` comments and `unreachable!()`.
- Update I/O error mapping to use `ErrorKind::InvalidData` for malformed encoding.

### Removed
- `#[allow(dead_code)]` from public `MAX_LEN` constants to clarify public API.

### Fixed
- Documentation inaccuracies for `decode`, `decode2`, and `decode3`.
- "Maximun" typos in `src/lib.rs`.

## [0.2.0] - 2025-09-24
### Added
- Project specifications.
- Additional tests.

### Fixed
- `clippy::uninlined_format_args` warning.

## [0.1.11] - 2024-06-09
### Changed
- Rename `config` to `config.toml`.

### Fixed
- `clippy::useless_vec` warning.
- `clippy::slow_vector_initialization` warning.
- `clippy::legacy_numeric_constants` warning.

## [0.1.10] - 2023-02-12
### Added
- Miri support for tests.

### Changed
- Refactor `Makefile`.

### Fixed
- `clippy::len_zero` warning.
- `clippy::let_unit_value` warning.

## [0.1.9] - 2023-02-10
### Added
- CI workflows for Ubuntu, macOS, and Windows.
- Test status badges in `README.tpl`.
- Additional tests: `test_u64_3::xxx()`, `test_io::xxx()`.
- Additional documentation: `vu64::io`, `vu64::signed`.
- `xtask` support.
- Redundant encoding tests.

### Removed
- `COPYING` file.

### Fixed
- `clippy::uninlined_format_args` warning.
- Rust version update from 1.56.0 to 1.58.0.
- `LICENSE-APACHE` and `LICENSE-MIT` files.
- Redundant encode bug.

## [0.1.8] - 2023-01-28
### Added
- CI workflow for tests.
- Test status badges in `README.tpl`.

### Fixed
- Rustc version update in `Makefile` from 1.66.0 to 1.66.1.
- `clippy::unnecessary_cast` warning.

## [0.1.7] - 2023-01-10
### Added
- Version difference links in `CHANGELOG.md`.
- `rust-version = "1.56.0"` in `Cargo.toml`.
- `all-test-version` target in `Makefile`.
- Badges in `README.tpl`.

### Changed
- Rename `test-no_std` target to `test-no-default-features` in `Makefile`.

## [0.1.6] - 2023-01-06
### Changed
- Reformat `CHANGELOG.md`.

### Fixed
- `clippy::let_unit_value` warning.

## [0.1.5] - 2022-06-13
### Changed
- Update to Rust edition 2021.

## [0.1.4] - 2021-01-25
### Added
- `vu64_debug` feature.

## [0.1.3] - 2021-12-13
### Added
- `decode3()` function.
- `decode2()` and `decode_with_first_and_follow()` functions.

### Changed
- Rewrite `read_and_decode_vu64` using `decode_with_first_and_follow()`.

## [0.1.2] - 2021-11-26
### Changed
- Additional test code.

## [0.1.1] - 2021-11-18
### Added
- Signed 64-bit value encoding using zigzag encoding.

### Changed
- Rewrite `decode_with_length()` for improved performance.
- Rewrite `encoded_len()` using a constant table.

### Fixed
- Redundant `decode()` call.

## [0.1.0] - 2021-11-10
### Added
- Initial release.

[Unreleased]: https://github.com/aki-akaguma/vu64/compare/v0.3.0..HEAD
[0.3.0]: https://github.com/aki-akaguma/vu64/compare/v0.2.0..v0.3.0
[0.2.0]: https://github.com/aki-akaguma/vu64/compare/v0.1.11..v0.2.0
[0.1.11]: https://github.com/aki-akaguma/vu64/compare/v0.1.10..v0.1.11
[0.1.10]: https://github.com/aki-akaguma/vu64/compare/v0.1.9..v0.1.10
[0.1.9]: https://github.com/aki-akaguma/vu64/compare/v0.1.8..v0.1.9
[0.1.8]: https://github.com/aki-akaguma/vu64/compare/v0.1.7..v0.1.8
[0.1.7]: https://github.com/aki-akaguma/vu64/compare/v0.1.6..v0.1.7
[0.1.6]: https://github.com/aki-akaguma/vu64/compare/v0.1.5..v0.1.6
[0.1.5]: https://github.com/aki-akaguma/vu64/compare/v0.1.4..v0.1.5
[0.1.4]: https://github.com/aki-akaguma/vu64/compare/v0.1.3..v0.1.4
[0.1.3]: https://github.com/aki-akaguma/vu64/compare/v0.1.2..v0.1.3
[0.1.2]: https://github.com/aki-akaguma/vu64/compare/v0.1.1..v0.1.2
[0.1.1]: https://github.com/aki-akaguma/vu64/compare/v0.1.0..v0.1.1
[0.1.0]: https://github.com/aki-akaguma/vu64/releases/tag/v0.1.0
