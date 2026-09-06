# Chapter 2 - Introduction To The _Everything Data Model_

The _Everything Data Model_ is an interpretation of a family of objects. It assigns meaning and semantics to some abstract objects. **The real identifiers of the abstract objects are omitted because they bloat the text.** You can look up the integer values in [this file](../crates/everything_objects/src/abstracts.rs) and in [this file](../crates/everything/src/ext/abstracts.rs).

In the following sections, aliases are used to talk about the abstract objects. They are written inline `$LIKE_THIS`.

## Goals

The model tries to be the basis for any data model. This is a bit like JSON schema which itself provides a meta-schema, validating itself. But this model includes a lambda calculus-like programming language.

## Vocabulary / Definitions

* The _knowledge_ is a composite object which is a parameter to each expression. This is the data of the database expressions may be evaluated in.

* An object _has a tag with a value_ iff there exists a statement in the knowledge that includes the object, the tag, and the value; or, if the object is a composite object, `tag = value` is a property in the object.

* An object _has a tag_ iff there exists a value such that the object has a tag with this value.

* An object is _axiomatic_ iff it has `$AXIOMATIC`.

* An object _a_ is included in another object _b_ iff _b_ has `$CONTAINS` with the value _a_.

## Sets

Everything models sets with the abstract object `$CONTAINS`. The set values of an object _set_ consist of every object that is included in _set_. Examples for sets include:

```
{}                               <- empty set
{$534: $13}                      <- also the empty set
{$CONTAINS: {}}                  <- set that contains the empty object
{$CONTAINS: $42, $CONTAINS: $90} <- set that contains $42 and $90
```

## Statements

A _statement_ is an object that has `$STATEMENT_SUBJECT` with a subject, `$STATEMENT_TAG` with a tag, and `$STATEMENT_VALUE` with a value. "subject", "tag", and "value" are still just objects but with different "roles".

For example this is a statement:

```
{
    $STATEMENT_SUBJECT: $5345389,
    $STATEMENT_TAG: $9034593,
    $STATEMENT_VALUE: $4353459
}
```

Conceptually,

* the subject is the **object in question** which the statment is about.
* The tag is the **"attribute" (or "predicate") of the subject**.
* The value is **associated data** specific to that subject and tag.

## Booleans

A set with no items is "false". A set with one or more items is "true".

## Integers

Everything constructs the integers recursively through object nesting. An object is an integer iff

* it is equal to the abstract object `$ZERO` or
    * it either has a single tag `$SUCCESSOR_OF` with the associated value being an integer (recursive definition) or
    * it has a single tag `$PREDECESSOR_OF` with the associated value being an integer.

By repeatetly constructing composite objects as successors or predecessors, you can get any integer. An integer is _negative_ iff it has `$PREDECESSOR_OF`. An integer is positive iff it has `$SUCCESSOR_OF`.

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

**To use an object as a tag in a statement, it must be axiomatic.** The value you provide when stating that your object is axiomatic is called the _constraint_. Constraints are expressions/nodes that **validate each specific use of your object as a tag**.

Let's say your axiomatic object _T_ is used with a subject _S_ and a value _V_. Then, first, the constraint of _T_ is called with _S_ as the parameter. Then, the result of that first call is called with _V_ as the parameter. If the result of that second call is "true", then the statement is valid. Otherwise it is not, and the engine will report your knowledge as invalid.

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

An object is an _or_ node iff

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
