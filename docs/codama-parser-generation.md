# Generate a Shipstern Parser with Codama
### How-to generate Shipstern parser with Codama

This guide walks you through generating a [Shipstern](https://github.com/solana-rpc/shipstern) Parser using [Codama](https://github.com/abklabs/codama), a tool for rendering Rust SDKs and parser implementations from IDLs.

Shipstern is a framework for building real-time program data pipelines in Rust. This guide helps you scaffold a parser that can be used in the Shipstern runtime to decode and process Solana program data.

## ✅ Prerequisites

1. **You must have an idl.json file—either an Anchor-generated IDL or a custom one.**

2. **Install [pnpm](https://pnpm.io/) (or use npm/yarn if preferred).**

3. **Initialize a JavaScript Project (for Codegen)**

    From within the parser directory (where the `idl.json` file is located), run:

    ```bash
    pnpm init
    ```

## 📦 Installation
Install the required Codama packages:

```bash
pnpm install @codama/renderers-shipstern-parser
```

For the parser generation script, you’ll also need:

```bash
pnpm install \
  @codama/nodes \
  @codama/nodes-from-anchor \
  @codama/renderers-core \
  @codama/visitors-core
```

## 🛠 Setup

**1. Create a Parser Generation Script**

In the same directory, create a new file called `codama.cjs`:

```javascript
// codama.cjs
const path = require("node:path");
const { rootNode } = require("@codama/nodes");
const { rootNodeFromAnchor } = require("@codama/nodes-from-anchor");
const { readJson } = require("@codama/renderers-core");
const { visit } = require("@codama/visitors-core");
const { renderVisitor } = require("@codama/renderers-shipstern-parser");

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
│  ├── generated_parser/  # Shipstern parser logic
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
Before building your project, ensure there is a const export of the program address in `generated_sdk/programs.rs`:

```rust
pub const DCA_ID: Pubkey = pubkey!("DCA265Vj8a9CEuX1eb1LWRnDT7uK6q1xMipnNyatn23M");
```

```bash
cargo build
```

If successful, you now have a working parser for Solana account data using Shipstern.

## 🎉 You’re Done!
You’ve successfully generated a custom Shipstern parser. It can now be fully integrated into a Shipstern pipeline for parsing and handling account state or instructions updates from your Solana program, or be used with the Shipstern streams gRPC server.

## 🧠 Notes
- Codama enables reproducible parser generation from your program’s IDL. Any time your program updates, just re-run the script.

- Generated code is idiomatic Rust and integrates directly with shipstern-core.

- Parsers are composable and can be used in a source → parser → sink pipeline for high-throughput indexing.

## Self-CPI events (`emit_cpi!`)

`emit_cpi!` does not write a log line. It invokes the program from itself, with
instruction data shaped as:

```
[envelope tag][event discriminator][borsh payload]
```

Codama has no node for that tag, so Shipstern infers it positionally from the
event's discriminator chain, the same rule the Carbon renderer uses. Declare two
or more `constantDiscriminatorNode`s on the event:

```json
"discriminators": [
  { "kind": "constantDiscriminatorNode", "offset": 0,
    "constant": { "kind": "constantValueNode",
      "type": { "kind": "fixedSizeTypeNode", "size": 8, "type": { "kind": "bytesTypeNode" } },
      "value": { "kind": "bytesValueNode", "data": "e445a52e51cb9a1d", "encoding": "base16" } } },
  { "kind": "constantDiscriminatorNode", "offset": 8,
    "constant": { "kind": "constantValueNode",
      "type": { "kind": "fixedSizeTypeNode", "size": 8, "type": { "kind": "bytesTypeNode" } },
      "value": { "kind": "bytesValueNode", "data": "40c6cde8260871e2", "encoding": "base16" } } }
]
```

The rules:

- The constant at **offset 0** is the envelope tag.
- Every remaining constant forms the event's own discriminator.
- `payload_offset` is `sorted[1].offset`, the declared offset of the next
  discriminator, not the tag's length. Padded layouts therefore round-trip, and
  discriminators need not be contiguous.
- One program has one envelope. Events that declare one must agree on both the
  tag bytes and the payload offset, or the build fails.
- An event with a single discriminator declares no envelope. It keeps matching
  at offset 0 and stays reachable from `emit!` log lines.
- When no event declares an envelope, Anchor's default 8-byte tag is used.

`e445a52e51cb9a1d` is the little-endian serialisation of
`sha256("anchor:event")[..8]`, whose natural byte order is `1d9acb512ea545e4`.
Write the wire bytes, not the digest prefix.

### The envelope must not mask an instruction

The generated parser filters any instruction whose data starts with the envelope
tag before it dispatches, so an instruction whose discriminator shares a prefix
with the tag at offset 0 could never be parsed. That is rejected at build time:

    error: Invalid CPI event envelope in "...": CPI event envelope 09 collides
    with instruction `swapBig` (discriminator 09 at offset 0); the generated
    parser filters every instruction whose data starts with the envelope tag, so
    `swapBig` would never parse

The check runs against the tag the parser actually uses, so it covers an IDL
envelope, the `cpi_event_discriminator` fallback, and the Anchor default alike.
It compares the tag only against instructions. Instruction variants that
deliberately share a discriminator with each other, resolved at runtime by
account count or a `CustomInstructionParser`, are unaffected.

Note this differs from Anchor, whose dispatcher checks the event-CPI sentinel
after every instruction rather than before.

### Deliberate divergences from Carbon

Both follow from Shipstern supporting `emit!` log events, which Carbon's
generated decoders do not.

1. **Mixed programs are accepted.** Carbon bails on the whole program if any
   single event lacks the two-discriminator form; Shipstern accepts a program
   that mixes `emit_cpi!` and `emit!` events. An IDL Shipstern accepts may
   therefore render nothing in Carbon.
2. **A lone two-part discriminator reads as enveloped.** The heuristic is
   positional and no marker node exists, so an event with a genuine two-part
   discriminator starting at offset 0 is indistinguishable from
   envelope-plus-discriminator, and is treated as the latter.

### Macro arguments are deprecated

`cpi_event_discriminator` and `cpi_event_payload_offset` on
`include_shipstern_parser!` remain as a fallback for IDLs that declare no
envelope. When the IDL declares one it wins and the arguments are ignored, with
a deprecation warning at the call site. Both are removed in 0.10; move the
envelope into the IDL.
