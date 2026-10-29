# Agents.md

_Everything_ is a database for storing arbitrary knowledge. It uses a highly flexible and reflective data model paired with a functional computation system.

## Documentation

Docs are found in `docs/public`.

### Design

Many design decisions have been discussed frequently in `docs/design`. Here you may find answers to why things are how they are (particulary in the object model).

## Code Style

### Testing

Do not write "inline" `mod tests { ... }` test modules within the source file of the code to be tested. Tests shall be written in their own `tests.rs` file **with the parent module being the code to test**. To accomplish this, you may need to move the module to be tested, e.g. `<module>.rs`, to its own folder, i.e. into `<module>/mod.rs`, with the test module living at `module/tests.rs`.

The test module shall be declared as `#[cfg(test)]` in `<module>/mod.rs`.
