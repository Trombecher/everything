> [!WARNING]
> This project is still in early development.

# Everything

_Everything_ defines a universal data format, along with an interpretation that allows for storing abstract knowledge and arbitrary, structured data. It can be used as knowledge bases.

This project contains a few crates. To interact with Everything, check out the [cli crate](./crates/everything_cli/). You can run the cli from top level with:

```sh
cargo run --release -p everything_cli
```

You can provide options by appending `--` to the command. For an introduction to the data format, as well as the interpretation, read [`INTRODUCTION.md`](./INTRODUCTION.md).