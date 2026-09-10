# 3 - Introduction To The _Everything Data Model_

The _Everything Data Model_ is an interpretation of a family of objects. It assigns meaning and semantics to some objects.

Object literals on this page may contain placeholders, prefixed by a dollar sign `$LIKE_THIS`. These represent abstract objects whose real integer values can be looked up in [this file](../../crates/everything_objects/src/abstracts.rs) and in [this file](../../crates/everything/src/ext/abstracts.rs).

## Goals

The model tries to be the basis for any data model. This is a bit like JSON schema which itself provides a meta-schema, validating itself. But this model includes a lambda calculus-like programming language.

## Vocabulary / Definitions

* The _knowledge_ is a composite object which is a parameter to each expression. This is the data of the database expressions may be evaluated in.

* An object _has a tag with a value_ iff there exists a statement in the knowledge that includes the object, the tag, and the value; or, if the object is a composite object, `tag = value` is a property in the object.

* An object _has a tag_ iff there exists a value such that the object has a tag with this value.

## Sets

Everything models set inclusion via the tag `$CONTAINS`.

Every object is a set. An object is _item_ or _element_ of a set iff it is value of a property with `$CONTAINS` on the set.

**To state that an object is a set may indicate that the items of the object are of contextual relevance, or that the object's purpose is to hold items.**

### Examples

```
{}                               <- empty set
{$534: $13}                      <- also the empty set
{$CONTAINS: {}}                  <- set that contains the empty object
{$CONTAINS: $42, $CONTAINS: $90} <- set that contains $42 and $90
```

## Statements

An object is a _statement_ iff

* it is composite,
* it has tags `$STATEMENT_SUBJECT`, `$STATEMENT_TAG`, and `$STATEMENT_VALUE`, each with a single value, and
* **the associated value of `$STATEMENT_SUBJECT` is abstract**.

* The associated value of `$STATEMENT_SUBJECT` is also called the _subject_ of the statement. It may be interpreted as the (abstract) object a thing is stated about.
* The associated value of `$STATEMENT_TAG` is also called the _tag_ of the statement. It may be interpreted as the attribute of the subject which is stated about.
* The associated value of `$STATEMENT_VALUE` is also called the _value_ of the statement. Is may be interpreted as an associated "value" which "answers" the question of the tag.

### Example

```
{
    $STATEMENT_SUBJECT: $5345389,
    $STATEMENT_TAG: $9034593,
    $STATEMENT_VALUE: $4353459
}
```

## Booleans

A set with no items is considered _false_ or _falsy_. A set with one or more items is considered _true_ or _truthy_.

## Integers

Everything constructs the integers recursively through object nesting. An object is an _integer_ iff

* it is equal to the abstract object `$ZERO`, or
* it is composite and it only has a single property with
    * the tag being `$SUCCESSOR_OF` or `$PREDECESSOR_OF` and
    * the value being an integer.

By repeatetly constructing composite objects as successors or predecessors, you can get any integer. An integer is _negative_ iff it has the tag `$PREDECESSOR_OF`. An integer is positive iff it has the tag `$SUCCESSOR_OF`.

This approach of constructing integers is similar to the Peano-axioms definition of natural numbers.

### Examples

```
# -3
{$PREDECESSOR_OF: {$PREDECESSOR_OF: {$PREDECESSOR_OF: $ZERO}}}

# 0
$ZERO

# 1
{$SUCCESSOR_OF: $ZERO}

# 2
{$SUCCESSOR_OF: {$SUCCESSOR_OF: $ZERO}}

# Not an integer
{$SUCCESSOR_OF: {}}
```

## Knowledge

_Knowledge_ is a set of statements which are _valid_. What is "valid" will defined later.

> [!NOTE]
>
> No sub-queries are made when querying knowledge. More info TODO.

## Axiomatic

Everything imposes a restriction on how and when you can use objects as tags in statements. To use an object as a tag in a statement, it must be _axiomatic_, meaning, Everything requires a secondary property `$AXIOMATIC: constraint` **on the object** where `constraint` is an object which will validate each statement involving your object as a tag.

On each statement involving your object as a tag, the constraint will be (functionally) called two times,

1. with the subject of the statement and
2. with the value of the statement.

Your logic then decides what object to return.

* If the returned object is truthy, then Everything is happy;
* If the returned object is falsy, then your database contains invalid knowledge.

Not only does this validation apply to statements, but also to every property of every composite used.

### Example

This is best illustrated with an example. Let's say we want to model the following situation:

* Some objects are people.
* Some objects have an age but only people can have an age.
* Age must be a non-negative integer.

For that let's define aliases for abstract objects: `$PERSON` and `$AGE`. Content in `<<brackets>>` is placeholder. We can extend the base knowledge with these statements:

```
{
    $STATEMENT_SUBJECT: $PERSON,
    $STATEMENT_TAG: $AXIOMATIC,
    $STATEMENT_VALUE: {$CONTAINS: {}},
}

{
    $STATEMENT_SUBJECT: $AGE,
    $STATEMENT_TAG: $AXIOMATIC,
    $STATEMENT_VALUE: <<constraint: only on person and with non-negative integer>>,
}
```

