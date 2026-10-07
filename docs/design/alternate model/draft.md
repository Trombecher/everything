# Everything Object Model V2

This is version two of the Everything object model. It solves a lot of issues with the first one, at the expense of adding another object type.

## Objects

**An _object_ is either an atom, a set of objects, or a construction.**

### Atoms

An _atom_ is an object whose semantic meaning is atomic, it is inherently uncomposed and has no internal structure. An atom is identified by a non-negative integer called "id". Ids of atoms

- must capture the semantic meaning of their atoms uniquely and
- must not carry any semantic meaning.

Thus, the id of an atom is the semantic hash of the atom's meaning. It is STRONGLY recommended to use some combination of time and randomness (like ULIDs) to generate atom ids, even though this technically contradict the semantic condition.

### Constructions

A _construction_ is a combination of two objects, written `a b`. Constructions construct properties from a "property constructor" object (here `a`) and a "value" object (here `b`).

The purpose of constructions is to give objects "roles" when used in larger structures. The object model itself cannot impose constraints on what objects can be used as property constructors but later, when describing knowledge, constraints will be laid out. Semantically, a construction may then be interpreted as the construction of a "property" (which an object then may have).

### Equality

Objects can be compared using structural equality. If your model encodes semantic meaning uniquely into object structure, then two objects are structurally equal if and only if they are semantically equal. This is a VERY useful property to have and one of the main goals of Everything: providing a framework to allow users to quantify semantic meaning.

## Examples
