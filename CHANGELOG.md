# Changelog: vu64

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased] *
### Added
* version difference link into `CHANGELOG.md`


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
