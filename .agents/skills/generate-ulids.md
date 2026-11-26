---
name: Generate ULIDs
description: Generate ULIDs for Everything by running the generate_ulids crate. Use when new atom ids or other unique identifiers are needed.
---

# Generate ULIDs

ULIDs are the repository convention for unique identifiers, such as global atom
ids in `crates/everything/src/model/atoms/mod.rs`. Generate them with the
`generate_ulids` crate instead of inventing ids by hand.

## Usage

From the workspace root, run:

```bash
cargo run -p generate_ulids
```

The command prints ten fresh ULIDs as decimal `u128` values, one per line. Pick
as many as you need. Run it again if you need more.

Use the identifiers exactly as printed. Do not reuse an existing id, and do not
edit a generated id (for example, to avoid collisions); generate another one
instead.

## Related

See the [Node Management](node-management.md) skill for where new atom ids must
be declared and documented.
