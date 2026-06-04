# Everything Structures Text File Format Specification

This format allows for storing **one** structure on disk. The file should have the ending `.evts` but there are magic bytes at the start to identify it if that extension is ever lost.

## Motivation

The previous format (`.struct` files) was not good. This new format is designed to be human

* -readable,
* -editable,
* Git-friendly, but still
* scalable.

## Format

In this specification, string literals are used with escape codes for non-printable characters, like `"\n"` for the ASCII line feed (LF) character.

Every file is UTF-8 and must start with `"EVERYTHINGTS001\n"`.
Then a list of statements follows. Each statement is delimitered by a LF.

## Statements

Statements may contain LF characters. Statements are indexed and other statements may refer to previous ones. There must be a last statement. This last statement is the root structure/knowledge stored. A statement may only refer to statements beforehand. A statement is one of those:

### Any Structure

An _Any Structure_ is denoted with `"A"`, followed by the number of properties this structure will have. Then the properties follow.

A property is encoded in the format `"\n<<OBJECT>>:<<OBJECT>>"`.

### Inline Text

A text structure that is stored inline. Begins with `"T"`, followed by the number of BYTES in UTF-8 this text has.

## Objects

### Abstract Objects

They are encoded with `@` and then a number.

### Structure References

They are encoded using `R` and then an index of a previous structure.

### Empty Structure

`E`

### Characters

`C<<char>>`

### Integers

Just integers, also negative.
