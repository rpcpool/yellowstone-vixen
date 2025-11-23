# yellowstone-vixen-proc-macro

This crate provides a procedural macro for generating Yellowstone Vixen parser modules from [Codama JSON IDL](https://github.com/codama-idl/codama).

## Usage

Add this crate to your `Cargo.toml` (usually as a `proc-macro` dependency):

```toml
[dependencies]
borsh = "^1.0.0"
yellowstone-vixen-parser = { version = "0.6.0" }
yellowstone-vixen-proc-macro = { version = "0.6.0" }
```

Then, in your code:

```rust
use yellowstone_vixen_proc_macro::include_vixen_parser;

// Provide the path (relative to your crate root) to your Codama JSON IDL file.
include_vixen_parser!("path/to/idl.json");
```

This macro will generate Rust modules containing type-safe account and instruction parsers for the specified Solana program.
