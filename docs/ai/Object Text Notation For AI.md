# Language specification for the Object Text Notation

This document provides all essential context for an AI to transform and interpret Everything object text notation to and from natural language.

## Context

### What is Everything?

Everything is a project trying to store data and knowledge in its purest form. It works with objects and structures to provide a basis for the user to model anything they like. This underlying structure is outlined in the next section.

### Everything Objects and Structures

* An _object_ is either an abstract object or a structure.
* An _abstract object_ is an abstract identifier which is just a 128 bit unsigned integer.
* A _structure_ is a set of properties.
* A _property_ is a pair of the form `(tag, value)`, both objects.

This recursive definition is the basis of Everything.

### The Everything Object Text Notation

The _Everything object text notation_ is a "programming language" to represent common objects in Everything. Although all objects in Everything are either abstract or structures, this document does not describe the transformation process of the abstract syntax tree of a program in Object Text Notation. But rather just tells you what the expressions do on a higher level. This is to make it easier for you to understand these concepts and not emit raw structures when you could have used syntactic sugar you are already kind of familiar with.

Under the hood this syntactic sugar to create certain kinds of expressions and objects is transformed into raw abstract objects and structures, and then interpreted by the Everything inference engine.

### Placeholders

Because all identififers of abstract objects are unsigned integers, syntax like `@MyObject` would be invalid. However, working with very long numbers like `@8329058043850934535643` is cumbersome and not great when working with AI, this document uses placeholders like `$CONTAINS`, `$AXIOMATIC`, and more to talk about these abstract objects.

When the actual query is finalized, it is the user's job to replace these placeholders with actual abstract objects. PLEASE DO NOT USE TEXT AS TAGS. USING TEXT AS TAGS LIKELY MEANS THAT YOU WANT TO DESCRIBE SOME SORT OF ATTRIBUTE, IN WHICH CASE A PLACEHOLDER IS THE ONLY CORRECT CHOICE.

## Definitions

The _knowledge_ is an implicit parameter to all evaluations. It is a structure being a set of statements which can be queried.

### Sets

Every object is a set and every set is an object.

The _set values_ of an object _S_ are all objects that are yielded from a values query with the subject of that query being _S_ and the tag of that values query being `$CONTAINS`.

Referring to an object by a set implies that the set values of that object are of contextual relevance.

An _object is a value of a set_ if and only if is is amongst the set values of that set. An _object is included in a set_ if and only if the object is a value of that set.

### Truthiness

An object is _truthy_ if and only if it has at least one set value. An object is _falsy_ if and only if it is not truthy.

Often, `{}` (the empty structure) is returned as "false" because it is falsy. Often, `{$CONTAINS = {}}`

### Expression terminology

All objects are _expressions_ and all expressions are objects. When referring to an object by an expression, it has contextual significance. For example only for some objects, it makes sense to evaluate them. These are called expressions.

All objects, that are passed into an expression, are called the _inputs_ of that expression.

The object, an expression evaluates to, is called the _result_ of that expression.

## Expression Types

### Abstract Objects

An abstract object has the following format:

```
@<<DIGITS>>
```

where `<<DIGITS>>` is a non-empty sequence of ASCII digits. But most of the time, you will not use this but instead use placeholders.

### Integers

You know what integers are.

### Text

Text is a list of characters. The text-expression has this JavaScript-style format:

```
"this is the text content"
```

If you need to write special characters, use character escapes:

#### Character escapes

* `\n` is the ASCII line feed character
* `\r` is the ASCII carriage return character
* `\t` is the ASCII tab character
* `\0` is the ASCII null character
* `\"` is the ASCII quote `"`

### Raw Structures

Raw structures have the following format:

```
{
    <<TAG_1>> = <<VALUE_1>>,
    <<TAG_2>> = <<VALUE_2>>,
    ...
}
```

where `<<TAG_X>>` and `<<VALUE_X>>` are expressions. This is the standard notation to create arbitrary structures.

### Add `+`

Add-expressions have the following format:

```
<<LEFT>> + <<RIGHT>>
```

where `<<LEFT>>` and `<<RIGHT>>` are expressions. The add expression is binary.

If both `<<LEFT>>` and `<<RIGHT>>` evaluate to integers, then the add-expression evaluates to the arithmetic sum of those integers. If the sum of those integers exceeds implementation-defined bounds, then the result is the abstract object `$ARITHMETIC_OVERFLOW`.

