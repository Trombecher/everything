# Optimal Object Representation In Memory

## Inline Integer

```plain
0xxxxxxx xxxxxxxx xxxxxxxx xxxxxxxx
xxxxxxxx xxxxxxxx xxxxxxxx xxxxxxxx
xxxxxxxx xxxxxxxx xxxxxxxx xxxxxxxx
xxxxxxxx xxxxxxxx xxxxxxxx xxxxxxxx
```

## Inline UTF-8 with a byte length of 1

```
10000000 00000000 00000000 00000000
00000000 00000000 00000000 00000000
00000000 00000000 00000000 00000000
00000000 00000000 00000000 aaaaaaaa
```

`aaaaaaaa` is the only byte stored.

## ...

## Inline UTF-8 with a byte length of 15

```
10001110 aaaaaaaa bbbbbbbb cccccccc
dddddddd eeeeeeee ffffffff gggggggg
hhhhhhhh iiiiiiii jjjjjjjj kkkkkkkk
llllllll mmmmmmmm nnnnnnnn oooooooo
```

## Inline bytes with a byte length of 1

## ...

## Inline bytes with a bytes length of 15

## Empty composite

```plain
10100000 00000000 00000000 00000000
00000000 00000000 00000000 00000000
00000000 00000000 00000000 00000000
00000000 00000000 00000000 00000000
```

## Character

```
10100000 00000000 00000000 00000000
00000000 00000000 00000000 00000000
00000000 00000000 00000000 00000001
aaaaaaaa bbbbbbbb cccccccc dddddddd
```

where bits `a...d` compose a 32 bit unsigned integer `x` (with the a's being the MSB) such that

- `x < 0x110000`,
- `x < 0xD800`, and
- `0xDFFF < x`.

## Inline Rational Numbers
