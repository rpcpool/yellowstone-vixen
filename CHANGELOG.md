# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### 2024-01-15
- Fixed: Improve onboarding of the project ([b056d2b])

### 2024-11-29
- Fixed: Truncate the id for parsers to be below 32 characters to meet grpc validations ([4b61e84])

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
- Changed: Update readme and add orca, raydium proto parsers to example binary ([f5a0b9d])
- Changed: Cleanup ([1af377d])

### 2024-10-28
- Added: Add protobufs for orca and raydium ([86b64e5])

### 2024-10-26
- Changed: Run clippy ([43313b0])
- Merged: Pull request #34 for version conflict with zeroize ([51d164c])

### 2024-10-25
- Fixed: Zeroize dependency conflicts ([84a13ee])
- Changed: Cleanup ([0c3e083])
- Added: Adding flag ([75a1b97])
- Changed: Updating this error on crates ([85d921f])

### 2024-10-18
- Changed: Cleanup ([7045d8b])
- Changed: Update README ([6735934])
- Changed: Cleanup ([df20d73])
- Merged: Pull request #22 from rpcpool/naga-jup-program-parsers-rebase ([3efcbe9])

### 2024-10-17
- Fixed: Bump upstream dep versions ([c20ef77])
- Merged: Pull request #30 from Taylor123/fix-versioning ([31e4375])

### 2024-10-15
- Merged: Pull request #29 from rpcpool/june/stream-reflection ([4e6fa91])
- Changed: Change proto_helpers module to proto module in helpers ([4594a88])

### 2024-10-14
- Changed: Change directory structure ([881f1dd])
- Changed: Change dir structure and change acc, ix parsers naming conventions ([73964e1])

### 2024-10-12
- Added: Add stream builder method to add gRPC descriptors ([8e1b41a])

### 2024-10-11
- Changed: Format JSON ([c338008])
- Changed: Cleanup ([19664c1])
- Fixed: Typo fix ([bb0e2ed])
- Changed: Refactor mock crate to run fixtures as well as parsing ([d9600e4])

### 2024-10-10
- Changed: Cleanup ([1863f76])

### 2024-10-09
- Merged: Pull request #25 from rpcpool/naga/protobufs-ixs ([661276b])
- Merged: Pull request #28 from rpcpool/june/parser-refactor ([097f66b])

### 2024-10-03
- Added: Complete Jupiter:Raydium account and instruction parsers ([228cc6a])

### 2024-09-27
- Changed: Rebase off of june/parser-refactor ([156494b])

### 2024-09-26
- Added: Init Raydium ([5964918])
- Changed: Clean up error & pubkey types in the parser crate ([2423bcc])
- Changed: Pass 1 at removing extraneous helpers from parser ([e81b37e])

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
- Merged: Pull request #23 from rpcpool/june/docs ([4b2d423])

### 2024-09-18
- Added: WIP: Add token extensions instruction parsing ([a646118])
- Changed: WIP: Extensions ([3fb3c68])
- Changed: Pull upstream ([d338bfe])
- Merged: Pull request #24 from rpcpool/naga/protobufs-accs-ixs ([6b4668b])
- Changed: Format code ([128d12f])
- Merged: Pull request #26 from rpcpool/june/mock-refactor ([e801cc3])

### 2024-09-17
- Added: Add instructions proto for token program ([1f94712])
- Added: Unwrap fixture data in fixture macros ([1fad542])

### 2024-09-16
- Changed: Cleanup and add readme to stream-parser example ([c81859a], [fce8459])
- Changed: Refactor proto bytes to string for pubkey fields ([a312803])

### 2024-09-13
- Changed: Format and cleanup ([baa45cb])
- Added: Finish up proto account parsers for token program and token extensions ([2bb863f])

### 2024-09-11
- Added: Add token program and token extensions protobuf ([b7df6a7])

### 2024-09-10
- Changed: Add raydium program parsers ([c56b1cf])
- Changed: Resolve lints, plus first pass at documentation ([27dff1b])

### 2024-09-09
- Changed: Update mocking ([7681283])
- Fixed: Format ([feb3181])
- Added: Add filters to fixtures ([7ced3fb])

### 2024-09-06
- Fixed: Remove Vixen toml ([3498f8b])
- Merged: Pull request #20 from rpcpool/main-fix-readme ([524c21d])
- Fixed: README ([f153739])

### 2024-09-05
- Added: Add Orca account and instruction parsers ([0eb1c2b])
- Changed: Rename examples ([4333241])
- Merged: Pull request #19 from rpcpool/naga/tx-parsers-rebased ([3ba383e])
- Merged: Pull request #18 from rpcpool/june/streams ([ded47de])

### 2024-09-04
- Changed: Update READMEs and cleanup ([8a13be5], [b6f6fd2])
- Changed: Format using nightly toolchain ([3e3a554])
- Changed: Add metrics placeholder config for prometheus in Vixen.toml ([5bd4fd3])
- Changed: Update READMEs ([30e65a3])

### 2024-09-03
- Changed: Refactor Ix parsers logic to consume InstructionUpdate ([f7a55d2])

### 2024-08-30
- Added: Add missing clippy config to remaining crates ([04b43d6])
- Added: Add support for adapting prebuilt parsers to gRPC ([ad74dee])

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
- Changed: Setup Ixs Parsers (Token program and Token extensions program) #16 ([84c9cc5])
- Changed: Merge pull request #13 from rpcpool/vixen-parsers ([5971be2])
- Changed: Merge branch 'main' into vixen-parsers ([9af4912])
- Changed: Refactor code for mock ([e422188])

### 2024-08-06
- Fixed: Formatting ([4032158])
- Changed: Update cargo.lock ([4c318b5])

### 2024-08-05
- Changed: Update feature flags ([17001a8])
- Changed: Cleanup ([c8589d4])

### 2024-08-01
- Changed: Cleanup ([43f9fbf])
- Fixed: README ([9c524b9])

### 2024-07-31
- Changed: Update mock README ([68025da])
- Fixed: Add env setup in mock README ([a068b7f])
- Added: Add dotenv to fetch RPC_ENDPOINT and CLUSTER ([e4d5152])

### 2024-07-30
- Changed: Cleanup ([11f6925], [575cfb0], [6725f5b], [f41588d])
- Fixed: README ([5cb349e])
- Fixed: Add imports and installation info on readme ([9746dcf])
- Changed: Refactor token extension parsing ([01e26e6])
- Fixed: Feature flags ([3fe77a3])
- Changed: Resolve PR comments ([16243ef])
- Added: Add readme for parser and mock ([6183f63])
- Added: Setup mock testing suite ([30fb5da])

### 2024-07-26
- Added: Setup unit tests for parsers ([305ceaf])
- Added: Add extended account data ([4fa3843])

### 2024-07-24
- Added: Create crate for housing parsers sponsored by vixen ([8b80adb])
- Added: Implement token program parser using SPL token program ([cc17443])
- Changed: Merge pull request #11 from WilfredAlmeida/patch-1 ([5379be7])

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

