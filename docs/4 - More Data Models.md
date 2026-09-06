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
