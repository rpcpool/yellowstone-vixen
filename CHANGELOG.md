# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### 2024-11-21
- Added: Use crates for proto and gRPC client [#145] ([538b3e3])

### 2024-11-20
- Changed: Change vixen example config ([7d3fd2c])
- Added: Make prometheus username and password optional ([8a54308])

### 2024-11-11
- Added: Add setter for prefilter accounts field ([c81f7a1])

### 2024-10-30
- Added: Example vixen toml ([02f250b])

### 2024-10-29
- Fixed: gRPC reflection issues ([39bcfac])
- Changed: Cleanup ([1af377d])

### 2024-10-28
- Added: Add protobufs for orca and raydium ([86b64e5])

### 2024-10-26
- Changed: Run clippy ([43313b0])

### 2024-10-25
- Fixed: Zeroize dependency conflicts ([84a13ee])
- Changed: Cleanup ([0c3e083])
- Added: Adding flag ([75a1b97])
- Changed: Updating this error on crates ([85d921f])

### 2024-10-18
- Changed: Cleanup ([7045d8b])
- Changed: Cleanup ([df20d73])
- Changed: Update README ([6735934])

### 2024-10-17
- Fixed: Bump upstream dep versions ([c20ef77])
- Changed: Merge pull request #30 from Taylor123/fix-versioning ([31e4375])

### 2024-10-14
- Changed: Change directory structure ([881f1dd])

### 2024-10-12
- Added: Add stream builder method to add gRPC descriptors ([8e1b41a])

### 2024-10-11
- Changed: Format JSON ([c338008])
- Changed: Cleanup ([19664c1])
- Fixed: Typo fix ([bb0e2ed])

### 2024-10-10
- Changed: Cleanup ([1863f76])

### 2024-10-03
- Added: Complete Jupiter:Raydium account and instruction parsers ([228cc6a])

### 2024-09-26
- Changed: Pass 1 at removing extraneous helpers from parser ([e81b37e])
- Added: Init Raydium ([5964918])
- Changed: Clean up error & pubkey types in the parser crate ([2423bcc])

### 2024-09-24
- Changed: WIP: Jupiter programs ([84ce5c4])

### 2024-09-23
- Added: Add all proto declarations for extensions instructions ([b493d6f])
- Changed: Format code ([b86bba7])
- Changed: Cleanup ([e381c8c])
- Changed: Format and cleanup ([0a35edd])
- Changed: Resolve clippy warnings ([c485077])

### 2024-09-20
- Changed: WIP: Extensions instructions ([7c56a6c], [c1b4e5d])

### 2024-09-19
- Fixed: Stream error with duplicate program IDs ([ba53df2])

### 2024-09-18
- Added: WIP: Add token extensions instruction parsing ([a646118])
- Changed: WIP: Extensions ([3fb3c68])
- Changed: Pull upstream ([d338bfe])

### 2024-09-17
- Added: Add instructions proto for token program ([1f94712])
- Added: Unwrap fixture data in fixture macros ([1fad542])

### 2024-09-16
- Changed: Cleanup and add readme to stream-parser example ([c81859a], [fce8459])

### 2024-09-11
- Added: Add token program and token extensions protobuf ([b7df6a7])

### 2024-09-10
- Changed: Add raydium program parsers ([c56b1cf])
- Changed: Resolve lints, plus first pass at documentation ([27dff1b])

### 2024-09-09
- Changed: Update mocking ([7681283])
- Fixed: Format ([feb3181])
- Added: Add filters to fixtures ([7ced3fb])

### 2024-09-05
- Added: Add Orca account and instruction parsers ([0eb1c2b])
- Changed: Rename examples ([4333241])

### 2024-09-04
- Changed: Update READMEs and cleanup ([8a13be5], [b6f6fd2])
- Changed: Format using nightly toolchain ([3e3a554])
- Changed: Update READMEs ([30e65a3])

### 2024-08-30
- Added: Add support for adapting prebuilt parsers to gRPC ([ad74dee])
- Added: Add missing clippy config to remaining crates ([04b43d6])

### 2024-08-28
- Added: Add InstructionPipeline for reparsing instructions ([2d36c79])

### 2024-08-24
- Added: Add OpenTelemetry exporter to hold meter provider ([3182a24])

### 2024-08-23
- Changed: WIP: Implement gRPC streams + refactor everything ([3a74a1d])

### 2024-08-22
- Added: Add example crate for basic OpenTelemetry metrics ([f7bf29f])

### 2024-08-21
- Changed: Retool metrics, reinstate OTel, bump dependencies ([99dda82])

### 2024-08-13
- Changed: Merge pull request #17 from rpcpool/vixen-metrics-setup ([e77e897])

### 2024-08-10
- Removed: Drop OpenTelemetry ([c2f51ce], [cfb1255])

### 2024-08-08
- Added: Setup metrics for OpenTelemetry and Prometheus ([7963de3])
- Changed: Cleanup ([5e74813])

### 2024-08-07
- Changed: Refactor code for mock ([e422188])
- Changed: Merge branch 'main' into vixen-parsers ([9af4912])
- Changed: Merge pull request #13 from rpcpool/vixen-parsers ([5971be2])

### 2024-08-06
- Changed: Update cargo.lock ([4c318b5])
- Fixed: Formatting ([4032158])

### 2024-08-05
- Changed: Update feature flags ([17001a8])
- Changed: Cleanup ([c8589d4])

### 2024-08-01
- Fixed: README ([9c524b9])
- Changed: Cleanup ([43f9fbf])

### 2024-07-31
- Added: Add dotenv to fetch RPC_ENDPOINT and CLUSTER ([e4d5152])
- Fixed: Add env setup in mock README ([a068b7f])
- Changed: Update mock README ([68025da])

### 2024-07-30
- Added: Setup mock testing suite ([30fb5da])
- Changed: Cleanup ([f41588d])
- Added: Add readme for parser and mock ([6183f63])
- Changed: Resolve PR comments ([16243ef])
- Changed: Cleanup ([6725f5b])
- Fixed: Feature flags ([3fe77a3])
- Changed: Refactor token extension parsing ([01e26e6])
- Fixed: Add imports and installation info on readme ([9746dcf])
- Changed: Cleanup ([575cfb0])
- Fixed: README ([5cb349e])
- Changed: Cleanup ([11f6925])

### 2024-07-26
- Added: Add extended account data ([4fa3843])
- Added: Setup unit tests for parsers ([305ceaf])

### 2024-07-24
- Changed: Merge pull request #11 from WilfredAlmeida/patch-1 ([5379be7])
- Added: Create crate for housing parsers sponsored by vixen ([8b80adb])
- Added: Implement token program parser using SPL token program ([cc17443])

### 2024-07-22
- Added: Scaffold streams crate and remove Solana SDK deps ([8ce8a62])

### 2024-07-16
- Added: Add metrics and wrap entry in Runtime to configure ([c52b29c])

### 2024-07-07
- Fixed: Fix path ([af0f404])

### 2024-06-21
- Added: Update readme, add contributing and code of conduct ([961b584])
- Changed: Merge pull request #7 from rpcpool/june/runtime ([1cce465])
- Added: Add developers and section on dragonmouth to readme ([2481072])
- Fixed: Name of project on the readme ([53386e0])
- Fixed: Typos ([e565d67])
- Changed: Merge pull request #9 from rpcpool/typos-20240621 ([e298286])

### 2024-06-20
- Added: Add signal handlers & real runtime error reporting ([4e384d0])
- Changed: Resolve TODOs, change lints, and fix linter errors ([627e900])

### 2024-06-19
- Fixed: Runtime panic in topograph ([dca0f46])

### 2024-06-18
- Changed: WIP: Cleaned up some handler creation boilerplate ([0c64c96])

### 2024-06-12
- Added: Quick'n'dirty scaffold of handler manager creation ([2c1ae0d])

### 2024-06-04
- Changed: Merge pull request #4 from rpcpool/june/runtime ([88c73ff])

### 2024-05-31
- Changed: WIP: Refactor Parser trait to allow extern parsers ([54d2063])

### 2024-05-15
- Added: WIP: Minimum viable example parsing SPL Token 2022 ([07be095])

### 2024-05-04
- Changed: Continue scaffolding runtime framework for parsers ([13dd1ea])

### 2024-04-23
- Added: WIP: Begin designing subscriber runtime framework ([3f2a269])

### 2024-04-16
- Added: Initial readme ([a65f4dc])

