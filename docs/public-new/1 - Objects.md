# Chapter 1 - Objects

_This is version two of the Everything object model. It solves a lot of issues with the first one, at the expense of adding another object type._

## Things

Living beings talk and think about things. These things are their domain of discourse.

The Everything object model is a meta-model that allows the user to quantify things. The Everything model's representation of a thing is the _object_. The act of determining a bijection between things and objects is called _modelling_.

It is the user's responsibility to ensure that models are well-founded (i.e. are true bijections). All that the Everything object model sees are objects. The Everything object model therefore assumes that the model is well-founded. The user has an incentive to provide good models because they are the ones using the Everything object model.

## Objects

**An _object_ is either an atom, a set of objects, or a construction.**

### Atoms

An _atom_ is an object whose semantic meaning is atomic: it is inherently uncomposed and has no internal structure. An atom is identified by a non-negative integer called its _id_. Ids of atoms:

- must capture the semantic meaning of their atoms uniquely, and
- must not carry any semantic meaning.

Thus, the id of an atom is the semantic hash of the atom's meaning. It is STRONGLY recommended to use some combination of time and randomness (like ULIDs) to generate atom ids, even though this technically contradicts the semantic condition.

Throughout these docs (and when talking about atoms) it is beneficial to just use text in `snake_case` to denote atoms. This encodes the semantic meaning directly in the written form, making it easier to work with them. However, these are only (unique) text aliases. The real integer values of globally used atom aliases can be looked up in [`crates/everything_2/src/model/atoms.rs`](../../crates/everything_2/src/model/atoms.rs).

### Sets

A _set_ is a collection of objects. It is written as a comma-separated collection of objects enclosed in curly braces, for example `{a, b, c}`. The empty set is written `{}`. Sets are unordered: `{a, b}` and `{b, a}` denote the same object.

### Constructions

A _construction_ is a combination of two objects, written `a b`. Constructions construct properties from a "property constructor" object (here `a`) and a "value" object (here `b`).

The purpose of constructions is to give objects "roles" when used in larger structures. The object model itself cannot impose constraints on what objects can be used as property constructors but later, when describing knowledge, constraints will be laid out. Semantically, a construction may then be interpreted as the construction of a "property" (which an object then may have).

Constructions associate to the right: `a b c == a (b c)`.

### Equality

Objects can be compared using structural equality. If your model encodes semantic meaning uniquely into object structure, then two objects are structurally equal if and only if they are semantically equal. This is a VERY useful property to have and one of the main goals of Everything: providing a framework to allow users to quantify semantic meaning.

## Examples

You can use those definitions to construct objects as representatives of things.

### Natural numbers

You can model the natural numbers recursively by following a similar approach to the Peano axioms:

- `zero` is an atom and
- `n + 1` is modelled by `{successor_of n}`.

Thus, 1 is `{successor_of zero}` and 2 is `{successor_of {successor_of zero}}`.

### Model of itself

You can model the Everything object model in itself.

- Atoms can be modelled as `{atom <n>}` where `<n>` is a natural number.
- Constructions can be modelled as `{constructor <c>, value <v>}` where `<c>` and `<v>` are both modelled objects.
- Sets can be modelled as a set whose elements are `{contains <o>}` for each element `<o>` of the modelled set. The empty set is modelled as `{}`.
