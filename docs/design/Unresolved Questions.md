# How do you prove that you can model anything in Everything?

# Should child expressions of nodes required to be nodes?

# Open vs. closed definitions.

(This question needs to be considered although it is not that relevant for the database model.)

## Examples

* The definition of integers is _closed_, meaning that for a composite object to be an integer, it must not have any other unrelated properties.
* The definition for sets is _open_, meaning something can be a set, even if it has unrelated properties.

## Gemini Answer

Neither closed nor open definitions are universally better; their utility depends entirely on whether strict structural equality or flexible composition is the priority for the domain you are modeling. In the Everything model, two objects are considered equal if they share the exact same structure, meaning structural equivalence maps directly to semantic equivalence. Because of this rule, choosing between an open or closed definition fundamentally changes how an object behaves in your database.

**Closed Definitions: Prioritizing Strict Identity**
Closed definitions require an object to have no unrelated properties. This design is highly effective for atomic, mathematical, or foundational data types where extraneous information would corrupt the object's identity.

* **Predictable Equality:** By defining an integer as a composite object that *only* has a single `$SUCCESSOR_OF` or `$PREDECESSOR_OF` property, the model guarantees there is exactly one structural representation for any specific integer.

* **Normalization:** Rational numbers enforce closed boundaries by demanding that *every* tag on the object is either `$NUMERATOR` or `$DENOMINATOR`. If a rational number were open, you could theoretically have a fraction with a "color" or "timestamp" property. Structurally, that object would no longer equal a bare fraction of the same value, breaking mathematical equality.

**Open Definitions: Prioritizing Composition**
Open definitions allow an object to act as a specific entity while freely holding unrelated properties, which is crucial for extensibility and real-world modeling.

* **Rich Data Structures:** Every object in the Everything model is considered a set, and an object belongs to a set if it is the value of a `$CONTAINS` property on that set. This means any complex composite object—such as a user profile or a query result—can organically hold items and act as a set without needing to be wrapped in a strict, isolated container.

* **Contextual Expansion:** Real-world entities accumulate properties over time. When querying the knowledge base, a composite object might actually possess more properties than its core definition implies, as additional statements can be made about it. Open definitions accommodate this seamlessly.

If an entity's semantic meaning is compromised by the presence of a foreign property (like a number or a byte), a closed definition is required to preserve equivalence. If the entity is a container, trait, or real-world concept meant to absorb context alongside its primary function, an open definition provides the necessary flexibility.

# Semantic meaning of tags: transitive enforcement vs. decoupled primitives

This topic asks the following question:

> Should an object's "validity" be enforced by the tags the object is composed of (structurally) or through a function that checks the invariants?

The question really boils down to whether semantic meaning of a tag should extend to other tags (or if this invariant is checked by a tag that represents the whole semantic meaning on their own). 

## Example

Applying this question to the definition of integers:

Should `$SUCCESSOR_OF` and `$PREDECESSOR_OF` prevent you from constructing objects with them that are no integers (via their tag constraints)? Or should any tag requiring integers just check for themselves (using a function)?

## Arguments in favour of transitive enforcement (TE)

* TE guarantees that malformed intermediate objects can never exist in knowledge, protecting structural equivalence at the point of insertion.
* DP allows non-sensical or corrupted composite structures to persist in knowledge unhindered as long as no query or consumer evaluates them.

## Arguments in favour of decoupled primitives (DP)

* TE tightly couples primitive tags to each other and requires recursive validation costs every time a statement is added.
* DP keeps individual tags single-purpose, composable, and cheap to store.

## Gemini Added:

In the Everything data model, composite objects are identified solely by their intrinsic structure. For foundational types (integers, rational numbers, lists) where the structure is the data type itself, tag semantics must extend down the object tree to preserve the integrity of structural equality. For high-level domain modeling (such as `$PERSON` or `$AGE`), tags should remain simple primitives and delegate validation to whole-concept predicate functions.
