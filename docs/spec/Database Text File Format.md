# Everything Text Database File Format Specification

This format allows for storing **one** object on disk. The file should have the ending `.evtdb` but there are magic bytes at the start to identify it if that extension is ever lost.

## Motivation

This format is designed to be human

* -readable,
* -editable (well, if you know what you're doing), and
* Git-friendly, but still
* scalable.

## Format

In this specification, string literals are used with escape codes for non-printable characters, like `"\n"` for the ASCII line feed (LF) character.

Every file is UTF-8 and must start with `"EVERYTHINGTEXTDATABASE01"`. Then a list of statements follows.

## Statements

A statement is a triple

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

## BNF

```
digit = "0" | ... | "9"
lowercase = "a" | ... | "z"
uppercase = "A" | ... | "Z"
line_break = "\n" | "\r\n"

base64_character = lowercase | uppercase | digit | "-" | "_"

abstract_object = "@" digit*22
empty_composite_object = "E"
arbitrary_composite_object = "(" (object ":" object)*_ ")"
text_composite_object = "\""  "\""
binary_composite_object = "<" base64_character*_ ">"
integer_composite_object = "-"? digit*1:32

object = abstract_object
    | empty_composite_object
    | arbitrary_composite_object
    | text_composite_object
    | binary_composite_object

statement = line_break abstract_object "," object "," object

header = "EVERYTHINGTEXTDATABASE01"

database = header statement*_ line_break?
```
