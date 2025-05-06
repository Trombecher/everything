# Meta

_Meta_ is Everything's global config. It contains important information including the object id sequence counter.
_Meta_ is stored in a file called "meta" in the root folder. It is always 4KiB, so it can be memory mapped.

## List Of Config Entries

| Offset | Name              | Datatype       | Description                                |
|--------|-------------------|----------------|--------------------------------------------|
| 0      | Magic Bytes       | `EVERYTHINGKB` |                                            |
| 12     | Version           | `u32`          | Version of the data.                       |
| 16     | Object Sequence   | `u64`          | The current value is the next object id.   |
| 24     | Resource Sequence | `u64`          | The current value is the next resource id. |
| 32     | Features          |                |                                            |