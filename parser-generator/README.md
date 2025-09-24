# Parser Generator

## Prerequisite

```bash
pnpm install
```

## Get a right idl

In order to generate parser with codama, you need the latest version of anchor idl.
You may either download the IDL through anchor cli below

```bash
anchor idl fetch --provider.cluster mainnet DCA265Vj8a9CEuX1eb1LWRnDT7uK6q1xMipnNyatn23M
```

or from solscan. Kamino limit order [for example](https://solscan.io/account/LiMoM9rMhrdYrfzUCxQppvxCSG1FcrUK9G8uLq4A1GF#anchorProgramIdl)

After download this idl, you need to check if your idl look like this format with metadata, and instruction discriminator. 

```json
{
  "address": "6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P",
  "metadata": {
    "name": "pump",
    "version": "0.1.0",
    "spec": "0.1.0"
  },
  "instructions": [
    {
      "name": "initialize",
      "discriminator": [
        175,
        175,
```

otherwise, add an empty metadata field and use this command to convert to new version

```bash
anchor idl convert dca.json
```

see this [discussion](https://discord.com/channels/889577356681945098/889584618372734977/1255195564803489873) and [doc](https://docs.rs/anchor-lang-idl/0.1.1/anchor_lang_idl/convert/fn.convert_idl.html) for more detail.

## Update and generate

Updated stream-processor/Cargo.toml to include project name you wrote in script.cjs

```rust
       2    members = [
       3        "event-proto",
       4        "kryptogo-vixen-pumpfun-parser",
       5 +      "kryptogo-vixen-jupiter-dca-parser",
       6        "common-program-parsers",
       7        "stream-processor-bin",
       8        "block-processor",
```

```bash
node IDL_FOLDER_YOU_WANT_TO_GEN_PARSER/script.cjs
```

## Misc

1. Refer to the official doc for [general usage](https://docs.triton.one/project-yellowstone/vixen-data-pipelines/generate-parsers-with-codama)
2. There's a [repo](https://github.com/bitquery/solana-idl-lib) keeping most (mostly) of latest idls. Do cross check with other sources yourself
