# Common Data Models

Everything does not specify how you model your data. However some models are stored optimally in memory, which may be desireable.

## Characters

A unicode code point could be modelled as `{(@CODE_POINT, <<integer>>)}` where _integer_ is an integer between 0 and 0x10FFFF, excluding surrogates. Characters are stored optimally in memory.

## Text

Text can be modelled as a list of characters.

## Byte

A byte can be modelled as

```
{
    (BIT_SLOT_0, <<bit 0>>)
    (BIT_SLOT_1, <<bit 1>>)
    (BIT_SLOT_2, <<bit 2>>)
    (BIT_SLOT_3, <<bit 3>>)
    (BIT_SLOT_4, <<bit 4>>)
    (BIT_SLOT_5, <<bit 5>>)
    (BIT_SLOT_6, <<bit 6>>)
    (BIT_SLOT_7, <<bit 7>>)
}
```

where bit n is a bit. A bit is either the abstract object `BIT_0` or the abstract object `BIT_1`.

## Bytes

Bytes / binary data can be modelled as a list of bytes.
