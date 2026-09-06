# Chapter 3 - Common Data Models

Everything gives you the freedom to model anything. However some models are stored optimally in memory, which may be desireable.

## Lists

A list can be modelled recursively like this:

* The empty list is the empty object.
* A list with an item has the form `{@LIST_ITEM: <<item>>, @LIST_TAIL: <<tail>>}` where the tail is a list.

## Characters

A unicode code point could be modelled as `{@CODE_POINT: <<integer>>}` where _integer_ is an integer between 0 and 0x10FFFF, excluding surrogates. Characters are stored optimally in memory.

## Text

Text can be modelled as a list of characters. Text is stored as UTF-8 in-memory.

## Byte

A byte can be modelled as

```
{
    $BIT_SLOT_0: <<bit 0>>,
    $BIT_SLOT_1: <<bit 1>>,
    $BIT_SLOT_2: <<bit 2>>,
    $BIT_SLOT_3: <<bit 3>>,
    $BIT_SLOT_4: <<bit 4>>,
    $BIT_SLOT_5: <<bit 5>>,
    $BIT_SLOT_6: <<bit 6>>,
    $BIT_SLOT_7: <<bit 7>>,
}
```

where `<<bit n>>` is a bit. A bit is either the abstract object `@BIT_0` or the abstract object `@BIT_1`.

## Bytes

Bytes / binary data can be modelled as a list of bytes. It is stored as actual bytes in memory.

## Rational Numbers

A rational number in math is represented by two integers $a$ and $b$: $a/b$. The goal is to model rational numbers in Everything such that **two objects representing rational numbers are equal (and that means _structurally_) iff the rational numbers the objects represent are equal**.

However, $2/1 = 4/2$. So we have to define some kind of normalization. And another requirement is that integers (as in the previous model) must also be rational numbers ($5 = 5/1$). This leads to this definition:

An object is a rational number iff

* it is an integer or
    * it has a single tag `$NUMERATOR`,
    * a single tag `$DENOMINATOR`,
    * the associated value of `$NUMERATOR` (the _numerator_) is an integer,
    * the associated value of `$DENOMINATOR` (the _denominator_) is an integer,
    * the numerator is not equal to zero,
    * the denominator is greater than one, and
    * numerator and denominator are coprime (i.e. the greatest common divisor of their absolute values is one).

### Examples

```
# 1 / 2
{
    $NUMERATOR: {}
}
```
