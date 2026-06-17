# Everything Structures

This crate provides an implementation of abstract objects and structures.

## Optimization For Some Structures

Some structures are stored in an optimized way:

| Name                              | Structure (Form)                                                                | Internal Storage Type |
|-----------------------------------|---------------------------------------------------------------------------------|-----------------------|
| Natural numbers                   | `@ZERO`, `{(@SUCCESSOR_OF, @ZERO)}`, ...                                        | `u128`                |
| Text (list of unicode characters) | `@EMPTY_LIST`, `{(@LIST_ITEM, <unicode char>), (@LIST_TAIL, @LIST_EMPTY)}`, ... | `str`                 |
| Binary data (list of bytes)       | `@EMPTY_LIST`, `{(@LIST_ITEM, <byte>), (@LIST_TAIL, @LIST_EMPTY)}`, ...         | `[u8]`                |
