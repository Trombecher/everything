# Set Models

> **Status: resolved.** The open set model is now part of the public documentation. Every atom and every composite is a set; its items are the values of its `contains ·· %item%` properties. The model is defined in [`docs/public/4 - Common Models.md`](../public/4%20-%20Common%20Models.md#sets), the set operations are described in [`docs/public/3 - Computation.md`](../public/3%20-%20Computation.md#sets), and the validity rule that motivated the change is in [`docs/public/2 - Knowledge.md`](../public/2%20-%20Knowledge.md#valid-objects).
>
> This document is kept as the design record. It describes the older object model, why that model was chosen, and why it was replaced. The historical description uses today's notation so that it can be compared directly with the current documentation.

## The Old Model

The old object model had three kinds of objects: atoms, sets, and constructions. The kind called a _set_ is what the public documentation now calls a _composite_: an unordered, duplicate-free collection of objects, written `⟨%a%, %b%, %c%⟩`. Constructions were written with a colon at the time; today they are written `%c% ·· %v%`. Metavariables are now written `%...%`.

In that model, membership was purely structural. A set was just a collection of objects; its members were not required to be valid properties of the set, and their constraints were not run with the set as subject. There was also no separate model of a data set. The same object kind served as:

- the structure of a value, such as an integer, a list, or a record, and
- the result of a set-valued computation, such as a query, a union, or a map.

Set-taking computation nodes therefore operated directly on this structural collection kind, i.e. on what we now call composites. There was no first-class set model apart from composites, and no way for an atom to be a set.

That design was attractive for a few reasons:

- one collection kind was enough for both object structure and data collections;
- set operations could work directly on that kind, without an extra representation or wrapper;
- collections had the same structural equality as every other object;
- integers, lists, and other common values were already modelled as collections of properties, so treating data collections the same way looked uniform;
- the notation could be simple and direct.

The cost was that object structure and data-set membership were the same thing. That worked only while set membership carried no property semantics. Once validation required the members of a composite to be valid properties of that composite—the rule the current model has—the conflation became a real problem.

## Problem Description

A set-valued node should produce a _set_ of result objects: the result objects are its items. It should not produce a composite that has those result objects as its own properties.

Under the old model those two ideas were the same, because a collection of results and a composite of properties were the same object. A query node returning all atoms with a property would build a composite whose members were the result objects. If the results were themselves properties with constraints, the composite then had to satisfy those constraints with the composite as subject. A perfectly valid set of results could therefore fail validation.

### Example

Let `first` and `last` be non-constructed properties declared with the same constraint `%f%`:

```
first is non_constructed_property ·· %f%
last is non_constructed_property ·· %f%
```

Suppose `%f%` rejects a subject that has both `first` and `last`. A query for all atoms with the property `non_constructed_property ·· %f%` yields `first` and `last`.

Under the old model, the query would build the composite `⟨first, last⟩`. That composite has both `first` and `last` as properties, so validating it runs `%f%` with the composite as subject, and the constraint rejects it. The query therefore cannot return an object that is conceptually just the set of its two results.

## Solution

Separate a set's items from the properties of the object that represents the set. Model sets with `contains ·· %item%` properties, where `contains` is a property constructor whose constraint accepts every value for every subject.

The query result is then a composite of `contains` constructions, not a composite of the result objects themselves:

```
⟨contains ·· first, contains ·· last⟩
```

This is the canonical set form, written with sugar as:

```
{first, last}
```

Because `contains` accepts every value for every subject, validating the collection only checks the items themselves as objects; it does not run the constraints of the collected objects with the collection as subject. The query can return the two results without pretending that the result object also has `first` and `last` as properties.

This is the open set model that landed in the public documentation:

- every atom and every composite is a set;
- the items of an atom are the values `%item%` of its statements `%atom% is contains ·· %item%` in the knowledge;
- the items of a composite are its members `contains ·· %item%`;
- a construction has no properties and is therefore not a set;
- `{...}` is sugar for the canonical composite whose members are the corresponding `contains` constructions, and `{}` is the empty composite.

The model is open because a set may also have properties that are not `contains` properties. Those properties are not items; they are ordinary properties and are validated normally. This is what allows an atom to be a set while still carrying knowledge about itself.

## Resolved Design Questions

### Open or closed?

The model is **open**. A set may have arbitrary other properties alongside its `contains` properties. A closed model would require every property of a set to be a `contains` property, which would prevent atoms from being sets, prevent sets from carrying ordinary knowledge, and reintroduce the old conflation through the back door.

The open model keeps the important guarantee: canonical sets are valid whenever their items are, and collecting computed results never runs the constraints of the collected objects.

### Naming

The old structural collection is now called a _composite_. The old term _object set_ refers to a composite. The old _data set_ or _model set_ is now simply a _set_. A composite is still a set in the open model: _composite_ names its object kind, while _set_ names the data view. The terminology no longer distinguishes two kinds of set, because there is only one open set model.

Set-taking nodes operate on sets, which may be atoms or composites. They do not treat arbitrary composite members as items; only `contains` properties are items.

### Notation

The earlier idea of using large angle brackets for object sets was adopted in the form of `⟨...⟩`. The hash-delimited metavariable notation that was proposed at the time was not adopted; metavariables use `%...%`. The `{...}` sugar preserves the old surface syntax for collections, but now it always denotes a composite of `contains` constructions rather than an arbitrary collection.

### Composite operations

Set operations act on items, so they do not directly cover the properties of a composite, which are not `contains` properties. Rather than adding a parallel union, intersection, map, and filter family for composites, the model reuses the set algebra through two conversion nodes: _properties of_ turns an object's properties into set items, and _composite from set_ rebuilds a composite from set items. A property-level operation is then a set operation followed by one conversion.

The conversion node is also the validation boundary. Canonical set results are valid by construction; turning arbitrary set items back into members is the one place that may create a composite whose members do not satisfy its property constraints. Keeping that decision in one node avoids duplicating a future runtime validation rule across every composite operation.

## Implementation Notes

The implementation problems recorded at the time were mostly about keeping structural collections and data collections apart while still sharing set operations. The open set model resolved them as follows:

- Canonical sets in `{...}` form are stored optimally in memory.
- Query nodes and the set operations were rewritten to compute set items and materialise canonical sets. Set operations accept atoms and composites and reject constructions.
- The old idea of duplicate nodes for merging object sets and model sets became unnecessary. There is one set operation family, and the old distinction between object sets and data sets no longer exists.
- An object's properties can be turned into a set explicitly with the properties-of node, which is distinct from treating the object's own `contains` properties as items.

## Effects

- Atoms can be sets. For example, `alices_favourite_integers is contains ·· 10` makes the atom `alices_favourite_integers` a set whose item is `10`, without changing the atom's identity.
- Computed sets are ordinary, valid objects even when the constraints of their items are mutually exclusive when applied to a single subject.
- The two roles are now distinct without losing either concept: composites remain objects with validated properties, and sets are the first-class data model for collections.
