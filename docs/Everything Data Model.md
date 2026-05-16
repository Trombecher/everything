# Introduction To The _Everything Data Model_

The _Everything Data Model_ is an interpretation of a family of structures and abstract objects. To interpret, the model assigns meaning/semantics to some abstract objects. **The real identifiers are omitted because they are meaningless and bloat the text.** You can look up the integer values in [this file](../crates/everything_structures/src/abstracts.rs) and in [this file](../crates/everything/src/ext/abstracts.rs).

(The reason that there are two files is, that some abstract objects are needed in structures that are stored more optimally in memory, like natural numbers, text, bytes, etc.)

In the following sections, aliases are used to talk about the abstract objects. They are written in inline code `LIKE_THIS`.

## Goals

The model tries to be the basis for any data model. This is a bit like JSON schema, which itself provides a meta-schema, validating itself. But the model includes a lambda calculus-like programming language.

## Vocabulary

The _knowledge_ is a structure which is a parameter to each expression, a sort of context which influences evaluation.

An object _has a tag with a value_ iff

* there exists a statement in the knowledge that includes the object, the tag, and the value; or if
* the tag has `COMPUTED` and the value is is included in the result of the application of the tag with the object as the parameter.

An object _has a tag_ iff there exists a value such that the object has a tag with this value.

An object _a_ is included in another object _b_ iff _b_ has `CONTAINS` with the value _a_.

## Sets

Everything models sets with the object `CONTAINS`. These are examples for sets:

```
{}                                <- empty set
{(@534, @13)}                     <- also the empty set
{(CONTAINS, {})}                  <- set that contains the empty object
{(CONTAINS, @42) (CONTAINS, @90)} <- set that contains @42 and @90
```

## Statements

A _statement_ is an object that has a subject with `STATEMENT_SUBJECT`, a tag with `STATEMENT_TAG`, and a value with `STATEMENT_VALUE`.

For example this is a statement:

```
{
    (STATEMENT_SUBJECT, @5345389) <- subject
    (STATEMENT_TAG, @9034593)     <- tag
    (STATEMENT_VALUE, @4353459)   <- value
}
```

Conceptually, the subject is the object in question which the statment is about. The tag is the "attribute" of the subject. The value is associated data with that association.

## Booleans

A set with no items is "false". A set with one or more items is "true".

## Natural Numbers

Everything models the natural numbers with nested successors. The zero is represented by an abstract object `ZERO` and the successor of a natural number is represented with the abstract object `SUCCESSOR_OF`. For example `{(SUCCESSOR_OF, {(SUCCESSOR_OF, ZERO)})}` would be the number 2.

## Knowledge

_Knowledge_ is a set of statements in which every structure is valid. More about validity later.

## Axiomatic

To use an object as a tag in a statement, it must be axiomatic, meaning it must have the tag `AXIOMATIC`. The value you provide when stating that your object is axiomatic is a function that validates each specific use of your tag with a subject and a value.

### Example

This is best illustrated with an example. Let's say we want to model the following situation: we have an attribute "owns car" and an attribute "owns blue car" that we want to model. "owns car" should be applicable to anything, but "owns blue car" should only be a refinement to people that own a car.

Let `@111111` be "owns car" and `@222222` be "owns blue car". Then these statements (with `<<...>>` being templates we will fill out later)

```
{
    (STATEMENT_SUBJECT, @111111)
    (STATEMENT_TAG, AXIOMATIC)
    (STATEMENT_VALUE, <<ALWAYS>>)
}

{
    (STATEMENT_SUBJECT, @222222)
    (STATEMENT_TAG, AXIOMATIC)
    (STATEMENT_VALUE, <<ONLY WHEN @111111>>)
}
```

mean "`@111111` is axiomatic and can always be used with any value" and "`@222222` is axiomatic and can only be used when the subject has the tag `@111111`".

Now we do this for `@333333` and `@444444` being some people:

