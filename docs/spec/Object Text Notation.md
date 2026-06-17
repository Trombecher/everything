# Everything Object Text Notation

The _Everything Object Text Notation_ (OTN) is a functional-style programming language to represent queries, expressions, and objects concisely. This document describes a mapping between OTN syntax and raw abstract objects and structures. The latter will be written using a subset of OTN which is desugared (only abstract objects and raw structures).

Note that this document does not cover the _meaning of these structures_. For that, read up on [how the engine evaluates objects](./Object%20Evaluation%20Specification.md).

## Expression Types

## Meta (Literal)

A meta literal is an expression starting with `$` which is followed by an identifier. Meta literals will resolve either to

* function parameter references (if the name matches with a parameter), or
* to some library user defined object at transform time.

The purpose of this expression type is to allow addressation of objects by their (local) semantic meaning. Therefore, one (popular) decision is to resolve these literals to objects that have same _name_. If there is no object (or multiple objects) with this name, resolution should fail.

Examples include:

* `$Xä`
* `$343`
* `$SomeObjectName`

### Raw Structure

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

`<<VAR>>` must be a meta literal, e.g. `$x` or `$variable_name123`.

### Escape `escape`

An escape expression `escape <<INNER>>` is syntactic sugar for:

```eotn
{@2148623946948209931514052368378168923 = <<INNER>>}
```

### Equals `==`

An equals expression `<<LEFT>> == <<RIGHT>>` is syntactic sugar for:

```eotn
{
    @2148623984105467336671475554302291443 = <<LEFT>>,
    @2150546540588687321716707989954282134 = <<RIGHT>>,
}
```

### Less `<`

A less expression `<<LEFT>> < <<RIGHT>>` is syntactic sugar for:

```eotn
{
    @2150755802916608365774567517427204904 = <<LEFT>>,
    @2150755809462832010701281576784730955 = <<RIGHT>>,
}
```

### Knowledge `knowledge`

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

### Multiply `*`

A multiply expression `<<LEFT>> * <<RIGHT>>` is syntactic sugar for:

```eotn
{
    @2150955291898078111384483788783842606 = <<LEFT>>,
    @2150955291897679523990373018161137292 = <<RIGHT>>,
}
```

### Count `count`

A count expression `count <<INNER>>` is syntactic sugar for:

```eotn
{
    @2148623971839749022702961541901456532 = <<INNER>>
}
```

### If-Then-Else

An if expression `if <<CONDITION>> then <<THEN>> else <<ELSE>>` is syntactic sugar for:

```eotn
{
    @2150756911395278548780220055702389669 = <<CONDITION>>,
    @2150756911395360151235546653103149781 = <<THEN>>,
    @2150756911395636855407117245584275797 = <<ELSE>>,
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
