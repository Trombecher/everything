# Knowledge

We define _knowledge_ as a set of statements. A statement is a combination of an atom `a` with an object `o`, written `a is o`. Mathematically, it's a pair. Semantically, a statement states that an atom `a` has the property `o` and therefore is element of the set of all objects that have this property.

## Validation

To prevent knowledge from reaching arbitrariness, Everything validates knowledge. Knowledge `K` is _valid_ iff for every statement `a is p ∈ K`, `p` is valid and a property in `K`. This ensures that all objects are valid and that only _properties_ are being used as properties.

Models can restrict themselves by imposing arbitrary constraints on values in property constructions and on sets where properties are used. These constraints are modeled directly in Everything with a functional computation system. This system can evaluate objects and is layed out in [the next chapter](./3%20-%20Computation.md).

### Properties

In Everything, every property is either non-constructed or constructed. You can construct properties via constructions with property constructors. Formally:

An object `p` is a property of `o` in knowledge `K` iff

- `p` is an atom implies that `p is property f ∈ K` and `{node_call_callee f, node_call_value o}` evaluates to a truthy object; and
- `p` is a construction `c v` implies that `c is property_constructor f ∈ K` and `{node_call_callee {node_call_callee f, node_call_value o}, node_call_value v}` evaluates to a truthy object; and
- `p` is a set implies that `property f ∈ p` and `{node_call_callee f, node_call_value p}` evaluates to a truthy object.

### Valid Objects

The motivation behind valid objects is to check that all property constructions in that object are valid. Formally:

TODO

An object `o` is _valid_ in knowledge `K` iff

- `o` is a construction `c v` implies that `c` and `v` are valid objects, `c is property_constructor f ∈ K`, and `(f p) v` is truthy; and
- `o` is a set implies that every object `e ∈ o` is valid.

## Examples

```
person is property {{}}
date_time is property_constructor {{}}
child_of is property_constructor _ |-> $person |-> $person is person

David is person
David is born {date_time 10234823948}
David is child_of Anna

Anna is person
Anna is born {date_time 45385034985039}
```
