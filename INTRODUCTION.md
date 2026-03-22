# Introduction To Everything

## Objects And Structures

_Objects_ are either abstract objects or structures. An _abstract object_ is just a natural number, an identifier. A structure is a set of properties, a property being a doublet of the form (tag, value), both objects.

Conceptually, abstract objects derive their semantics through their identifier **which should carry no semantic meaning** (the number should not mean anything). Structures derive their semantic meaning through the properties they are defined by.

### Notation

Abstract objects are written with an `@`. Examples include `@43`, `@4538503485`, and `@2`. Structures are written this way:

```
{(<<OBJECT>>, <<OBJECT>>) (<<OBJECT>>, <<OBJECT>>) ...}
```

Properties are written as doublets with `<<OBJECT>>` being a placeholder for any object. Properties in structures may also have a separating/trailing comma. Here is an example structure:

```
{(@1, {(@534, @3)}), (@4, {})}
```

---

There is no more to structures, objects, and notation. What follows now is the interpretation of structures that Everything does. Any numbers chosen for abstract objects are arbitrary and carry no semantic meaning. You could even swap them out.

Generally, these abstract ids should be universally unique.

## Sets

Everything models sets with the tag `@1` "contains". These are examples for sets:

```
{}                    <- empty set
{(@534, @13)}         <- also the empty set
{(@1, {})}            <- set that contains the empty object
{(@1, @42) (@1, @90)} <- set that contains @42 and @90
```

## Statements

A _statement_ is an object that has a subject with `@4` (statement.subject), a tag with `@5` (statement.tag), and a value `@6` (statement.value).

For example this is a statement:

```
{
    (@4, @5345389) <- subject
    (@5, @9034593) <- tag
    (@6, @4353459) <- value
}
```

## Booleans

A set with no items is "false". A set with one or more items is "true".

## Natural Numbers

Everything models the natural numbers with nested successors. The abstract "zero" object is `@9` and the "successor of" tag is `@10`. For example `{(@10, {(@10, @9)})}` would be 2, meaning "successor of successor of 0".

## Knowledge

_Knowledge_ is a set of statements in which every structure is valid. More about validity later.

## Axiomatic

To state anything valid, we have to introduce the axiomatic primitive `@2`. In statements, Everything only allows you to use an object as a tag if **it is axiomatic**, meaning it has `@2`. You have to state that. In _this_ statement you have to specify a function or expression that validates that use of a subject and value with that tag.

### Example

This is best illustrated with an example. Let's say we want to model the following situation: we have an attribute "owns car" and an attribute "owns blue car" that we want to model. "owns car" should be applicable to anything, but "owns blue car" should only be a refinement to people that own a car.

Let `@111111` be "owns car" and `@222222` be "owns blue car". Then these statements (with `<<...>>` being templates we will fill out later)

```
{(@4, @111111), (@5, @2), (@6, <<ALWAYS>>)}
{(@4, @222222), (@5, @2), (@6, <<ONLY WHEN @111111>>)}
```

mean "`@111111` is axiomatic and can always be used with any value" and "`@222222` is axiomatic and can only be used when the subject has `@111111`".

Now we do this for `@333333` and `@444444` being some people:

```
{(@4, @333333), (@5, @111111), (@6, {})}
{(@4, @333333), (@5, @222222), (@6, {})}

{(@4, @444444), (@5, @222222), (@6, {})} <- this is not possible
{(@4, @444444), (@5, @333333), (@6, {})} <- this is also not possible
```

The first statement states that `@333333` owns a car. The second statement states that `@333333` owns a blue car (which is allowed, since they own a car).

The third statement is not valid since `@444444` does not own any car, so they can't own a blue car by definition of `@222222`. The fourth statement tries to use `@333333` as a tag but `@333333` is not `@2` (axiomatic), so this is also not allowed.

Later you will see how to express these constraints instead of placeholders.

## Computation / Reflection

Everything contains a computation system which is basically an extension of lambda calculus. It contains nodes which are just structures themselves. The engine then evaluates the nodes, reducing expressions.

### Computation / Parameter Nodes

The computation node / function is denoted with `@3`. The corresponding value is the function body. If the function is invoked with a parameter (either by the engine or other functions) then the function body will be evaluated with the parameter value.

Parameter references will then be replaced by their values on invocation. Parameter references are denoted with `@15` with the value being a natural number denoting the "relative depth". A depth of 0 means that this parameter reference is referencing the innermost function relative to that parameter reference. 1 means the wrapping function of the function addressed by 0, and so on.

This is best illustrated with examples:

| Structure                              | Non-normative Textual Representation         |
|:--------------------------------------:|:--------------------------------------------:|
| `{(@3, @5345345`                       | `x \|-> @5345345` (just a constant function) |
| `{(@3, {(@15, @9)})}`                  | `x \|-> x`                                   |
| `{(@3, {(@3, {(@15, @9))}})}`          | `x \|-> y \|-> y`                            |
| `{(@3, {(@3, {(@15, {(@10, @9)}))}})}` | `x \|-> y \|-> x`                            |

### Logical Primitives

There are nodes for "and" `@13`, "or" `@20`, "xor" `@21`, and "not" `@22`.

| Primitive   | Structure                  | Meaning                                             |
|:-----------:|:--------------------------:|:---------------------------------------------------:|
| `@13` "and" | `{(@13, ...), (@13, ...)}` | True iff all values (evaluated) are true            |
| `@20` "or"  | `{(@20, ...), (@20, ...)}` | True iff at least one value (evaluated) is true     |
| `@21` "xor" | `{(@21, ...), (@21, ...)}` | True iff only exactly one value (evaluated) is true |
| `@22` "not" | `{(@22, ...)}`             | True iff the value (evaluated) is false             |

## Dynamic Queries

The computation can inspect itself and run queries against its own knowledge. For that, there exists the tag `@18` "query". It accepts a value in a form of a statement. 