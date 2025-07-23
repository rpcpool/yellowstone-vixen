# Changelog

All notable changes to this project will be documented in this file.

This project follows [Keep a Changelog](https://keepachangelog.com/en/1.0.0/)
and adheres to [Semantic Versioning](https://semver.org/).

## [0.4.0] - 2025-07-23

### Added

- Added Meteora Pools Program parser by @sonicfromnewyoke in [#78](https://github.com/rpcpool/yellowstone-vixen/pull/78)
- Added Meteora Dynamic Bonding Curve Program parser by @sonicfromnewyoke in [#77](https://github.com/rpcpool/yellowstone-vixen/pull/77)
- Added Boop Program parser by @sonicfromnewyoke in [#76](https://github.com/rpcpool/yellowstone-vixen/pull/76)
- Added Raydium Launchpad Program parser by @sonicfromnewyoke in [#75](https://github.com/rpcpool/yellowstone-vixen/pull/75)
- Added Virtuals Program parser by @sonicfromnewyoke in [#74](https://github.com/rpcpool/yellowstone-vixen/pull/74)
- Added supported programs list & missing IDLs in parsers by @sonicfromnewyoke in [#80](https://github.com/rpcpool/yellowstone-vixen/pull/80)
- Added Meteora Vault Program parser by @sonicfromnewyoke in [#82](https://github.com/rpcpool/yellowstone-vixen/pull/82)
- Added codama-parser-generation md file by @fernandodeluret in [#89](https://github.com/rpcpool/yellowstone-vixen/pull/89)
- Added Multiple Sources by @kespinola in [#96](https://github.com/rpcpool/yellowstone-vixen/pull/96)

### Changed

- Removed Localset logic and improved Vixen Runtime docs by @fernandodeluret in [#85](https://github.com/rpcpool/yellowstone-vixen/pull/85)
- Updated codama-parser-generation.md by @XieJunhua in [#93](https://github.com/rpcpool/yellowstone-vixen/pull/93)
- Bumped yellowstone and exposed from_slot filter by @fernandodeluret in [#91](https://github.com/rpcpool/yellowstone-vixen/pull/91)
- Updated parsers with the latest codama features by @fernandodeluret in [#88](https://github.com/rpcpool/yellowstone-vixen/pull/88)
- Exposed tx_sig & slot to raydium handler by @fernandodeluret in [#99](https://github.com/rpcpool/yellowstone-vixen/pull/99)
- Updated RaydiumAmmV4 parser to handle optional accounts by @fernandodeluret in [#98](https://github.com/rpcpool/yellowstone-vixen/pull/98)
- Updated Vixen parsers with last Codama updates by @fernandodeluret in [#105](https://github.com/rpcpool/yellowstone-vixen/pull/105)

### Fixed

- Fixed protobuf vulnerability (RUSTSEC-2024-0437) by @aoikurokawa in [#81](https://github.com/rpcpool/yellowstone-vixen/pull/81)
- Updated example in README by @quangkeu95 in [#79](https://github.com/rpcpool/yellowstone-vixen/pull/79)

### Removed

- Removed old raydium parser from parser crate by @fernandodeluret in [#94](https://github.com/rpcpool/yellowstone-vixen/pull/94)

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
