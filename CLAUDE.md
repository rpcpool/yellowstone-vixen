# Claude-specific instructions

## Code style

- **Scoped bindings**: Use `{ ... }` blocks to limit variable scope
- **Early returns / guard clauses**: `let...else { return }` and early `match` + `return`
- **Doc comments with examples**: `/// Example output:` with `rust, ignore` code blocks. Blank `///` line at start/end when > 3 lines
- **Breathing room**: Blank lines between logical groups of statements
