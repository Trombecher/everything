# Chapter 2 - Knowledge

We define _knowledge_ as a set of statements. A statement is a combination of an atom `a` with an object `o`, written `a is o`. Mathematically, it is a pair. Semantically, a statement states that an atom `a` has the property `o` and therefore is an element of the set of all objects that have this property.

## Validation

To prevent knowledge from reaching arbitrariness, Everything validates knowledge. Knowledge `K` is _valid_ iff for every statement `a is p ∈ K`, `p` is a valid object and `p` is a property in `K`. This ensures that all objects are valid and that only properties are being used as properties.

Models can restrict themselves by imposing arbitrary constraints on values in property constructions and on sets where properties are used. These constraints are modelled directly in Everything with a functional computation system. This system can evaluate objects and is laid out in [the next chapter](./3%20-%20Computation.md).

### Properties

In Everything, every property is either non-constructed or constructed. You can construct properties via constructions with property constructors. The atoms `property` and `property_constructor` are declared in the base knowledge: both are property constructors with the constraint `{{}}`, which accepts every subject and value.

Formally, an object `p` is a property of `o` in knowledge `K` iff one of the following holds:

- `p` is an atom, `p is property f ∈ K`, and `{node_call_callee f, node_call_with o}` evaluates to a truthy object;
- `p` is a construction `c v`, `c is property_constructor f ∈ K`, and `{node_call_callee {node_call_callee f, node_call_with o}, node_call_with v}` evaluates to a truthy object; or
- `p` is a set, `property f ∈ p`, and `{node_call_callee f, node_call_with p}` evaluates to a truthy object.

### Valid Objects

The motivation behind valid objects is to check that all property constructions in that object are valid. Formally, an object `o` is _valid_ in knowledge `K` iff one of the following holds:

- `o` is an atom;
- `o` is a construction `c v`, `c` and `v` are valid objects, `c is property_constructor f ∈ K`, and `{node_call_callee {node_call_callee f, node_call_with o}, node_call_with v}` evaluates to a truthy object; or
- `o` is a set and every object `e ∈ o` is valid.

## Examples

For brevity, the examples use `x |-> body` for a function whose parameter is referenced as `x`, and `{{}}` for a truthy object that accepts everything.

```
# `person` is a property that accepts all subjects.
person is property {{}}

# `date_time`, `born`, and `child_of` are property constructors.
date_time is property_constructor {{}}
born is property_constructor {{}}
child_of is property_constructor subject |-> value |-> (subject is person and value is person)

David is person
David is born {date_time 10234823948}
David is child_of Anna

Anna is person
Anna is born {date_time 45385034985039}
```
