# Changelog

All notable changes to this project will be documented in this file. This project adheres to [Semantic Versioning](https://semver.org/).

## [0.2.1] - 2024-07-01

- (Internal change)
  - Update release mechanism to use manually-triggered worklow.
  - Add CI automation.
- Update compress-tools dependency to 0.15.0
- Add support for deleting archives after they are unpacked.

## [0.2.0] - 2024-06-30

- (Internal changes)
  - Alter library to support chained unzipping backends
  - Update Clap to 4.5.8.
- Add sevenz-rust, flake2, and tar unpacking backends

## [0.1.3] - 2024-06-23

- (Internal change) Add test harness and associated git worfklows.

## [0.1.2] - 2024-06-23

- (Internal change) Begin using workspace project layout.

## [0.1.1] - 2024-06-23

- Fixed single-file extraction logic.

## [0.1.0] - 2024-06-22

- Launch of the first stable release.
- Added basic capability to unzip a directory or single file recursively.
- Based on rust project [compress-tools](https://github.com/OSSystems/compress-tools-rs) wrapping [libarchive](https://www.libarchive.org/).