If not both `<<LEFT>>` and `<<RIGHT>>` evaluate to integers, the result is the abstract object `$UNDEFINED`.

### Subtract `-`

The _subtract_-expression is binary. This is the format:

```
<<LEFT>> - <<RIGHT>>
```

where `<<LEFT>>` and `<<RIGHT>>` are expressions.

If both `<<LEFT>>` and `<<RIGHT>>` evaluate to integers, then the subtract-expression evaluates to the arithmetic difference of those integers. If the difference of those integers exceeds implementation-defined bounds, then the result is the abstract object `$ARITHMETIC_OVERFLOW`.

If not both `<<LEFT>>` and `<<RIGHT>>` evaluate to integers, the result is the abstract object `$UNDEFINED`.

### Multiply `*`

Multiply-expressions have the following format:

```
<<LEFT>> * <<RIGHT>>
```

where `<<LEFT>>` and `<<RIGHT>>` are expressions.

If both `<<LEFT>>` and `<<RIGHT>>` evaluate to integers, then the multiply-expression evaluates to the arithmetic product of those integers. If the product exceeds implementation-defined bounds, then the result is the abstract object `$ARITHMETIC_OVERFLOW`.

### Equals `==`

Equals-expressions have the following format:

```
<<LEFT>> == <<RIGHT>>
```

where `<<LEFT>>` and `<<RIGHT>>` are expressions.

The _equal_-expression compares the results of expressions `<<LEFT>>` and `<<RIGHT>>` by their structure and returns a truthy object if they are equal; otherwise the equal-expression evaluates to a falsy object.

### Less `<`

Less-expressions have the following format:

```
<<LEFT>> < <<RIGHT>>
```

where `<<LEFT>>` and `<<RIGHT>>` are expressions.

If both `<<LEFT>>` and `<<RIGHT>>` evaluate to integers, then the result of the less-expression is truthy if the result of `<<LEFT>>` is arithmetically smaller than the result of `<<RIGHT>>`; otherwise the less-expression will evaluate to an object which is falsy.

If not both `<<LEFT>>` and `<<RIGHT>>` evaluate to integers, the result is the abstract object `$UNDEFINED`.

### Less or Equals `<=`

Less-or-equals-expressions have the following format:

```
<<LEFT>> <= <<RIGHT>>
```

where `<<LEFT>>` and `<<RIGHT>>` are expressions.

If both `<<LEFT>>` and `<<RIGHT>>` evaluate to integers, then the result of the less-expression is truthy if the result of `<<LEFT>>` is arithmetically smaller than or equal to the result of `<<RIGHT>>`; otherwise the less-expression will evaluate to an object which is falsy.

If not both `<<LEFT>>` and `<<RIGHT>>` evaluate to integers, the result is the abstract object `$UNDEFINED`.

### Not `not`

The _not_ expression inverts the result of the inner expression. If the inner expression evaluates to a truthy value, then the result of the _not_ expression is falsy; otherwise a truthy value returned.

The _not_ expression is unary. This is the format:

```
not <<INNER>>
```

where `<<INNER>>` is an expression.

### Count `count`

Count-expression have the following format:

```
count <<INNER>>
```

where `<<INNER>>` is an expression.

The _count_ expression counts the number of set values of the result of `<<INNER>>`. The count expression is unary.

### Function

A function expression has the following format. The variable/parameter must be prefixed by a $ like in PHP.

```
function $variable_name => <<BODY>>
```

where `<<BODY>>` is an expression.

### Query

The _query_-expression queries the knowledge. There are seven modes of this expression.

#### Query for values

Query-expressions in the values mode have the following format:

```
query values with (<<SUBJECT>>, <<TAG>>)
```

where `<<SUBJECT>>` and `<<TAG>>` are both expressions. This expression evaluates to a set of all objects that are values in a statement together with the result of `<<SUBJECT>>` (as a subject) and with the result of `<<TAG>>` (as a tag).

#### Query for tags and values

Query-expression in the _tags and values_ mode have the following format:

```
query tags and values with <<SUBJECT>>
```

where `<<SUBJECT>>` is an expression. This expression evaluates to a set of all objects of the structure `{$Statement_Tag = ..., $Statement_Value = ...}` that form a statement together with the result of `<<SUBJECT>>`.

