# Everything Object Model V2

This is version two of the Everything object model. It solves a lot of issues with the first one, at the expense of adding another object type.

## Things

Living beings talk and think about things. These things are their domain of discourse.

The Everything object model is a meta-model that allows the user to quantify things. The Everything model's representation of a thing is the _object_. The act of determining a bijection between things and object is called modelling.

It is the user's responsibility to ensure that models are well-founded (i.e. are true bijections). All that the Everything object model sees are objects. The Everything object model therefore assumes that the model is well-founded. The user has an incentive to provide good models because they are the ones using the Everything object model.

## Objects

**An _object_ is either an atom, a set of objects, or a construction.**

A set is written as a comma separated collection of objects, enclosed in curly braces.

### Atoms

An _atom_ is an object whose semantic meaning is atomic, it is inherently uncomposed and has no internal structure. An atom is identified by a non-negative integer called "id". Ids of atoms

- must capture the semantic meaning of their atoms uniquely and
- must not carry any semantic meaning.

Thus, the id of an atom is the semantic hash of the atom's meaning. It is STRONGLY recommended to use some combination of time and randomness (like ULIDs) to generate atom ids, even though this technically contradict the semantic condition.

Throughout these docs (and when talking about atoms) it is beneficial to just use text in snake_case to denote atoms. This encodes the semantic meaning directly in the written form, making it easier to work with them. However, these are only (unique) text aliases. The real integer values of globally used atom aliases can be looked up in the source code of the implementation.

### Constructions

A _construction_ is a combination of two objects, written `a b`. Constructions construct properties from a "property constructor" object (here `a`) and a "value" object (here `b`).

The purpose of constructions is to give objects "roles" when used in larger structures. The object model itself cannot impose constraints on what objects can be used as property constructors but later, when describing knowledge, constraints will be laid out. Semantically, a construction may then be interpreted as the construction of a "property" (which an object then may have).

### Equality

Objects can be compared using structural equality. If your model encodes semantic meaning uniquely into object structure, then two objects are structurally equal if and only if they are semantically equal. This is a VERY useful property to have and one of the main goals of Everything: providing a framework to allow users to quantify semantic meaning.

## Models

## Examples

You can use those definitions to construct objects as representants of things.

### Natural numbers

You can model the natural numbers recursively by following a similar approach to the Peano axioms:

- zero is an atom and
- n + 1 is modeled by `{successor_of n}`.

1 would be `{successor_of zero}` and 2 would be `{successor_of {successor_of zero}}`.

### Model of itself

You can model the Everything object model in itself.

- Atoms can be modelled as `{atom <n>}` where `<n>` is a natural number.
- Constructions can be modelled as `{constructor <c>, value <v>}` where `<c>` and `<v>` are both modelled objects.
- Sets can be modelled as `{contains <o>, ...}` where `<o>` is a modelled object (empty set as `{}`).

# Knowledge

## Definitions

- We define _knowledge_ as a set of statements.
- A statement is a combination of an atom `A` with an object `O`, written `A is O`. Mathematically, it's a pair.
- An object `p` is a property in knowledge `K` iff
    - `p` is an atom implies that `p is property f ∈ K` and `f p` is valid; and
    - `p` is a construction `c v` implies that `c is property_constructor f ∈ K` and `(f p) v` is truthy; and
    - `p` is a set implies that `property ∈ p`.
- An object `o` is valid in knowledge `K` iff
    - `o` is a construction `c v` implies that `c` and `v` are valid objects, `c is property_constructor f ∈ K`, and `(f p) v` is truthy; and
    - `o` is a set implies that every element `e ∈ o` is valid.
- Knowledge is _valid_ iff for every statement `a is p`, `p` is valid and a property in `K`.
