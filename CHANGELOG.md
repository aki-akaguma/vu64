# Changelog: vu64

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]


## [0.1.11] (2024-06-09)
### Changed
* rename: `config` to `config.toml`

### Fixed
* clippy: clippy::useless_vec
* clippy: clippy::slow_vector_initialization
* clippy: clippy::legacy_numeric_constants

## [0.1.10] (2023-02-12)
### Added
* miri supports on tests

### Changed
* refactored `Makefile`

### Fixed
* clippy: `len_zero`, `let_unit_value`

## [0.1.9] (2023-02-10)
### Added
* `.github/workflows/test-ubuntu.yml`
* `.github/workflows/test-macos.yml`
* `.github/workflows/test-windows.yml`
* test status badges into `README.tpl`
* more tests: `test_u64_3::xxx()`, `test_io::xxx()`
* more docs: `vu64::io`, `vu64::signed`
* `xtask`
* tests: redundant encoding

### Removed
* `COPYING`

### Fixed
* clippy: `uninlined_format_args`
* rust-version: "1.56.0" to "1.58.0"
* `LICENSE-APACHE`, `LICENSE-MIT`
* bug: redundant encode

## [0.1.8] (2023-01-28)
### Added
* `.github/workflows/test.yml`
* test status badges into `README.tpl`

### Fixed
* Makefile: rustc version `1.66.0` to `1.66.1`
* clippy: `unnecessary_cast`

## [0.1.7] (2023-01-10)
### Added
* version difference link into `CHANGELOG.md`
* rust-version = "1.56.0" into Cargo.toml
* `all-test-version` target into Makefile
* badges into README.tpl

### Changed
* rename target `test-no_std` to `test-no-default-features` on Makefile

## [0.1.6] (2023-01-06)
### Changed
* reformat `CHANGELOG.md`

### Fixed
* clippy: this let-binding has unit value

## [0.1.5] (2022-06-13)
### Changed
* change to edition 2021

## [0.1.4] (2021-01-25)
### Added
* add `vu64_debug` to feature.

## [0.1.3] (2021-12-13)
### Added
* add `decode3()`.
* add `decode2()` and `decode_with_first_and_follow()`.

### Changed
* rewrite `read_and_decode_vu64` with `decode_with_first_and_follow()`.

## [0.1.2] (2021-11-26)
### Changed
* add more test code.

## [0.1.1] (2021-11-18)
### Added
* add a signed 64-bits value encoding using zigzag encoding.

### Changed
* rewrites decode_with_length() for more speed.
* rewrites encoded_len() with a const table.

### Fixed
* redundant: decode().

## [0.1.0] (2021-11-10)
* first commit

[Unreleased]: https://github.com/aki-akaguma/vu64/compare/v0.1.11..HEAD
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
