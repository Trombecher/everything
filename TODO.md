# TODO

There are some optimizations to be made with objects. Currently, objects are enums with 32 bytes size but 16 bytes are just for the discriminant which is a waste of memory (half, specifically). Therefore, we can use some unsafe code to encode an object into 16 bytes with this layout:

| Variant(s)           | Encoding                                            |
|----------------------|-----------------------------------------------------|
| Inline abstract id   | `0 0 [126 bits of id]`                              |
| Reserved             | `0 1 [126 bits reserved]`                           |
| Universal structure  | `1 0...0 [4 byte prop count] [4 or 8 byte pointer]` |
| Inline bytes         | `1 0 0 0 [4 bit byte len] [15 inline bytes]`        |
| Inline string        | `1 0 0 1 [4 bit byte len] [15 inline bytes]`        |
| Reserved             | ...                                                 |
| Nat structure inline | `1 1 [126 bits of natural number]`                  |