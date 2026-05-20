# Chapter 2 - Introduction To The _Everything Data Model_

The _Everything Data Model_ is an interpretation of a family of structures and abstract objects. To interpret, the model assigns meaning/semantics to some abstract objects. **The real identifiers are omitted because they are meaningless and bloat the text.** You can look up the integer values in [this file](../crates/everything_structures/src/abstracts.rs) and in [this file](../crates/everything/src/ext/abstracts.rs).

In the following sections, aliases are used to talk about the abstract objects. They are written in inline code `@LIKE_THIS`.

## Goals

The model tries to be the basis for any data model. This is a bit like JSON schema, which itself provides a meta-schema, validating itself. But the model includes a lambda calculus-like programming language.

## Vocabulary / Definitions

* The _knowledge_ is a structure which is a parameter to each expression, a sort of context which influences evaluation.

* An object _has a tag with a value_ iff there exists a statement in the knowledge that includes the object, the tag, and the value; or, if the object is a structure, (tag, value) is a property on object.

* An object _has a tag_ iff there exists a value such that the object has a tag with this value.

* An object is _axiomatic_ iff it has `@AXIOMATIC`.

* An object _a_ is included in another object _b_ iff _b_ has `@CONTAINS` with the value _a_.

## Sets

Everything models sets with the abstract object `@CONTAINS`. The set values of an object _set_ consist of every object that is included in _set_. These are examples for sets:

```
{}                                <- empty set
{(@534, @13)}                     <- also the empty set
{(@CONTAINS, {})}                  <- set that contains the empty object
{(@CONTAINS, @42) (CONTAINS, @90)} <- set that contains @42 and @90
```

## Statements

A _statement_ is an object that has `@STATEMENT_SUBJECT` with a subject, `@STATEMENT_TAG` with a tag, and `@STATEMENT_VALUE` with a value. "subject", "tag", and "value" are still just objects but with different "roles".

For example this is a statement:

```
{
    (@STATEMENT_SUBJECT, @5345389) <- subject
    (@STATEMENT_TAG, @9034593)     <- tag
    (@STATEMENT_VALUE, @4353459)   <- value
}
```

Conceptually,

* the subject is the **object in question** which the statment is about.
* The tag is the **"attribute" (or "predicate") of the subject**.
* The value is **associated data** specific to that subject and tag.

## Booleans

A set with no items is "false". A set with one or more items is "true".

## Integers

Everything constructs the integers recursively through nested structures:

* The _zero_ is represented by the abstract object `@ZERO`.
* The successor of a non-negative integer is represented with the abstract object `@SUCCESSOR_OF`.
* The predecessor of a non-positive integer is represented with the abstract object `@PREDECESSOR_OF`.

By repeatetly constructing structures as successors or predecessors, you can get any integer. An integer is _negative_ iff it has `@PREDECESSOR_OF`. An integer is positive iff it has `@SUCCESSOR_OF`. An integer must not have both.

This approach of constructing integers is similar to the Peano-axioms definition of natural numbers.

### Examples

```
# -3
{(@PREDECESSOR_OF, {(@PREDECESSOR_OF, {(@PREDECESSOR_OF, @ZERO)})})}

# 0
@ZERO

# 1
{(@SUCCESSOR_OF, @ZERO)}

# 2
{(@SUCCESSOR_OF, {(@SUCCESSOR_OF, @ZERO)})}
```

## Knowledge

_Knowledge_ is a set of statements which are _valid_.

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

For that let's define aliases for abstract objects: `@PERSON` and `@AGE`. Content in `<<brackets>>` is placeholder. We can extend the base knowledge with these statements:

```
{
    (@STATEMENT_SUBJECT, @PERSON)
    (@STATEMENT_TAG, @AXIOMATIC)
    (@STATEMENT_VALUE, {(@CONTAINS, {})})
}

{
    (@STATEMENT_SUBJECT, @AGE)
    (@STATEMENT_TAG, @AXIOMATIC)
    (@STATEMENT_VALUE, <<constraint: only on person and with non-negative integer>>)
}
```

The first statement states:

> `@PERSON` is axiomatic and has the constraint `{(@CONTAINS, {})}` which is "true", so all subjects and values on uses are accepted.

The second statement states:

> `@AGE` is axiomatic and has a constraint that limits subjects on uses to people and values on uses to non-negative integers.