#### Query for subjects

Query-expressions in the subjects mode have the following format:

```
query subjects with (<<TAG>>, <<VALUE>>)
```

where `<<TAG>>` and `<<VALUE>>` are both expressions.

A query-expression in the _subjects_ mode evaluates to a set of all objects that are subjects in all statements of the knowledge with a given input tag and input value.

#### Query for subjects and values

Query-expressions in the _subjects and values_ mode have the following format:

```
query subjects and values with <<TAG>>
```

where `<<TAG>>` is an expression. This expression evaluates to a set of all objects of the structure `{$Statement_Subject = ..., $Statement_Value = ...}` such that each subject and value pair, together with the result of `<<TAG>>`, make up a statement.

#### Others

TODO

### If

If-expressions have the following format:

```
if <<CONDITION>> then <<THEN_BRANCH>> else <<ELSE_BRANCH>>
```

The _if_-expression evaluates to the result of `<<THEN_BRANCH>>` if the result of `<<CONDITION>>` is truthy; otherwise the if-expression evaluates to the result `<<ELSE_BRANCH>>`.

### Union

Union-expressions have the following format:

```
<<LEFT>> union <<RIGHT>>
```

where `<<LEFT>>` and `<<RIGHT>>` are expressions.

A union-expression evaluates to the set which includes all set values of the result of `<<LEFT>>` and all set values of the result of `<<RIGHT>>`. It performs a set-theoretic union.

### Intersection

Intersection-expressions have the following format:

```
<<LEFT>> intersection <<RIGHT>>
```

where `<<LEFT>>` and `<<RIGHT>>` are expressions.

An intersection-expression evaluates to the set which includes all set values that are included in the result of `<<LEFT>>` as well as the result of `<<RIGHT>>`. It performs a set-theoretic intersection.

### Map `map`

Map-expressions have the following format:

```
<<INPUT_SET>> map <<MAPPER>>
```

where `<<INPUT_SET>>` and `<<MAPPER>>` are expressions.

A _map_-expression maps every set value of the result of `<<INPUT_SET>>` to a new set value via the `<<MAPPER>>` object. For each set value, `<<MAPPER>>` is called with that set value. Although any object is accepted by `<<MAPPER>>`, common expressions are functions.

This functionality is equivalent to JavaScript's `Array.map(...)` but for sets.

### Filter `filter`

Filter-expressions have the following format:

```
<<INPUT_SET>> filter <<FILTER>>
```

where `<<INPUT_SET>>` and `<<FILTER>>` are both expressions.

A _filter_-expression retains all set values of the result of `<<INPUT_SET>>` for which the call of `<<FILTER>>` with that set value evaluates to a truthy value.

This functionality is equivalent to JavaScript's `Array.filter(...)` but for sets.

### Unwrap Or `unwrap or`

Unwrap or-expressions have the following format:

```
<<SET>> unwrap or <<DEFAULT>>
```

An _unwrap or_ expression extracts _the_ single value of the result of `<<SET>>` IF the result of `<<SET>>` is a set consisting of exactly ONE value. OTHERWISE, the result of `<<DEFAULT>>` is returned.

## Operator Precedence

Here is a list of operator precedences, from least binding to highest binding:

1. `map`
2. `filter`
3. `unwrap or`
4. `union`
5. `intersection`
6. `==`
7. `<`
8. `<=`
9. `+`
10. `-`
11. `*`

Use parentheses if you are not sure.

## Guidelines

Here are some guidelines.

### DO NOT USE DOT-ACCESS FOR "PROPERTIES"

If the user is describing some property/tag of an object, use the `query values` syntax to get a set of all values and then use the `unwrap or` syntax to extract that value.

### DO NOT FORGET THE FUNCTION KEYWORD ON FUNCTIONS

### DO NOT USE PLURAL NAMES FOR PLACEHOLDERS

If the user references an object via a plural name, then strongly consider using a query for (part of) that request. For example, if the user is talking about "all people", query for all objects which have the tag `$Person`.

### DO NOT INTRODUCE UNNECCESSARY PLACEHOLDERS

If the use gave some information that is relative to context (like time, location, etc.), try to assume as little as possible and compute as much as possible in the expression/query.
