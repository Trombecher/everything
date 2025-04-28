# Encoding

Associations and values need to be serialized to be saved to disk.

Because this knowledge base is basically a big set of three-tuples (target, tag, value),
such a tuple can be stored efficiently in 4x8 bytes.

## Encoding Values

Values are encoded with a fixed size of 2x8 bytes. Variable length data in text, URLs, binary, etc.
is sometimes offloaded to a separate file, called a _resource_. This happens when the length exceeds 15 bytes
or the user tries to access the file with a program.

## Indexing

It is not viable to store the whole database as a list of three-tuples. In theory there are six indices possible:

1. `Target        -> Tag, Value`
2. `Tag           -> Target, Value`
3. `Target, Tag   -> Value`
4. `Value         -> Tag, Target`
5. `Tag, Value    -> Target`
6. `Target, Value -> Tag`

(ranked by their importance).

You would need to maintain six + one rows for each association.

Instead, what we can do is drop the list and maintain indices one and two. Searching for values would be an O(n)
operation, but to be realistic: who searches for all associations including a number?
The third index is also essential but can be emulated by an O(n) search through the first index because
objects tend to have less than 200 associations, but tags may have millions.

### Size Impact

Let's say the user tries to store 10,000 images in Everything, each 1MB with 50 associations.

With three-tuples this would be 10,000 x 50 x 4 x 8 = 16MB in associations, and the 10,000 x 1MB = 10GB in image data.
This is a ration of 0.16% of association bytes to image bytes.

With two primary indices, associations would be 2 x 10,000 x 50 x 3 x 8 = 24MB in associations, and the 10GB image data.
This method would only add 8MB duplicate data with a ratio of 0.24% of association bytes to image bytes.
But the performance impact is noticeable.

## Target And Tag Rows

For each target there exists a list of pairs (tag, value). We can delegate hashing to the OS FS layer and
allocate a file for each target. This file would be a list of 3x8 byte slots. Same with tags.

The end of the file

When slots (=associations) are deleted, they are linked into a free-list.

```
(0: u64, x: u64, any: u64) -> a sentinel, pointing to the next free slot index
(t: u64, TEXT_MAX: u8, text: [u8; 15]) -> 15 bytes of inlined text
(t: u64, TEXT_RES: u8, 0: [u8; 7], r: u64) -> external text resource
(t: u64, TEXT: u8, len: 0..15, text: [u8; len], ...any) -> inlined text
```