How this constraint is encoded, [will be shown later](#encoding-the-constraint-from-the-example) and is currently irrelevant. Now we get play around with our statements by adding new ones for `@ALICE`, `@BOB`, and `@LOVE`:

```
{
    (@STATEMENT_SUBJECT, @ALICE)
    (@STATEMENT_TAG, @PERSON)
    (@STATEMENT_VALUE, {})
}
{
    (STATEMENT_SUBJECT, @BOB)
    (STATEMENT_TAG, @PERSON)
    (STATEMENT_VALUE, {})
}
```

The first statement states:

> `@ALICE` is a `@PERSON`.

The second statement states:

> `@BOB` is a `@PERSON`.

Since being a person does come with any context in our example, we just pass in `{}` as the value which is happily accepted by the constraint on `@PERSON`.

Now it gets interesting if we add the following statements:

```
{
    (@STATEMENT_SUBJECT, @ALICE)
    (@STATEMENT_TAG, @AGE)
    (@STATEMENT_VALUE, 42)
}
{
    (@STATEMENT_SUBJECT, @BOB)
    (@STATEMENT_TAG, @AGE)
    (@STATEMENT_VALUE, @ALICE)
}
{
    (@STATEMENT_SUBJECT, @LOVE)
    (@STATEMENT_TAG, @AGE)
    (@STATEMENT_VALUE, 69)
}
```

The first statement states:

> `@ALICE`'s `@AGE` is 42.

The second statement states:

> `@BOB`'s `@AGE` is `@ALICE`...?

...this cannot be right? In fact it is not valid. Because `@AGE` is used in a statement as the tag, its constraint must return "true" on

* `@BOB` as a subject and
* `@ALICE` as a value.

Although `@BOB` is a `@PERSON`, `@ALICE` is not a non-negative integer. So this statement invalidates the whole knowledge. Same with the third statement:

> `@LOVE`'s `@AGE` is 67.

Although 67 is a non-negative integer, `@LOVE` is not a person. Therefore, the engine tells you that this is not valid.

## Computation

_Everything_ contains a computation system which is basically an extension of lambda calculus. It assigns certain structures meaning. These structures are called nodes. The engine can evaluate nodes.

### Basis: Functions, Parameters, And Control-Flow

The function node is denoted with `@FUNCTION`. The associated value can be interpreted as the function body. If the function is called with a parameter, then the function body will be evaluated with the parameter value.

Parameter references resolve to the values which the wrapping functions have been called with. Parameter references are denoted with `@NODE_PARAMETER`. The associated value is a non-negative number denoting the "relative depth". A depth of 0 means that this parameter reference is referencing the **innermost function from the POV** of that parameter reference node. A depth of 1 means the wrapping function of the function addressed by depth 0, and so on.

You can use `@NODE_FUNCTION_SELF` to reference a wrapping function, just like `@NODE_PARAMETER`. You can use it to implement recursive functions.

| Structure                                                                  | Meaning                                                        |
|----------------------------------------------------------------------------|----------------------------------------------------------------|
| `{(@FUNCTION, ...)}`                                                       | A function (equivalent to an "abstraction" in lambda calculus) |
| `{(@NODE_PARAMETER, ...)}`                                                 | A reference to a function parameter                            |
| `{(@NODE_FUNCTION_SELF, ...)}`                                             | A reference to a wrapping function                             |
| `{(@NODE_IF_CONDITION, ...), (@NODE_IF_THEN, ...), (@NODE_IF_ELSE, ...)}`  | A conditional (or "select") node                               |
| `{(@NODE_CALL_CALLEE, ...), (@NODE_CALL_WITH, ...)`                        | A call of a node (almost always a function) with a value       |

#### Examples

This is best illustrated with examples:

| Structure                                                 | Non-normative Textual Representation         |
|-----------------------------------------------------------|----------------------------------------------|
| `{(@FUNCTION, @5345345)}`                                 | `x \|-> @5345345` (just a constant function) |
| `{(@FUNCTION, {(@NODE_PARAMETER, 0)})}`                   | `x \|-> x`                                   |
| `{(@FUNCTION, {(@FUNCTION, {(@NODE_PARAMETER, 0))}})}`    | `x \|-> y \|-> y`                            |
| `{(@FUNCTION, {(@FUNCTION, {(@NODE_PARAMETER, 1))}})}`    | `x \|-> y \|-> x`                            |
| `{(@FUNCTION, {(@NODE_FUNCTION_SET, 0)})}`                | `f := x \|-> f`                              |
| `{(@FUNCTION, {(@FUNCTION, {(@NODE_FUNCTION_SET, 1)})})}` | `f := x \|-> y \|-> f`                       |

### Logic

| Structure                                           | Meaning                                                                  |
|-----------------------------------------------------|--------------------------------------------------------------------------|
| `{(NODE_AND_LEFT, ...), (NODE_AND_RIGHT, ...)}`     | True iff both the left evaluates to "true" and the right (short circuit) |
| `{(NODE_OR_LEFT, ...), (NODE_OR_RIGHT, ...)}`       | True iff the left or the right evaluates to "true" (short circuit)       |
| `{(NODE_XOR_LEFT, ...), (NODE_XOR_RIGHT, ...)}`     | True iff either the left or the right evaluates to "true"                |
| `{(NODE_NOT, ...)}`                                 | True iff inner evaluates to "false"                                      |

### Arithmetic

| Structure                                                     | Domain    | Meaning                                             |
|---------------------------------------------------------------|-----------|-----------------------------------------------------|
| `{(NODE_ADD_LEFT, ...), (NODE_ADD_RIGHT, ...)}`               | Integers  | Evaluates and computes `left + right`               |
| `{(NODE_MULTIPLY_LEFT, ...), (NODE_MULTIPLY_RIGHT, ...)}`     | Integers  | Evaluates and computes `left * right`               |

### Set Manipulation

| Structure                                                     | Meaning                                                                 |
|---------------------------------------------------------------|-------------------------------------------------------------------------|
| `{(@NODE_UNION_LEFT, ...), (@NODE_UNION_RIGHT, ...)}`         | Merges the set values of left and right                                 |
| `{(@NODE_COUNT, ...)}`                                        | Counts the set values of the inner                                      |
| `{(@NODE_MAP_SET, ...), (@NODE_MAP_MAPPER, ...)}`             | Maps the set values of the input set with a node                        |
| `{(@NODE_FILTER_SET, ...), (@NODE_FILTER_FILTER, ...)}`       | Retains all set values for which the filter node returns a truthy value |

### Queries

There are nodes to query the knowledge. For each query, first the intrinsic values of the subject (if there is one and it is a structure) are used and then the whole knowledge is queried. Queries always return sets.

That means that each structure may actually have more properties that the structure is defined by. Because you can use the knowledge to state additional things about a structure.

> [!NOTE]
>
> Every object is a "set". What is meant is that return value of a query will be a structure that has no other (intrinsic) tags than `@CONTAINS`.

| Structure                                                                                         | Meaning                                                                                                                                 |
|---------------------------------------------------------------------------------------------------|-----------------------------------------------------------------------------------------------------------------------------------------|
| `{(@NODE_QUERY, {(@STATEMENT_SUBJECT, ...), (@STATEMENT_TAG, ...), (@STATEMENT_VALUE, ...)})}`    | Checks if this statement exists; returns a truthy values if yes, `{}` otherwise                                                         |
| `{(@NODE_QUERY, {(@STATEMENT_SUBJECT, ...), (@STATEMENT_TAG, ...)})}`                             | Queries the subject for values it has on the given tag and returns a set of all values                                                  |
| `{(@NODE_QUERY, {(@STATEMENT_SUBJECT, ...), (@STATEMENT_VALUE, ...)})}`                           | Queries the subject for tags it has with the given value and returns a set of all tags                                                  |
| `{(@NODE_QUERY, {(@STATEMENT_TAG, ...), (@STATEMENT_VALUE, ...)})}`                               | Queries all subjects that have this tag with this value and returns a set of all those subejcts                                         |
| `{(@NODE_QUERY, {(@STATEMENT_SUBJECT, ...)}`                                                      | Queries the subject for tags and value pairs; returns a set of objects `{(@STATEMENT_TAG, ...), (@STATEMENT_VALUE, ...)}`               |
| `{(@NODE_QUERY, {(@STATEMENT_TAG, ...)}`                                                          | Queries the knowledge for all subject and value pairs; returns a set of objects `{(@STATEMENT_SUBJECT, ...), (@STATEMENT_VALUE, ...)}`  |
| `{(@NODE_QUERY, {(@STATEMENT_VALUE, ...)}`                                                        | Queries the knowledge for all subject and tag pairs, returns a set of objects `{(@STATEMENT_SUBJECT, ...), (@STATEMENT_TAG, ...)}`      |

## More Examples

### Encoding The Constraint From The Example

Here is the example again:

> * Some objects are people.
> * Some objects have an age but only people can have an age.
> * Age must be a non-negative integer.

So the constraint on `@AGE` must validate that each associated subject is a person AND each associated is a non-negative integer. Here is the constraint:

```
{(@FUNCTION, {
    {(@FUNCTION, {
        (@NODE_AND_LEFT, {
            (@NODE_QUERY, {
                (@STATEMENT_SUBJECT, {
                    # the outer parameter: subject
                    (@NODE_PARAMETER, 1)
                })
                (@STATEMENT_TAG, @PERSON)
            })
        })
        (@NODE_AND_RIGHT, {
            (@NODE_OR_LEFT, {
                (@NODE_EQUALS_LEFT, {
                    # the inner parameter: value
                    (@NODE_PARAMETER, 0)
                })
                (@NODE_EQUALS_RIGHT, 0)
            })
            (@NODE_OR_RIGHT, {
                (@NODE_QUERY, {
                    (@STATEMENT_SUBJECT, {
                        (@NODE_PARAMETER, 0)
                    })
                    (@STATEMENT_TAG, @SUCCESSOR_OF)
                })
            })
        })
    })}
})}
```