```
{
    (STATEMENT_SUBJECT, @333333)
    (STATEMENT_TAG, @111111)
    (STATEMENT_VALUE, {})
}
{
    (STATEMENT_SUBJECT, @333333)
    (STATEMENT_TAG, @222222)
    (STATEMENT_VALUE, {})
}

# This is not possible
{
    (STATEMENT_SUBJECT, @444444)
    (STATEMENT_TAG, @222222)
    (STATEMENT_VALUE, {})
}

# This is also not possible
{
    (STATEMENT_SUBJECT, @444444)
    (STATEMENT_TAG, @333333)
    (STATEMENT_VALUE, {})
}
```

The first statement states that `@333333` owns a car. The second statement states that `@333333` owns a blue car (which is allowed, since they own a car).

The third statement is not valid since `@444444` does not own any car, so they can't own a blue car by definition of `@222222`. The fourth statement tries to use `@333333` as a tag but `@333333` is not `AXIOMATIC`, so this is also not allowed.

Later you will see how to express these constraints instead of placeholders.

## Computation / Reflection

Everything contains a computation system which is basically an extension of lambda calculus. It contains nodes which are just structures themselves. The engine then evaluates the nodes, reducing expressions.

### Computation / Parameter Nodes

The computation node / function is denoted with `COMPUTED`. The corresponding value is the function body. If the function is invoked with a parameter (either by the engine or other functions) then the function body will be evaluated with the parameter value.

Parameter references will then be replaced by their values on invocation. Parameter references are denoted with `NODE_PARAMETER` with the value being a natural number denoting the "relative depth". A depth of 0 means that this parameter reference is referencing the innermost function relative to that parameter reference. 1 means the wrapping function of the function addressed by 0, and so on.

This is best illustrated with examples:

| Structure                                           | Non-normative Textual Representation         |
|-----------------------------------------------------|----------------------------------------------|
| `{(COMPUTED, @5345345)}`                            | `x \|-> @5345345` (just a constant function) |
| `{(COMPUTED, {(NODE_PARAMETER, 0)})}`               | `x \|-> x`                                   |
| `{(COMPUTED, {(COMPUTED, {(NODE_PARAMETER, 0))}})}` | `x \|-> y \|-> y`                            |
| `{(COMPUTED, {(COMPUTED, {(NODE_PARAMETER, 1))}})}` | `x \|-> y \|-> x`                            |

### Logical Primitives

There are nodes for `NODE_AND`, `NODE_OR`, `NODE_XOR`, and `NODE_NOT`.

| Primitive  | Structure                            | Meaning                                             |
|:----------:|:------------------------------------:|:---------------------------------------------------:|
| `NODE_AND` | `{(NODE_AND, ...), (NODE_AND, ...)}` | True iff all values (evaluated) are true            |
| `NODE_OR`  | `{(NODE_OR, ...), (NODE_OR, ...)}`   | True iff at least one value (evaluated) is true     |
| `NODE_XOR` | `{(NODE_XOR, ...), (NODE_XOR, ...)}` | True iff only exactly one value (evaluated) is true |
| `NODE_NOT` | `{(NODE_NOT, ...)}`                  | True iff the value (evaluated) is false             |

## Dynamic Queries

The computation can inspect itself and run queries against its own knowledge. For that, there exists the tag `NODE_QUERY`. It accepts a value in a form of a statement. The statement may omit subject, tag, and/or value. These are the behaviours:

| Query Statement Form                | Returns                                                                              | 
|:-----------------------------------:|:------------------------------------------------------------------------------------:|
| `{(STATEMENT_SUBJECT, ...), (STATEMENT_TAG, ...), (STATEMENT_VALUE, ...)}` | true if this statement exists or the value is in the set returned by the computation |
| `{(STATEMENT_SUBJECT, ...), (STATEMENT_TAG, ...)}`            | A set of all values or the computation result of the tag on the subject              |

More will follow.
