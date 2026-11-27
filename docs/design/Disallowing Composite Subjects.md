# Disallowing Composite Subjects

**Status: resolved.** Only atoms can be the subjects of knowledge statements. Composites and constructions cannot. This document records why the model is built that way and why the restriction is not an arbitrary limitation but a consequence of the object model.

The older design record called the rejected alternative _Model A_ and the accepted restriction _Model B_. This document describes the accepted model directly and keeps those labels only when referring to the earlier discussion.

Text is written `"..."` in the examples for readability; in the object model it is an ordinary value (see [Chapter 4](../public/4%20-%20Common%20Models.md#text)).

## The Rule In The Current Model

[Chapter 2](../public/2%20-%20Knowledge.md) defines a statement as a pair of an atom `%a%` and an object `%o%`, written `%a% is %o%`. The subject is an atom by definition; there is no separate validation rule that rejects composites. The old model wrote a statement as a triple `(%subject%, %tag%, %value%)`. Today the tag and the value are folded into a single property object, usually a construction `%c% ·· %v%`. A _composite subject_ would be a statement `%c% is %p%` in which the first component `%c%` is a composite. Constructions are excluded as well.

The definition is tied to how properties are defined:

- the properties of an atom `%a%` in knowledge `%K%` are the objects `%p%` for which `%a% is %p% ∈ %K%`;
- the properties of a composite `%o%` are exactly the members of `%o%`; the property set is `%o%` itself;
- a construction has no properties at all, and it is forbidden to ask for them.

Every object that has properties therefore has exactly one property set. For an atom, the knowledge supplies it. For a composite, the object itself is it. This is the invariant the restriction protects. Constructions are excluded for the same reason, only more strongly: they have no property set to extend in the first place.

## The Rejected Alternative

Allowing composite subjects would mean admitting statements `%c% is %p%` where `%c%` is a composite. The properties of `%c%` would then have to be the union of its members and the properties asserted about it in the knowledge. Everything else follows from that single change.

## Why The Alternative Was Rejected

### 1. It Separates A Composite's Identity From Its Properties

Everything's central promise is that a well-founded model makes structural equality and semantic equality the same relation. A composite is exhausted by its members: `⟨a, b⟩` is the value with exactly the properties `a` and `b`. If knowledge can add `%p%` to `%c%`, then either `%c%` changes its structure, which is impossible because `%c%` is a fixed object, or `%c%` has properties that are not part of its structure.

The second option breaks the promise. Consider the empty composite and the composite containing it:

```
# Two different objects.
⟨⟩
⟨contains ·· ⟨⟩⟩
```

If `⟨⟩ is contains ·· ⟨⟩` were a statement, then `⟨⟩` would acquire the property `contains ·· ⟨⟩`. Its property set would then equal the members of `⟨contains ·· ⟨⟩⟩`. Two structurally different objects would be indistinguishable by their properties, while the empty set `{}` would contain an item in that knowledge without any object having changed. Note the direction of the problem: the composite would not gain a member, because its structure stays the same. It would gain a property outside its structure, so its structure would no longer determine what it is.

This also breaks the meaning of a composite across knowledges. A composite must mean the same thing in every knowledge base; merging two knowledges may add facts about atoms, but it must not change what a structural value is. If composite subjects were allowed, the union of two knowledges could give a composite properties that it lacks in each of them separately. [Chapter 5](../public/5%20-%20Across%20Knowledges.md) describes merging as combining statements about independently modelled entities; it would instead also be redefining values.

### 2. It Makes Canonical Representations Knowledge-Dependent

The common models rely on exact property sets. A nonzero integer has exactly one property, whose constructor is `successor_of` or `predecessor_of`. A non-integer rational number has exactly two properties, one with constructor `numerator` and one with constructor `denominator`, and every rational number has exactly one structural representation. A non-empty list has exactly one `list_item` property and exactly one `list_tail` property. For integers and rational numbers these definitions are _closed_: the structure is the data type itself, so an extra property would change the represented value (see [Chapter 4](../public/4%20-%20Common%20Models.md) and [Open versus closed definitions](./Q%20and%20A/Open%20versus%20closed%20definitions.md)).

The current model makes the canonical form explicit in the structure:

```
# 1
⟨successor_of ·· zero⟩

# Not 1: this composite has two properties instead of one.
⟨successor_of ·· zero, contains ·· ⟨⟩⟩
```

The second object is correctly not an integer. If `⟨successor_of ·· zero⟩ is contains ·· ⟨⟩` were allowed, the first object would have the same property set as the second while remaining structurally 1. Then either:

- the integer definition must ignore knowledge-added properties, which requires distinguishing "structural" properties from "asserted" properties, or
- whether an object is an integer depends on unrelated statements elsewhere in the knowledge.

Both outcomes are unacceptable. The first abolishes the idea that an object has _the_ properties; it introduces a second, hidden property set. The second makes the representation of a number depend on the knowledge it happens to live in, so the same structural value represents 1 in one knowledge and nothing in another. The old discussion already identified the contradiction: if `⟨successor_of ·· zero, contains ·· ⟨⟩⟩` counted as 1, then two structurally unequal objects would represent the same integer, violating the principle that semantically equal things have structurally equal objects. The current model resolves this by keeping the extra property in the structure, where it correctly produces a different object that is not 1.

### 3. The Forbidden Statements Are Computations, Not Knowledge

A relation whose truth is determined by the structures of its arguments does not need to be stored. The old discussion used the natural order on integers:

```
# The old discussion used zero < 1 as its example.
# Today this is a computation, not a statement:
⟨
    node_less_first ·· ⟨node_literal ·· zero⟩,
    node_less_second ·· ⟨node_literal ·· ⟨successor_of ·· zero⟩⟩
⟩
```

This evaluates to `true`. Storing the same fact as knowledge would add no information, and it could even contradict the computation if the two disagreed. The same holds for equality, set membership, containment, list length, and every other relation derivable from the representations.

Knowledge is for assertions that the structures cannot decide: that `Alice` is a person, that `David` is a child of `Anna`, that a particular date is someone's birthday. Those subjects are atoms precisely because an atom's structure decides nothing about them. The old discussion's caution still applies: it is not automatically true that _every_ relation between two composites is computable. It is true for relations that are part of the models of those values. If a relation between two composites is not computable from them, then the composites are standing in for entities, and those entities should be atoms.

The reversal observation from the old discussion also remains useful. A composite can always appear as the _value_ of a statement, so many relations can be reoriented around an atom:

```
# Instead of the rejected (address, verified_by, post_office):
post_office is verified_address ·· ⟨street ·· "Main St", city ·· "Bonn"⟩
```

The address stays a value and the atom `post_office` is the subject. When both endpoints are composites, the relation is either structural and should be computed, or it is about entities that need their own atoms.

### 4. Atoms Are Entities, Composites Are Values

This is the semantic reading of the two object kinds.

Atoms are opaque. Their ids carry no meaning, so everything about them must be asserted. A composite is a complete description: its members are everything it is. A composite can therefore carry its classification in its own structure:

```
⟨person, first_name ·· "Alice", last_name ·· "Smith"⟩
```

This object is a person because `person` is one of its members, and `person` being a member is part of its identity. No statement `⟨...⟩ is person` is needed, and adding one would be either redundant or a contradiction: if being a person matters to what the value is, it belongs in the value. Changing the last name produces a different object, which is exactly value semantics.

An entity instead receives its properties from knowledge:

```
Alice is person
Alice is name ·· ⟨first ·· "Alice", last ·· "Smith"⟩
Alice is age ·· 42
```

Now `Alice` can accumulate properties over time while remaining the same atom, which is entity semantics.

If a value needs to be talked about as an entity, the model does not pretend that it already is one. Introduce an atom for the entity and attach the value to it. For example, to record that a post office verified a particular address:

```
alice_address is address ·· ⟨street ·· "Main St", city ·· "Bonn"⟩
post_office is verified_address ·· alice_address
```

The address is still a value; the atom `alice_address` is the entity that carries the verification fact. The pattern applies to any value, including functions: a function value that needs metadata becomes an entity with an atom of its own. This is the recommended workaround when reversal is not enough. It is not a loss of expressiveness; it makes the entity/value boundary explicit. The old discussion's conclusion still holds: allowing knowledge to add properties to a composite would undermine the idea that the composite's structure completely defines the thing it represents. Disallowing composite subjects ensures that all information constituting a composite value is present in the value itself.

### 5. It Would Make Change And Versioning Incoherent

Composites are values, so a changed composite is a different object:

```
⟨name ·· "Alice", age ·· 42⟩
⟨name ·· "Alice", age ·· 43⟩
```

These are two values, not one object whose property changed. This is consistent with the decision that Everything does not provide database-level versioning: history is knowledge about entities, and entities are atoms (see [Database-Level Versioning](./Database-Level%20Versioning.md)). A composite that could also receive knowledge would be a hybrid. It would have value identity, because its structure is its identity, but entity-like accumulated properties, because knowledge could add to it. There would then be no clear answer to whether a change to that knowledge changes the value or merely records a new fact about an entity. Keeping composite subjects disallowed keeps the division clean: values are values, entities are atoms, and the user models history and change on top of entities.

### 6. It Has A Direct Implementation Cost

Statements pair an atom with a property. A property lookup for an atom consults the knowledge; a property lookup for a composite is its members, with no knowledge access at all. Allowing composite subjects would require storing or matching statements by structural object, merging structural members with asserted properties on every lookup, and deciding how asserted properties interact with definitions and constraints. It would also make the validity and meaning of a composite depend on statements elsewhere in the knowledge. Queries and computed collections would lose the guarantee that a composite is self-contained. The current rule keeps the fast paths for integers, lists, sets, and computed collections meaningful.

### 7. It Creates Footguns With Global Blast Radius

Statements are global. A statement about a composite would affect every use of an equal composite, including computed results and values that the author did not know were the same object. Adding a property to the empty composite changes every empty set in the knowledge; adding a property to the representation of an integer changes every occurrence of that integer. Because composites are pervasive as values, the effect of one statement is not local. Atoms do not have this problem: an atom is an identity, and knowledge about it is expected to be global. That is the whole point of identity.

## Old Arguments Reconsidered

### Infinite Composites

The old record offered `({$LIST_ITEM: 0}, $LIST_TAIL, {$LIST_ITEM: 0})` as an "infinite list of zeroes". In today's syntax, the corresponding statement would be `⟨list_item ·· 0⟩ is list_tail ·· ⟨list_item ·· 0⟩`. It is not infinite: both sides are finite objects. The current model has finite objects; cyclic or lazy structures would be a separate extension of the object model. Even with such an extension, the relation between a composite and its parts is structural and belongs in computation, not in knowledge.

### Polyglots

The old record argued that composite subjects would allow objects that satisfy multiple normally disjoint definitions. Polyglots are still possible today, but through structure and open definitions, not through knowledge. For example,

```
⟨person, contains ·· 1⟩
```

is both a person and a set containing `1`. `person` is a member, so being a person is part of the object's identity. Closed definitions such as integers and rational numbers deliberately exclude such overlap. No statement with a composite subject is needed in either case.

### Intrinsic Versus Extrinsic Properties

One proposal in the old discussion was to keep composite subjects but distinguish structural properties from knowledge properties. This was rejected because it reintroduces exactly the two-property-set problem. The current model has one property set per object. For a composite it is the members. If an "extrinsic" property is needed, the object is being treated as an entity, and it needs an atom.

### Canonical Direction Of A Relation

Sometimes a relation's natural direction has a composite on the left, as in "`a` is a subset of `b`" or "`a` is less than `b`". Under the restriction, that cannot be a stored statement with `a` as the subject. This is accepted:

- if the relation is structural, compute it;
- if one endpoint is an entity, orient the statement around the entity;
- if both endpoints are values and the relation is not structural, reify one of them as an atom.

The storage direction of a statement should not be confused with the mathematical direction of the relation.

## Verdict

The restriction follows from the model's foundational invariants:

- an atom's properties come from knowledge;
- a composite's properties are its structure;
- a construction has no properties;
- structural equality is semantic equality.

Allowing composite subjects would give composite objects a second, knowledge-dependent property set. It would undermine canonical representations, blur knowledge and computation, make the meaning of values depend on unrelated statements, and turn values into entities halfway. Composite values remain first-class: they can be statement values, members of other composites, and inputs and results of computation. When something must be said _about_ a value that is not computable from it, that thing is an entity and should be represented by an atom. The subject restriction is the price of keeping one object, one property set, and structural equality intact.

The resulting picture is deliberately small: knowledge is a set of atom-property pairs, and composites are self-contained values that computation can inspect. The restriction is what keeps those two worlds from leaking into each other.
