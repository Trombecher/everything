# Everything Object Text Notation

The _Object Text Notation_ (OTN) is a functional-style syntax for concise expression of queries, expressions, and objects. This document describes a mapping between OTN syntax and raw abstract object and structures. The latter will be written using a subset of OTN which is desugared.

Additionally, OTN syntax may contain placeholders which are resolved at compile-time.

## OTN Syntax

```ebnf
digit = "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9";
letterlike = "A" | "B" | "C" | "D" | "E" | "F" | "G"
    | "H" | "I" | "J" | "K" | "L" | "M" | "N"
    | "O" | "P" | "Q" | "R" | "S" | "T" | "U"
    | "V" | "W" | "X" | "Y" | "Z" | "a" | "b"
    | "c" | "d" | "e" | "f" | "g" | "h" | "i"
    | "j" | "k" | "l" | "m" | "n" | "o" | "p"
    | "q" | "r" | "s" | "t" | "u" | "v" | "w"
    | "x" | "y" | "z" | "ä" | "Ä" | "ö" | "Ö"
    | "ü" | "Ü" | "ß" | "_";

abstract_object = "@", digit, {digit};
structure_literal = ("{", "}")
    | ("{", expression, "=", expression, {",", expression, "=", expression}, [","], "}");

text_literal = "
placeholder = "$", letterlike, {letterlike | digit};

function_expression = "function", placeholder, "=>", expression;
call_expression = expession, expression
grouping_expression = "(", expression, ")"

binary_operator = "+" | "-" | "*" | "/" | "union" | "intersection"
    | "|>" | "|?" | "==" | "<=" | "<" | "and" | "or" | "xor"
    | "->";
binary_expression = expression, binary_operator, expression;

unary_expression = ("not" | "query"), expression;

expression = placeholder
    | abstract_object
    | structure_literal
    | function_expression
    | call_expression
    | grouping_expression
    | unary_expression
    | binary_expression;
```

### Calls

A call `<<CALLEE>> <<WITH>>` is syntactic sugar for:

```
{
    @2150967257692765401288058191156339282 = <<CALLEE>>,
    @2150967257692538503396115970645049110 = <<WITH>>
}
```

### Functions

A function expression `function <<VAR>> => <<BODY>>` is syntactic sugar for:

```eotn
{@2148623901005465698003044719488417081 = <<BODY>>}
```

### Add `+`

An add expression `<<LEFT>> + <<RIGHT>>` is syntactic sugar for:

```eotn
{
    @2148566534810416742677953060919673357 = <<LEFT>>
    @2148566534810416742677953060919673358 = <<RIGHT>>
}
```

### Subtract `-`

An subtract expression `<<LEFT>> - <<RIGHT>>` is syntactic sugar for:

```eotn
{
    TODO
}
```

### Queries `query`

A query-expression `query (<<SUBJECT>>, <<TAG>>, <<VALUE>>)` is syntactic sugar for:

```
{@2148623977746529761395662089576479852 = {
    @2148623909053123893672709737372288428 = <<SUBJECT>>,
    @2148623916651203732644414190007253763 = <<TAG>>,
    @2148623924076051576854508924514462036 = <<VALUE>>,
}}
```

If `<<SUBJECT>>`, `<<TAG>>`, and/or `<<VALUE>>` are `?`, then their corresponding property in the desugared query is omitted.

### Map `|>`

A map expression `<<SET>> |> <<MAPPER>>` is syntactic sugar for:

```eotn
{
    @2150755705084816915741037497738372617 = <<SET>>
    @2150755713297709286586583523030939076 = <<MAPPER>>
}
```

### Map `|?`

A filter expression `<<SET>> |? <<FILTER>>` is syntactic sugar for:

```eotn
{
    @2150755714608308122129968313720999578 = <<SET>>
    @2150755715328094057860222292607728283 = <<MAPPER>>
}
```

### Unwrap Or

An _unwrap or_ expression `<<SET>> unwrapor <<DEFAULT>>` is syntactic sugar for:

```eotn
{
    @2150860435707265317782695126754608091 = <<SET>>,
    @2150860435707466294816983480346985929 = <<DEFAULT>>
}
```

## Examples

```
query (?, $PERSON, ?)
|> (function $subject_and_value => query ($subject_and_value, $SUBJECT, ?) unwrapor {})
|? (function $person => let $age = query ($person, $AGE, ?) unwrapor 0 in $age => 20 <= $age && $age <= 30)
```
