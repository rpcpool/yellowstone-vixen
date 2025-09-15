# Changelog

All notable changes to this project will be documented in this file.

This project follows [Keep a Changelog](https://keepachangelog.com/en/1.0.0/)
and adheres to [Semantic Versioning](https://semver.org/).

## [0.5.0] - 2025-09-15

### Added

- Exposed transaction included and required accounts using `FilterPipeline` ([#104](https://github.com/rpcpool/yellowstone-vixen/pull/104) by @fernandodeluret)
- Added Stake Pool Program parser ([#87](https://github.com/rpcpool/yellowstone-vixen/pull/87) by @aoikurokawa)
- Exposed transaction message header to shared data ([#121](https://github.com/rpcpool/yellowstone-vixen/pull/121) by @fernandodeluret)
- Added fumarole `Source` ([#111](https://github.com/rpcpool/yellowstone-vixen/pull/111) by @fernandodeluret)
- Added support for all accounts fetching with empty filter ([#124](https://github.com/rpcpool/yellowstone-vixen/pull/124) by @fernandodeluret)
- Added block subscriber to Vixen ([#126](https://github.com/rpcpool/yellowstone-vixen/pull/126) by @Nagaprasadvr)
- Updated Vixen metrics to receive prometheus register ([#118](https://github.com/rpcpool/yellowstone-vixen/pull/118) by @fernandodeluret)
- Documentation and example updates for 0.5 release ([#125](https://github.com/rpcpool/yellowstone-vixen/pull/125) by @kespinola)

### Changed

- Updated Raydium CPMM and Launchpad to latest program version ([#122](https://github.com/rpcpool/yellowstone-vixen/pull/122) by @fernandodeluret)
- Updated the discriminator tracing log to debug formatting ([#113](https://github.com/rpcpool/yellowstone-vixen/pull/113) by @fernandodeluret)
- Updated contribution guide ([#123](https://github.com/rpcpool/yellowstone-vixen/pull/123) by @moses7054)

### Fixed

- Return runtime error on stream error ([#112](https://github.com/rpcpool/yellowstone-vixen/pull/112) by @fernandodeluret)
- Fixed import typo in README.md ([#117](https://github.com/rpcpool/yellowstone-vixen/pull/117) by @riprsa)
- Fixed various bugs in block subscriber and related features ([#126](https://github.com/rpcpool/yellowstone-vixen/pull/126) by @Nagaprasadvr)

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