The first statement states:

> `$PERSON` is axiomatic and has the constraint `{$CONTAINS: {}}` which is "true", so all subjects and values on uses are accepted.

The second statement states:

> `$AGE` is axiomatic and has a constraint that limits subjects on uses to people and values on uses to non-negative integers.

How this constraint is encoded, [will be shown later](#encoding-the-constraint-from-the-example) and is currently irrelevant. Now we get play around with our statements by adding new ones for `$ALICE`, `$BOB`, and `$LOVE`:

```
{
    $STATEMENT_SUBJECT: $ALICE,
    $STATEMENT_TAG: $PERSON,
    $STATEMENT_VALUE: {}
}
{
    $STATEMENT_SUBJECT: $BOB,
    $STATEMENT_TAG: $PERSON,
    $STATEMENT_VALUE: {}
}
```

The first statement states:

> `$ALICE` is a `$PERSON`.

The second statement states:

> `$BOB` is a `$PERSON`.

Since being a person does come with any context in our example, we just pass in `{}` as the value which is happily accepted by the constraint on `$PERSON`.

Now it gets interesting if we add the following statements:

```
{
    $STATEMENT_SUBJECT: $ALICE,
    $STATEMENT_TAG: $AGE,
    $STATEMENT_VALUE: 42
}
{
    $STATEMENT_SUBJECT: $BOB,
    $STATEMENT_TAG: $AGE,
    $STATEMENT_VALUE: $ALICE
}
{
    $STATEMENT_SUBJECT: $LOVE,
    $STATEMENT_TAG: $AGE,
    $STATEMENT_VALUE: 69
}
```

The first statement states:

> `$ALICE`'s `$AGE` is 42.

The second statement states:

> `$BOB`'s `$AGE` is `$ALICE`...?

...this cannot be right? In fact it is not valid. Because `$AGE` is used in a statement as the tag, its constraint must return "true" on

* `$BOB` as a subject and
* `$ALICE` as a value.

Although `$BOB` is a `$PERSON`, `$ALICE` is not a non-negative integer. So this statement invalidates the whole knowledge. Same with the third statement:

> `$LOVE`'s `$AGE` is 67.

Although 67 is a non-negative integer, `$LOVE` is not a person. Therefore, the engine tells you that this is not valid.

## Computation

_Everything_ contains a computation system which is basically an extension of lambda calculus. The engine is able to evaluate certain (families of) objects. These objects are called nodes.

### Functions

Functions are abstractions that accept one input object and evaluate to a node which itself or its children may reference the input object. An object is a function iff it has a single tag `$FUNCTION`. The associated value is the body.

### Parameters

Parameter nodes reference the object of the wrapping function that the node references. On evaluation, it resolves to the object which the wrapping function was applied to. An object is a parameter node iff it has a single tag `$NODE_PARAMETER` and the associated value is a non-negative integer.

The associated value is the _relative depth_.

* A depth of 0 means that this parameter node references the **innermost function** relative to that parameter node.
* A depth of 1 means the wrapping function of the function addressed by depth 0, and so on.

### Self-References

You can use `$NODE_FUNCTION_SELF` to reference a wrapping function, just like `$NODE_PARAMETER`. You can use it to implement recursive functions.

### Function Application

An object is a application (or call) node iff

* it has a single tag `$NODE_CALL_CALLEE` and
* a single tag `$NODE_CALL_WITH` (argument).

First, the callee child node is evaluated. If the result is a function, then all occurances of the parameter node inside the function's body node referencing this parameter are replaced with the argument object. If not, the result is returned.

### Conditional Nodes

Conditional nodes first evaluate their condition child node. If the result is truthy, they then evaluate the "then" child node; otherwise the "else" child node. An object is a conditional node iff

* it has a single tag `$NODE_IF_CONDITION`,
* has a single tag `$NODE_IF_THEN`, and
* has a single tag `$NODE_IF_ELSE`.

The associated values are the child nodes referenced earlier.

### Examples For Functions And Application

This is best illustrated with examples:

```
# x |-> $5345345
{$FUNCTION: $5345345}

# x |-> x, identity function
{$FUNCTION: {$NODE_PARAMETER: 0}}

# x |-> y |-> y
{$FUNCTION: {$FUNCTION: {$NODE_PARAMETER: 0}}}

# x |-> y |-> x
{$FUNCTION: {$FUNCTION: {$NODE_PARAMETER: 1}}}

# f := x |-> f
{$FUNCTION: {$NODE_FUNCTION_SELF: 0}}

# fix := f |-> f (fix f), fix-point operator
{$FUNCTION: {
    $NODE_CALL_CALLEE: {$NODE_PARAMETER: 0},
    $NODE_CALL_WITH: {
        $NODE_CALL_CALLEE: {$NODE_FUNCTION_SELF: 0},
        $NODE_CALL_WITH: {$NODE_PARAMETER: 0}
    }
}}
```

### Logical And

An object is an _and_ node iff

* it has a single tag `$NODE_AND_LEFT` and
* it has a single tag `$NODE_AND_RIGHT`.

This node first evaluates the left child node.

* If the result is false, then that result is returned from the and node.
* If the result is true, then the result of the right child node is returned.

### Logical Or

An object is an _or_ node iff

* it has a single tag `$NODE_OR_LEFT` and
* it has a single tag `$NODE_OR_RIGHT`.

This node first evaluates the left child node.

* If the result is true, then that result is returned from the and node.
* If the result is false, then the result of the right child node is returned.

### Logical XOR

An object is an _xor_ node iff

* it has a single tag `$NODE_XOR_LEFT` and
* it has a single tag `$NODE_XOR_RIGHT`.

This node first evaluates the left child node and then the right child node.

* If either the left result is true or the right result is true then the true result is returned.
* If both are true, then the empty composite is returned.
* If none are true, then the first result is returned.

### Logical Not

An object is a _not_ node iff it has a single tag `$NODE_NOT`. The associated value is the child node.

It first evaluates the child node and returns a truthy object if the result is false; otherwise it results a falsy object.

### Addition

An object is an _add_ node iff

* it has a single tag `$NODE_ADD_LEFT` and
* it has a single tag `$NODE_ADD_RIGHT`.

TODO: evaluation

### Multiplication

An object is a multiplication node iff

* it has a single tag `$NODE_MULTIPLY_LEFT` and
* it has a single tag `$NODE_MULTIPLY_RIGHT`.

TODO: evaluation

### Set Union

An object is a set union node iff

* it has a single tag `$NODE_UNION_LEFT` and
* it has a single tag `$NODE_UNION_RIGHT`.

It resolves to an object which contains all items from the result of the left child node and all items from the result of the right child node.

### Count Nodes

An object is a count node iff it has a single tag `$NODE_COUNT`. The associated value is the child node.

It resoves to an integer which represents the number of elements the result of the evaluated child node has.

### Filter Nodes

An object is a filter node iff

* it has a single `$NODE_FILTER_SET` and
* it has a single `$NODE_FILTER_FILTER`.

TODO: evaluation

### Map Nodes

An object is a map node iff

* it has a single `$NODE_MAP_SET` and
* it has a single `$NODE_MAP_MAPPER`.

TODO: evaluation

### Object Type Node

An object is an _is-abstract_ node iff

* it has a tag `$NODE_IS_ABSTRACT` with a single associated value.

It evaluates the child node being the associated value and resolves

* to `{$CONTAINS: {}}` if the resulting object of the evaluation of the child node is abstract and
* to `{}` otherwise.

### Queries

There are nodes that query the knowledge. For each query, first the composite properties of the subject are queries (abstract objects do not have composite properties) and then the whole knowledge is queried. Queries always return sets.

That means that each composite object may actually "have" more properties that the composite object is defined by. You can use the knowledge to state additional things about a composite object.

> [!NOTE]
>
> Every object is a "set". What is meant is that the return value of a query will be a composite object that has no other (composite) tags other than `$CONTAINS`.

| Composite Object                                                                                  | Meaning                                                                                                                                 |
|---------------------------------------------------------------------------------------------------|-----------------------------------------------------------------------------------------------------------------------------------------|
| `{$NODE_QUERY: {$STATEMENT_SUBJECT: ..., $STATEMENT_TAG: ..., $STATEMENT_VALUE: ...}}`    | Checks if this statement exists; returns a truthy values if yes, `{}` otherwise                                                         |
| `{$NODE_QUERY: {$STATEMENT_SUBJECT: ..., $STATEMENT_TAG: ...}}`                             | Queries the subject for values it has on the given tag and returns a set of all values                                                  |
| `{$NODE_QUERY: {$STATEMENT_SUBJECT: ..., $STATEMENT_VALUE: ...}}`                           | Queries the subject for tags it has with the given value and returns a set of all tags                                                  |
| `{$NODE_QUERY: {$STATEMENT_TAG: ..., $STATEMENT_VALUE: ...}}`                               | Queries all subjects that have this tag with this value and returns a set of all those subejcts                                         |
| `{$NODE_QUERY: {$STATEMENT_SUBJECT: ...}`                                                      | Queries the subject for tags and value pairs; returns a set of objects `{($STATEMENT_TAG, ...), ($STATEMENT_VALUE, ...)}`               |
| `{$NODE_QUERY: {$STATEMENT_TAG: ...}`                                                          | Queries the knowledge for all subject and value pairs; returns a set of objects `{($STATEMENT_SUBJECT, ...), ($STATEMENT_VALUE, ...)}`  |
| `{$NODE_QUERY: {$STATEMENT_VALUE: ...}`                                                        | Queries the knowledge for all subject and tag pairs, returns a set of objects `{($STATEMENT_SUBJECT, ...), ($STATEMENT_TAG, ...)}`      |
