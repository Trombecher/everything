# Everything Text Database File Format Specification

This document describes the file format for Everything text databases. Files should have the ending `.evtdb` but there are also magic bytes at the start of the file to identify it if that extension is ever lost.

## Prelude

In this format, string literals are enclosed by quotes `"..."` and contain text. These escape codes used in this specification:

- `\"` -> `"`
- `\\` -> `\\`
- `\n` -> U+000A (LF)
- `\r` -> U+000D (CR)
- `\0` -> U+0000

A _line break_ is either `"\n"` or `"\r\n"`.

## Structure

A EVTDB file is an UTF-8 encoded text file and must start with the string `"EVERYTHINGTEXTDB00000002"`. Then a set of statements follows. The file may contain a trailing line (break).

## Statements

Each statement is preceeded by a line break. Then an `S` follows.

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
