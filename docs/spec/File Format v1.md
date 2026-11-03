# Everything Knowledge Base File Format - Version 1

**Status:** Draft, normative.

**Reference implementation:** `crates/everything_db`.

**Related documents:** [Chapter 1 - Objects](../public/1%20-%20Objects.md),
[Chapter 2 - Knowledge](../public/2%20-%20Knowledge.md).

This document specifies the byte-level format of a persistent Everything
knowledge base, version 1. It is intended to be sufficient for an independent
implementation to read and write files that this repository's implementation
accepts and to explain every byte of a file produced by it.

**Contents:** [Conformance](#1-conformance-language) · [Terminology](#2-terminology) ·
[Conventions](#3-conventions) · [File layout](#4-file-layout) · [Payload header](#5-payload-header) ·
[Base](#6-base) · [Change log](#7-change-log) · [Object encoding](#8-object-encoding) ·
[Tag definitions](#9-tag-definitions) · [Reserved atoms](#10-reserved-atoms) ·
[Canonical encoding](#11-canonical-encoding) · [Decoding](#12-decoding-requirements) ·
[Errors](#13-errors) · [Lifecycle](#14-file-lifecycle) · [Robustness](#15-robustness) ·
[Versioning](#16-versioning-and-extension) · [Conformance](#17-conformance) ·
[Test vectors](#appendix-b-test-vectors)

## 1. Conformance language

The key words **MUST**, **MUST NOT**, **REQUIRED**, **SHOULD**, **SHOULD NOT**,
**MAY**, and **OPTIONAL** are to be interpreted as described in RFC 2119.

A *decoder* reads a file. An *encoder* (or *writer*) produces one. Where this
document says something is *invalid*, a decoder MUST report an error rather than
recover.

## 2. Terminology

- **Object**: an atom, a construction, or a set — as defined in
  [Chapter 1 - Objects](../public/1%20-%20Objects.md).
- **Atom id**: the `u128` identifying an atom.
- **Statement**: a pair of an atom and an object, written `a is o`.
- **Knowledge base**: a finite map from atoms to non-empty sets of objects, i.e.
  a set of statements that is grouped by subject.
- **Base**: the section of the file that contains the knowledge base as of the
  last full write.
- **Change log**: the section of the file that contains statements to add to or
  remove from the base, in order.
- **Tag**: the leading `u8` of an encoded object that selects its encoding.
- **Canonical encoding**: the unique encoding of a value that satisfies
  [§11](#11-canonical-encoding).

Examples throughout use the object notation of the public documentation. `@n`
denotes the atom with id `n`; `a b` denotes a construction; `{a, b}` denotes a
set. Note that `@0` is an ordinary atom, whereas `@zero` is the distinguished
zero atom with id `2148623498527692884679683682469014909` (see [§9.3](#93-the-zero-atom-tag-2)).

## 3. Conventions

- All fixed-width integers are **little-endian**.
- `u8`, `u32`, `u64`, and `u128` denote unsigned integers of that width.
- **LEB128** is unsigned LEB128: seven value bits per byte, least significant
  group first, bit `0x80` set on every byte except the last.
- Offsets are zero-based file offsets.
- Lengths are counts of **bytes**, never of characters or objects, unless
  explicitly stated otherwise.

## 4. File layout

```text
+--------+---------------------------+---------------------------------------+
| Offset | Size                      | Field                                 |
+--------+---------------------------+---------------------------------------+
| 0      | 28                        | magic bytes                           |
| 28     | 4                         | version (u32), MUST be 1              |
| 32     | 8                         | base length in bytes (u64)            |
| 40     | 24                        | reserved (MUST be zero on write)      |
| 64     | base length               | base                                  |
| 64 + L | rest of file              | change log                            |
+--------+---------------------------+---------------------------------------+
```

`L` is the value of the base length field. A file MUST be at least 64 bytes
long; a file with an empty knowledge base and an empty change log is exactly
64 bytes. Any bytes after the last complete change record are invalid.

The magic bytes occupy 28 bytes: the first 16 are the little-endian encoding of
the `u128` value `2167892790992798108174164308701667773`, and the last 12 are
the ASCII string `EVERYTHINGKB`:

```text
BD E1 A6 7D BB F6 CF 62 2D B7 92 E9 5D 85 A1 01
45 56 45 52 59 54 48 49 4E 47 4B 42
```

A decoder MUST reject a file whose magic bytes differ, whose version is not 1,
which is shorter than 64 bytes, or whose base length exceeds the number of
bytes available after offset 64.

## 5. Payload header

Immediately after the header, at file offset 32, the payload begins with:

| Offset (relative) | Size | Field |
|---:|---:|---|
| 0 | 8 | base length in bytes (`u64`) |
| 8 | 24 | reserved |

`base length` is the exact number of bytes of the base section. The reserved
bytes are not interpreted in version 1. Encoders MUST write zero there;
decoders MUST ignore their contents. They are reserved for future use (see
[§16](#16-versioning-and-extension)).

## 6. Base

The base encodes the knowledge base directly, without a change log.

```text
base  := entry*
entry := atom:u128 values
values := count:leb128-u64 object{count}
```

Each entry contributes the statements `a is o` for every object `o` in its
values. The values of an entry are a set: they are unique and unordered.

Rules:

- The base MUST contain exactly the bytes given by the base length field; a
  base length that does not fall on an entry boundary is invalid.
- An entry whose values are empty contributes no statements. Encoders SHOULD NOT
  emit such entries.
- Atom ids in the base and in statements are always raw `u128` values. The
  special tag 2 applies only to objects, never to a subject.
- Every atom id MUST appear in at most one entry with non-empty values.
  Duplicate entries are invalid (see [§12](#12-decoding-requirements) and
  [Appendix C](#appendix-c-reference-implementation)).
- The order of entries and the order of values within an entry carry no
  meaning. See [§11.3](#113-deterministic-output).

## 7. Change log

The change log is a sequence of change records that are applied, in order, to
the knowledge base read from the base.

```text
change_log := change*
change     := 0x01 statement   ; add
            | 0x02 statement   ; remove
statement  := atom:u128 object
```

Applying a change record `add s` inserts the statement `s`. Applying
`remove s` deletes it. Inserting a statement that is already present and
removing a statement that is not present are no-ops. When the last value of an
atom is removed, the atom no longer occurs in the knowledge base.

The change log is append-only: encoders add records to the end of an existing
file and MUST NOT modify the header or the base. There are no commit markers,
checksums, or length prefixes; the change log extends to the end of the file,
and a partial trailing record is invalid (see [§15](#15-robustness)).

## 8. Object encoding

```text
object := tag:u8 payload
```

An object is always encoded as a tag byte followed by the payload defined for
that tag. The complete tag table is:

| Tag | Name | Payload | Encodes |
|---:|---|---|---|
| 1 | atom | `u128` | the atom with that id |
| 2 | zero atom | *(empty)* | the zero atom |
| 3 | construction | object, object | a construction |
| 4 | character-1 | 1 byte | the character set for a code point |
| 5 | character-2 | 2 bytes | the character set for a code point |
| 6 | character-3 | 3 bytes | the character set for a code point |
| 7 | character-4 | 4 bytes | the character set for a code point |
| 8 | text | LEB128 length, bytes | a text set |
| 9 | *(reserved)* | *(undefined)* | *(none; MUST NOT be used)* |
| 10 | set | LEB128 count, objects | a set |
| 11 | byte | 1 byte | a byte set |
| 12 | positive integer | LEB128 length, bytes | the integer set for a positive integer |
| 13 | negative integer | LEB128 length, bytes | the integer set for a negative integer |
| 14-255 | *(unassigned)* | *(undefined)* | *(none; MUST NOT be used)* |

A decoder MUST reject a tag it does not know, including tag 9 and all tags
above 13.

Tags 4 through 13 encode *specialized representations of sets*, not scalar
values. Their meaning is defined in [§9](#9-tag-definitions).

## 9. Tag definitions

### 9.1. Atom (tag 1)

Payload: an atom id as `u128`.

Encodes `Object::Atom(id)`. Every `u128` is a valid id. An atom that does not
occur as a subject in the represented knowledge base is still a valid object.

The zero atom (see §9.3) MUST NOT be encoded with tag 1 in a canonical encoding,
but a decoder MAY accept that encoding.

### 9.2. Constructions (tag 3)

Payload: the constructor object followed by the value object.

Encodes the construction `constructor value`. Constructions associate to the
right at the level of concrete syntax, but the binary encoding is explicit:
`a (b c)` is encoded as the construction of `a` with the construction of `b`
with `c`.

### 9.3. The zero atom (tag 2)

Payload: none.

Encodes the atom with id `2148623498527692884679683682469014909`. This atom is
the model's representative of the integer zero (see §9.5). It is a normal atom
in every other respect and may be used as any object.

> Note: the zero atom is **not** the atom with id 0. `@0` is an ordinary atom
> and is encoded with tag 1 and payload `00 00 ... 00`.

### 9.4. Sets (tag 10)

Payload: a LEB128 count followed by exactly that many encoded objects.

Encodes a set. Sets are unordered and contain each object at most once.
Decoders MUST accept members in any order and MUST ignore duplicates.

The empty set is encoded as tag 10 followed by a zero count.

A set MUST use the most specific applicable representation:

- the empty set uses tag 10 with count 0;
- a set of the form `{successor_of k}` with an integer `k >= 0` uses tag 12;
- a set of the form `{predecessor_of k}` with an integer `k <= 0` uses tag 13;
- a set of the form `{code_point c}` uses tag 4 through 7;
- a byte set (eight `bit_slot_i` members) uses tag 11;
- a non-empty text set uses tag 8;
- every other set uses tag 10, including a set whose sole member is a
  construction.

Canonical encoders MUST use the most specific tag. Decoders MAY accept the
generic form (tag 10) for a set that has a specialized encoding, but if they do,
they MUST produce the same set as the specialized form. Because sets are
unordered, a decoder MUST also accept the members of a generic set in any order
and MUST ignore duplicates.

### 9.5. Integer sets (tags 12 and 13)

```text
integer := tag:u8 length:leb128-u64 magnitude:byte{length}
```

`magnitude` is the absolute value of the represented integer, little-endian.
Tag 12 represents the positive integer with that magnitude; tag 13 represents
its negative.

The magnitude MUST be nonzero; the length MUST be at least 1. The integer zero
has no integer encoding - it is the zero atom (§9.3). A decoder MUST reject a
magnitude of zero and MUST reject a length of zero.

An integer set with represented value `n > 0` is the set whose only member is
the construction `successor_of k` with `k = n - 1`. For `n < 0` it is the set
whose only member is `predecessor_of k` with `k = n + 1`. In other words, the tag
stores the represented integer, not the construction argument. The argument is
the zero atom when `k` is zero and an integer set otherwise. The reserved atoms
used here are listed in [§10](#10-reserved-atoms).

For example, `1` is encoded as tag 12, length 1, magnitude `01`, and denotes the
set `{successor_of @zero}`. `-1` is encoded as tag 13, length 1, magnitude
`01`, and denotes `{predecessor_of @zero}`.

Canonical encodings use the shortest possible magnitude (no trailing zero byte)
and the shortest LEB128 length. A decoder MAY accept padded magnitudes as long
as the value is nonzero.

### 9.6. Character sets (tags 4 through 7)

```text
character := tag:u8 code_point:byte{tag - 3}
```

The payload is the Unicode code point as a little-endian `u32`, truncated to the
number of bytes selected by the tag. The value MUST be a Unicode scalar value:
`0 <= code_point <= 0x10FFFF` and not in `0xD800..=0xDFFF`.

A character set is the set `{code_point c}` where `c` is the character and
`code_point` is the reserved atom of §10.

Canonical encodings use the shortest tag that can hold the code point: tag 4
for code points below `2^8`, tag 5 below `2^16`, tag 6 below `2^24`, and tag 7
otherwise. In particular, the code point `0` is encoded with tag 4 and the
payload `00`. A decoder MAY accept a wider tag for a code point that would fit
in a narrower one.

Note that a multi-byte code point always has its least significant byte first,
so `U+1F600` is encoded as tag 6 with payload `00 F6 01`.

### 9.7. Byte sets (tag 11)

Payload: one byte, the bit mask of the set.

For each bit position `i` (least significant bit = position 0) the set contains
the construction `bit_slot_i bit_on` if the bit is set and `bit_slot_i bit_off`
if it is clear. The atoms involved are the reserved atoms of §10.

A byte set always has exactly eight members; every mask value is a valid byte
set. There is no ambiguity between tag 11 and tag 10 because a byte set is never
encoded as a generic set in a canonical encoding.

### 9.8. Text sets (tag 8)

```text
text := tag:u8 length:leb128-u64 utf8:byte{length}
```

The payload is a byte length followed by that many bytes of UTF-8. The length
MUST be at least 1, and the bytes MUST be valid UTF-8; an empty text set does
not exist. The length is a byte count, not a character count.

A text set is the virtual set whose members are the two list cell constructions:

```text
text("c s") = {list_item c, list_tail text(s)}   ; s is non-empty
text("c")   = {list_item c, list_tail {}}        ; the tail is the empty set
```

where `list_item` and `list_tail` are the reserved atoms of §10 and `c` is the
first character of the text. The tail is the text set of the remaining
characters, or the empty set once no characters remain. Thus every non-empty
text set has exactly two members and there is exactly one text set per non-empty
UTF-8 string.

### 9.9. Reserved tag 9

Tag 9 is reserved and has no defined payload. It MUST NOT appear in any version 1
file, and a decoder MUST reject it. Its assignment is reserved for a future
version (see §16).

## 10. Reserved atoms

The specialized encodings of §9 are defined in terms of the following atoms.
Their ids are part of this format; changing them would require a new format
version.

| Name | Id | Used by |
|---|---:|---|
| `zero` | 2148623498527692884679683682469014909 | zero atom, integer sets |
| `successor_of` | 2148623541928107461725235547620677496 | positive integer sets |
| `predecessor_of` | 2150448436013648618415526635010123333 | negative integer sets |
| `list_item` | 2148623561006058368074852138222840554 | text sets |
| `list_tail` | 2148623576528286069034804554905263388 | text sets |
| `code_point` | 2148623592788974381730175331142334217 | character sets |
| `bit_off` | 2148623606609578531456976446246481504 | byte sets |
| `bit_on` | 2148623648901638714578593280489697845 | byte sets |
| `bit_slot_0` | 2148623657764931307024391951942698252 | byte sets |
| `bit_slot_1` | 2148623779289647569021414352435474390 | byte sets |
| `bit_slot_2` | 2148623793297509473303838197251962004 | byte sets |
| `bit_slot_3` | 2148623803164736660711697141897226232 | byte sets |
| `bit_slot_4` | 2148623819303808917979289470225598602 | byte sets |
| `bit_slot_5` | 2148623826522742854507738117057019657 | byte sets |
| `bit_slot_6` | 2148623834321554672547365012703956257 | byte sets |
| `bit_slot_7` | 2148623841265101367555509279336886325 | byte sets |

These atoms MAY also occur in ordinary statements; the encoding does not
distinguish their role.

## 11. Canonical encoding

A canonical encoding is one that satisfies every rule in this section. Encoders
MUST produce canonical encodings except where §11.3 explicitly allows a choice.

### 11.1. Structural rules

1. The magic bytes and version are exactly as specified in §4.
2. The 24 reserved bytes are zero.
3. Every LEB128 integer uses its shortest encoding.
4. Every atom is encoded with tag 1, except the zero atom, which is encoded with
   tag 2.
5. Every set uses the most specific tag of §9.4.
6. Base entries have non-empty values, and no atom has more than one entry.
7. Every integer magnitude is nonzero and has no trailing zero byte; lengths are
   minimal and at least 1.
8. Every character uses the shortest tag that holds its code point.
9. Every text set is non-empty and encoded as a byte length plus valid UTF-8.
10. The change log contains only complete records; a full write produces an
    empty change log.

### 11.2. Semantics are order-independent

The order of base entries, the order of values within an entry, and the order of
members within a set are not part of the format. Two files that differ only in
those orders denote the same knowledge base, and a decoder MUST produce the same
result for both.

### 11.3. Deterministic output

An encoder SHOULD emit deterministic output so that rewriting the same knowledge
base yields the same bytes. Any deterministic order is acceptable; the format
does not require a particular one. The reference implementation orders base
entries by ascending atom id and orders values and set members by its structural
object order.

Deterministic output is a SHOULD, not a MUST: files MUST NOT be compared
byte-for-byte across independent implementations to decide semantic equality.

## 12. Decoding requirements

A decoder MUST accept every canonical encoding.

A decoder MUST reject:

- a file shorter than 64 bytes;
- incorrect magic bytes or a version other than 1;
- a base length that exceeds the available bytes, or a base that is not an
  exact sequence of entries;
- an unknown object tag, including tag 9 and tags above 13;
- a truncated object, statement, or change record;
- an invalid UTF-8 text set, or an empty text set;
- an integer magnitude of zero or a length of zero;
- a code point that is not a Unicode scalar value;
- a knowledge base with more than one non-empty base entry for the same atom;
- any change record whose kind byte is neither `0x01` nor `0x02`;
- trailing bytes that do not form a complete change record.

A decoder MAY additionally accept the following non-canonical encodings, in
which case it MUST interpret them as the canonical value:

- non-minimal LEB128 encodings of up to 10 bytes;
- integer magnitudes with leading (high-order) zero bytes;
- character payloads wider than necessary;
- the zero atom encoded with tag 1;
- a generic set (tag 10) for a set that has a specialized encoding;
- a base entry with zero values;
- non-zero contents in the reserved bytes.

A decoder MUST NOT accept an encoding that has no unambiguous canonical value.
In particular, a file with more than one non-empty base entry for an atom is
ambiguous and is invalid, not merely non-canonical.

## 13. Errors

The conditions of §12 map to errors as follows in the reference implementation.
This table is informative; the exact error names are not part of the format.

| Condition | Reference error |
|---|---|
| file shorter than the 32-byte header | `ContentIsTooShortForHeader` |
| incorrect magic bytes | `InvalidMagicBytes` |
| version other than 1 | `InvalidVersion` |
| payload shorter than 32 bytes | `ContentIsTooShortForV1` |
| base length larger than available bytes | `ContentIsTooShortForV1` |
| truncated atom id in the base | `ContentIsTooShortForAtom` |
| unknown object tag | `InvalidObjectType` |
| truncated object | `ContentIsTooShortForObject` |
| invalid code point | `InvalidCharacter` |
| invalid LEB128 set count | `InvalidSetValueCount` |
| invalid LEB128 text length | `InvalidTextLength` |
| truncated text | `ContentIsTooShortForText` |
| invalid UTF-8 text | `InvalidText` |
| empty text | `EmptyText` |
| truncated change kind | `ContentIsTooShortForStatementChange` |
| unknown change kind | `InvalidStatementChange` |
| invalid LEB128 integer length | `InvalidIntegerLength` |
| truncated integer magnitude | `ContentTooShortForInteger` |
| zero integer magnitude | `InvalidIntegerEncodingFound0` |

## 14. File lifecycle

The format is designed for append-only updates:

- A **full write** produces a header, a base containing the whole knowledge
  base, and an empty change log.
- An **incremental update** appends change records to the end of the file. The
  header, the base, and earlier change records are never modified.
- **Compaction** rewrites the whole file from the current knowledge base,
  producing a fresh base and an empty change log.

The reference implementation compacts by writing `<path>.compacting` and
renaming it over the database file. The format itself does not prescribe how a
file is created, replaced, or synchronized; see §15.

## 15. Robustness

- There are no checksums or signatures. Corruption is only detected when it
  breaks the structure; a corrupted magnitude or atom id that remains
  well-formed is silently accepted. Applications that need integrity guarantees
  must add them outside this format.
- Object encoding is recursive. An implementation MUST NOT fail by exhausting
  the stack on a syntactically valid file: it must either handle arbitrary
  nesting or reject excessively deep nesting with a parse error. Implementations
  SHOULD support at least 64 levels of nesting.
- Lengths and counts are bounded only by `u64` and by the file size.
  Implementations MUST report values that do not fit their address space as
  errors rather than truncating them.
- A partially appended change record makes the file unreadable by a strict
  decoder. Writers should encode a whole update before appending it and should
  ensure that the update reaches stable storage before relying on it.
- The format has no locking or concurrency control. A file MUST NOT be modified
  concurrently with a read or another write, except that a reader may work on a
  snapshot. The reference implementation assumes a single writer.

## 16. Versioning and extension

- The version field is a `u32`. Version 1 is the only version defined by this
  document. A decoder MUST reject any other version.
- The 24 reserved bytes are available for future extensions that do not change
  the meaning of the rest of the file. Version 1 decoders MUST ignore them.
- Tags 14 through 255 are unassigned. A version 1 decoder MUST reject unknown
  tags; new tags require a new version unless this document is revised.
- Tag 9 is reserved and MUST NOT be used by version 1 encoders.
- Reserved atom ids are part of the format. Adding new specialized
  representations that depend on new atoms still requires a new version,
  because version 1 decoders reject their tags.

## 17. Conformance

A conforming decoder:

1. reads the header and payload header of §4 and §5;
2. reads the base into a map from atoms to sets of objects;
3. applies every change record in order;
4. accepts every canonical encoding and everything permitted by §12;
5. rejects everything required to be rejected by §12.

A conforming encoder:

1. emits the header of §4 exactly;
2. emits a canonical base (§11);
3. emits change records only in the form of §7;
4. uses tags and payloads only as defined in §8 and §9.

The test vectors in [Appendix B](#appendix-b-test-vectors) are suitable for
checking both directions. Because ordering is not normative, an encoder is
conforming if its output decodes to the same knowledge base as the
corresponding vector.

## Appendix A. Quick reference

```text
file       := magic[28] version:u32le base_length:u64le reserved[24]
              base[base_length] change_log
magic      := BDE1A67DBBF6CF622DB792E95D85A10145564552595448494E474B42
version    := 1
base       := entry*
entry      := atom:u128le values
values     := leb128-u64 object{count}
change_log := change*
change     := 0x01 statement | 0x02 statement
statement  := atom:u128le object
object     := tag:u8 payload
atom       := u128le
leb128-u64 := 1..=10 bytes, least significant group first, 0x80 = continuation
```

| Tag | Payload |
|---:|---|
| 1 | `u128` atom id |
| 2 | none |
| 3 | object, object |
| 4 | 1-byte code point |
| 5 | 2-byte code point |
| 6 | 3-byte code point |
| 7 | 4-byte code point |
| 8 | `leb128-u64` byte length, UTF-8 bytes |
| 9 | reserved - MUST NOT appear |
| 10 | `leb128-u64` count, count objects |
| 11 | 1-byte bit mask |
| 12 | `leb128-u64` magnitude length, magnitude |
| 13 | `leb128-u64` magnitude length, magnitude |
| 14-255 | unassigned - MUST NOT appear |

## Appendix B. Test vectors

Every vector below shows only the **base** region, i.e. the bytes that start at
file offset 64. Prefix each one with the 32-byte file header of §4 and the
32-byte payload header of §5, setting the base length field to the length given
in the third column. All vectors have an empty change log unless stated
otherwise.

The empty knowledge base is the single vector with an empty base. For
reference, the complete empty file (64 bytes) is:

```text
00000000  BD E1 A6 7D BB F6 CF 62 2D B7 92 E9 5D 85 A1 01
00000010  45 56 45 52 59 54 48 49 4E 47 4B 42 01 00 00 00
00000020  00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
00000030  00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
```

| Knowledge | Base bytes | Length |
|---|---|---:|
| *(empty)* | *(empty)* | 0 |
| `@1 is @2` | `01000000000000000000000000000000 01 01 02000000000000000000000000000000` | 34 |
| `@1 is 1` | `01000000000000000000000000000000 01 0C 01 01` | 20 |
| `@1 is -1` | `01000000000000000000000000000000 01 0D 01 01` | 20 |
| `@1 is 128` | `01000000000000000000000000000000 01 0C 01 80` | 20 |
| `@1 is 'A'` | `01000000000000000000000000000000 01 04 41` | 19 |
| `@1 is U+1F600` | `01000000000000000000000000000000 01 06 00 F6 01` | 21 |
| `@1 is "hi"` | `01000000000000000000000000000000 01 08 02 68 69` | 21 |
| `@1 is x2A` | `01000000000000000000000000000000 01 0B 2A` | 19 |
| `@1 is {@2}` | `01000000000000000000000000000000 01 0A 01 01 02000000000000000000000000000000` | 36 |
| `@1 is {2 3}` | `01000000000000000000000000000000 01 0A 01 03 01 02000000000000000000000000000000 01 03000000000000000000000000000000` | 54 |
| `@1 is {2 3, 4 5}` | `01000000000000000000000000000000 01 0A 02 03 01 02000000000000000000000000000000 01 03000000000000000000000000000000 03 01 04000000000000000000000000000000 01 05000000000000000000000000000000` | 89 |

Spaces inside the byte strings are only for readability. `x2A` denotes the byte
set with mask `0x2A`; `U+1F600` denotes the character with that code point.

Notes on individual vectors:

- `@1 is 1` demonstrates that integers are sets: tag 12 stores the represented
  value `1`, from which the single member `{successor_of @zero}` follows.
- `@1 is "hi"` encodes two characters. The set has the two members
  `{list_item 'h'}` and `{list_tail "i"}`, the latter being another text set
  with the members `{list_item 'i'}` and `{list_tail {}}`.
- `@1 is x2A` encodes the byte set with bits 1, 3, and 5 set.
- `@1 is {@2}` is a generic set with one atom member; it cannot use a
  specialized tag.

### B.1. A change log

Starting from a database whose base is `@1 is @2` (34 bytes), appending

```text
@3 is @4          (add)
@1 is @2          (remove)
```

produces the change log

```text
01 03000000000000000000000000000000 01 04000000000000000000000000000000
02 01000000000000000000000000000000 01 02000000000000000000000000000000
```

for a total file size of `64 + 34 + 34 + 34 = 166` bytes. After applying the
log, the knowledge base contains only `@3 is @4`.

### B.2. A non-canonical encoding

The statement `@1 is @zero` is canonically encoded as

```text
01000000000000000000000000000000 01 02
```

The same statement may also be written with tag 1 and the raw id of the zero
atom:

```text
01000000000000000000000000000000 01 01 7DED01929488CF2963F7993F51CF9D01
```

A decoder MAY accept the second form (§12); an encoder MUST NOT produce it
(§11.1, rule 4).

## Appendix C. Reference implementation

This appendix is informative. It maps the format to the reference
implementation in `crates/everything_db` and lists where the implementation
does not yet satisfy this specification.

| Format concept | Reference type |
|---|---|
| atom | `everything::model::Atom` |
| object | `everything::model::Object` |
| construction | `everything::model::Construction` |
| set | `everything::model::Set` and its specializations |
| statement | `everything::knowledge::Statement` |
| knowledge base | `everything::knowledge::Knowledge` (`OrdMap<Atom, Values>`) |
| base entry | one `(Atom, Values)` pair of the map |
| change record | `StatementChange::Add` / `StatementChange::Remove` |
| specialized set | `Set::Integer`, `Set::Character`, `Set::Byte`, `Set::Text`, `Set::OneConstruction`, `Set::Any` |

Known deviations of the reference implementation from this document:

1. **Tag 9 panics.** `parse_object` calls `todo!()` for tag 9 instead of
   reporting an error. This document requires rejection.
2. **Duplicate base entries are accepted.** `collect_base` inserts each entry
   into the map; a later entry for the same atom silently replaces the earlier
   one. This document declares such a file invalid.
3. **No nesting limit.** Object parsing is directly recursive, so a hostile file
   can exhaust the stack instead of producing a parse error, contrary to §15.
4. **32-bit targets are unsupported.** `u64_to_usize` is only defined for
   `target_pointer_width = "64"`, so the crate does not build for 32-bit
   targets. The format does not require 32-bit support, but an implementation
   that targets them must check length conversions itself.
5. **Change-log partial writes.** The database layer encodes a whole update
   before appending it, but a crash during the append can still leave a partial
   trailing record. This is permitted by §15 but makes the file unreadable.

The following behaviours are conforming under §12 and are therefore not
deviations: non-minimal LEB128 encodings, padded integer magnitudes, wider
character tags, tag 1 carrying the zero atom, generic sets with one member,
empty base entries, ignored reserved bytes, and any ordering choice.

## Appendix D. Revision history

| Date | Change |
|---|---|
| 2026-11-03 | Initial draft of version 1. |
