# Chapter 4 - Common Models

Everything gives you the freedom to model anything. However, some models are stored optimally in memory, which may be desirable. This chapter presents common models that are used throughout Everything or that are useful as a starting point.

## Sets

The atom `contains` is introduced by this model and is not predefined in `atoms.rs`. It is a property constructor whose constraint accepts every value for every subject.

An object is a _set_ iff it is a composite whose members are all constructions of the form `contains ·· %item%`. The objects `%item%` are the _items_ of the set. The empty set is the empty composite.

Sets inherit their structure from composites: they are unordered and contain each item at most once, `⟨contains ·· %a%, contains ·· %b%⟩ == ⟨contains ·· %b%, contains ·· %a%⟩` and `⟨contains ·· %a%, contains ·· %a%⟩ == ⟨contains ·· %a%⟩`.

`{...}` is syntactic sugar for sets: `{%a%, %b%, %c%}` stands for `⟨contains ·· %a%, contains ·· %b%, contains ·· %c%⟩`, and `{}` stands for the empty set. Braces therefore always denote sets, never composites with arbitrary members.

Because the constraint of `contains` accepts every value for every subject, a set places no demands on its items beyond their own validity: a set is valid whenever its items are. This is what makes sets data: computed collections, such as query results or the output of map and filter, never run the constraints of the objects they collect.

Sets are stored optimally in memory.

### Example

```
# {1, 2} is the composite
⟨contains ·· 1, contains ·· 2⟩

# {} is the empty composite
⟨⟩
```

## Integers

Integers are constructed recursively through object nesting. An object is an _integer_ iff

- it is equal to the atom `zero`, or
- it is a composite with a single property whose constructor is `successor_of` or `predecessor_of` and whose value is an integer.

An integer is _positive_ iff it has a `successor_of` property and _negative_ iff it has a `predecessor_of` property. The atom `zero` is neither positive nor negative. For the canonical representation, the value of a `successor_of` property must be non-negative and the value of a `predecessor_of` property must be non-positive.

This approach of constructing integers is similar to the Peano axioms definition of natural numbers.

### Examples

```
# -3
⟨predecessor_of ·· ⟨predecessor_of ·· ⟨predecessor_of ·· zero⟩⟩⟩

# 0
zero

# 1
⟨successor_of ·· zero⟩

# 2
⟨successor_of ·· ⟨successor_of ·· zero⟩⟩

# Not an integer
⟨successor_of ·· ⟨⟩⟩
```

Integers are stored optimally in memory.

## Lists

An object is a _list_ iff either

- it is the empty set `{}`, or
- it has exactly one property with the constructor `list_item`, exactly one property with the constructor `list_tail`, and the value of the `list_tail` property is a list.

The value of the `list_item` property is the first element of the list and the value of the `list_tail` property is the rest of the list.

### Example

```
# [1, 2, 3]
⟨
    list_item ·· ⟨successor_of ·· zero⟩,
    list_tail ·· ⟨
        list_item ·· ⟨successor_of ·· ⟨successor_of ·· zero⟩⟩,
        list_tail ·· ⟨
            list_item ·· ⟨successor_of ·· ⟨successor_of ·· ⟨successor_of ·· zero⟩⟩⟩,
            list_tail ·· {}
        ⟩
    ⟩
⟩
```

## Characters

A Unicode code point can be modelled as `⟨code_point ·· %integer%⟩` where `%integer%` is an integer between 0 and 0x10FFFF that does not represent a surrogate code point (0xD800 to 0xDFFF). Such an object is a _character_.

Characters are stored optimally in memory.

### Example

```
# 'A'
⟨code_point ·· 65⟩
```

## Text

Text can be modelled as a list of characters. Text is stored as UTF-8 in memory.

## Byte

A _byte_ is modelled as

```
⟨
    bit_slot_0 ·· %bit 0%,
    bit_slot_1 ·· %bit 1%,
    bit_slot_2 ·· %bit 2%,
    bit_slot_3 ·· %bit 3%,
    bit_slot_4 ·· %bit 4%,
    bit_slot_5 ·· %bit 5%,
    bit_slot_6 ·· %bit 6%,
    bit_slot_7 ·· %bit 7%
⟩
```

where each `%bit n%` is either the atom `bit_off` or the atom `bit_on`. `bit_slot_0` is the least significant bit and `bit_slot_7` is the most significant bit. A byte must have exactly one property for each bit slot.

Bytes are stored optimally in memory.

### Example

```
# 0xA5
⟨
    bit_slot_0 ·· bit_on,
    bit_slot_1 ·· bit_off,
    bit_slot_2 ·· bit_on,
    bit_slot_3 ·· bit_off,
    bit_slot_4 ·· bit_off,
    bit_slot_5 ·· bit_on,
    bit_slot_6 ·· bit_off,
    bit_slot_7 ·· bit_on
⟩
```

## Bytes

Bytes / binary data can be modelled as a list of bytes. It is stored as actual bytes in memory.

## Rational Numbers

The atoms `numerator` and `denominator` are introduced by this model and are not predefined in `atoms.rs`.

A rational number in math is represented by two integers $a$ and $b$: $a/b$. The goal is to model rational numbers in Everything such that **two objects representing rational numbers are equal (and that means _structurally_) iff the rational numbers the objects represent are equal**.

However, $2/1 = 4/2$. So we have to define some kind of normalization. Another requirement is that integers must also be rational numbers ($5 = 5/1$). This leads to this definition:

An object is a _rational number_ iff either

- it is an integer, or
- all of the following hold:
    - it has the property constructor `numerator` with a single associated value,
    - it has the property constructor `denominator` with a single associated value,
    - every property on the object is constructed using `numerator` or `denominator`,
    - the associated value of `numerator` (the _numerator_) is an integer,
    - the associated value of `denominator` (the _denominator_) is an integer,
    - the numerator is not equal to zero,
    - the denominator is greater than one, and
    - numerator and denominator are coprime (i.e. the greatest common divisor of their absolute values is one).

### Example

```
# 1 / 2
⟨
    numerator ·· ⟨successor_of ·· zero⟩,
    denominator ·· ⟨successor_of ·· ⟨successor_of ·· zero⟩⟩
⟩
```
