# Introduction To Everything

## Objects And Structures

_Objects_ are either abstract objects or structures. An _abstract object_ is just a natural number, an identifier. A structure is a set of properties, a property being a doublet of the form (tag, value), both objects.

Conceptually, abstract objects derive their semantics through their identifier **which should carry no semantic meaning** (the number should not mean anything). Structures derive their semantic meaning through the properties they are defined by.

### Notation

Abstract objects are writting with an `@`. Examples include `@43`, `@4538503485`, and `@2`. Structures are written this way:

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

## Axiomatic

To state anything, we have to introduce the axiomatic primitive `@2`. If you want to state something, you have to involve a subject, a tag, and a value (all objects). Everything only allows you to use a tag if it is axiomatic, meaning it has `@2`. You have to state that. In _this_ statement you have to specify a function or expression that validates that use of a subject and value with that tag.

### Example

This is best illustrated with an example. Let's say we want to model the following situation: we have an attribute "owns car" and an attribute "owns blue car" that we want to model. "owns car" should be applicable to anything, but "owns blue car" should only be a refinement to people that own a car.

Let `@111111` be "owns car" and `@222222` be "owns blue car". Then these statements (with `<<...>>` being templates we will fill out later)

```
(@111111, @2, <<ALWAYS>>)
(@222222, @2, <<ONLY WHEN @111111>>)
```

mean "`@111111` is axiomatic and can always be used with any value" and "`@222222` is axiomatic and can only be used when the subject has `@111111`".

Now we do this for `@333333` and `@444444` being some people:

```
(@333333, @111111, {})
(@333333, @222222, {})

(@444444, @222222, {}) <- this is not possible
(@444444, @333333, {}) <- this is also not possible
```

The first statement states that `@333333` owns a car. The second statement states that `@333333` owns a blue car (which is allowed, since they own a car).

The third statement is not valid since `@444444` does not own any car, so they can't own a blue car by definition of `@222222`. The fourth statement tries to use `@333333` as a tag but `@333333` is not `@2` (axiomatic), so this is also not allowed.

Later you will see how to express these constraints instead of placeholders.

## Natural Numbers

Everything represents the natural numbers with nested successors. The abstract "zero" object is `@9` and the "successor of" tag is `@10`. For example `{(@10, {(@10, @9)})}` would be 2, meaning "successor of successor of 0".