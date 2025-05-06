# Changelog

All notable changes to this project will be documented in this file.

This project follows [Keep a Changelog](https://keepachangelog.com/en/1.0.0/)
and adheres to [Semantic Versioning](https://semver.org/).

## [0.3.0] - 2025-05-06

### Added

- Added Kamino Limit Orders parser (https://github.com/rpcpool/yellowstone-vixen/pull/72)
- Added new set of pasers: (https://github.com/rpcpool/yellowstone-vixen/pull/70)
  - meteora-amm
  - moonshot
  - orca-whirlpool
  - pump-swaps
  - raydium-amm-v4
  - raydium-clmm
  - raydium-cpmm
- Jupiter Swaps Parser (https://github.com/rpcpool/yellowstone-vixen/pull/69)
- Meteora parser (https://github.com/rpcpool/yellowstone-vixen/pull/65)
- Pump Fun parser (https://github.com/rpcpool/yellowstone-vixen/pull/66)
- Subscribe to block_meta (https://github.com/rpcpool/yellowstone-vixen/pull/67)

### Changed

- Remove feature `orca` from `yellowstone-vixen-parser` moved parser to dedicated crate `yellowstone-vixen-orca-whirlpool-parser`
- Update raydium-amm-v4 discriminators strategy (https://github.com/rpcpool/yellowstone-vixen/pull/73)
- Proto generation with Codama (https://github.com/rpcpool/yellowstone-vixen/pull/68)

## [0.2.0] - 2025-04-03

### Added

- Set commitment level on grpc subscription (https://github.com/rpcpool/yellowstone-vixen/pull/58)

### Changed

- Make `AccountKeys` get method public (https://github.com/rpcpool/yellowstone-vixen/pull/60)

[Unreleased]: https://github.com/you/project/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/you/project/releases/tag/v0.2.0
