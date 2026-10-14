# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [0.0.2] - Unreleased

### Changed

- **The Rust crate's MSRV is now Rust 1.89.0** (was 1.67.1), the floor
  shared by the maintainer's Rust repositories. The old floor was never
  buildable: `reqwest` 0.12 and the `[lints]` table both need far newer
  toolchains.

### Fixed

- `rust/` builds again on current toolchains: `PaymentResult` derives
  `Debug` and `process_payment` is `pub(crate)`, as the manifest's own
  `forbid` lints require.
