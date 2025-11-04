# Generate a Yellowstone Vixen Parser with Codama
### How-to generate Vixen parser with Codama

This guide walks you through generating a [Vixen](https://github.com/rpcpool/yellowstone-vixen) Parser using [Codama](https://github.com/abklabs/codama), a tool for rendering Rust SDKs and parser implementations from IDLs.

Vixen is a framework for building real-time program data pipelines in Rust. This guide helps you scaffold a parser that can be used in the Vixen runtime to decode and process Solana program data.

## ✅ Prerequisites

1. **You must have an idl.json file—either an Anchor-generated IDL or a custom one.**

2. **Install [pnpm](https://pnpm.io/) (or use npm/yarn if preferred).**

3. **Initialize a JavaScript Project (for Codegen)**

    From within the parser directory (where the `idl.json` file is located) run:

    ```bash
    pnpm init
    ```

## 📦 Installation
Install the required Codama packages:

```bash
pnpm install @codama/renderers-vixen-parser
```

For the parser generation script, you’ll also need:

`{ "savq/melange-nvim" }``bash
pnpm install \
  @codama/nodes \
  @codama/nodes-from-anchor \
  @codama/renderers-core \
  @codama/visitors-core
```

## 🛠 Setup

**1. Create a Parser Generation Script**

In the same directory create a new file called `codama.cjs`:

```javascript
// codama.cjs
const path = require("node:path");
const { rootNode } = require("@codama/nodes");
const { rootNodeFromAnchor } = require("@codama/nodes-from-anchor");
const { readJson } = require("@codama/renderers-core");
const { visit } = require("@codama/visitors-core");
const { renderVisitor } = require("@codama/renderers-vixen-parser");

const projectName = "example-parser";
const idl = readJson(path.join(__dirname, "idl.json"));

// Use the appropriate node constructor based on your IDL type:
const node = rootNodeFromAnchor(idl); // for Anchor/Shank idls
// const node = rootNode(idl.program); // for Codama idls

visit(
    node,
    renderVisitor({
        projectFolder: __dirname,
        projectName,
    }),
);
```

> 💡 Tip: The `projectName` is going to be used for the cargo crate name of the generated parser

**2. Run the Code Generation Script**

```bash
node codama.cjs
```
Your folder structure should look like this:
```bash
example-parser/
├── proto/
│  └── example_parser.proto
├── src/
│  ├── generated_parser/  # Vixen parser logic
│  │  ├── accounts_parser.rs
│  │  ├── instructions_parser.rs
│  │  ├── mod.rs
│  │  └── proto_helpers.rs
│  ├── generated_sdk/  # Program sdk client logic
│  │  ├── accounts/
│  │  ├── instructions/
│  │  ├── types/
│  │  ├── ...
│  └── lib.rs
├── build.rs
├── Cargo.toml
├── codama.cjs
└── idl.json
```

**3. Build and Verify**
Before building you project, ensure there is a const export of the program address in `generated_sdk/programs.rs`:

```rust
pub const DCA_ID: Pubkey = pubkey!("DCA265Vj8a9CEuX1eb1LWRnDT7uK6q1xMipnNyatn23M");

```bash
cargo build
```
If successful, you now have a working parser for Solana account data using Yellowstone Vixen.

## 🎉 You’re Done!
You’ve successfully generated a custom Vixen parser. It can now be fully integrated into a Vixen pipeline for parsing and handling account state or instructions updates from your Solana program, or be used with the Vixen streams gRPC server.

## 🧠 Notes
- Codama enables reproducible parser generation from your program’s IDL. Any time your program updates, just re-run the script.

- Generated code is idiomatic Rust and integrates directly with yellowstone-vixen-core.

- Parsers are composable and can be used in a source → parser → sink pipeline for high-throughput indexing.
