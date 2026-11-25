# Set Models

> **Status: resolved.** This solution has landed in the public documentation: the common set model (sets as composites of `contains ·· %item%` constructions with `{...}` sugar) is defined in `docs/public/4 - Common Models.md`, the computation nodes operate on it (`docs/public/3 - Computation.md`), and it resolves the validity question in `docs/public/2 - Knowledge.md`. This document predates the notation change: composites are now written `⟨...⟩`, constructions `c ·· v`, and data sets keep `{...}` as sugar.

(This problem assumes that of sets to be valid objects, all elements must be valid properties of that set, i.e. their constraint must be run.)

## Problem Description

Currently, inputs and outputs of many nodes operate on set objects (see union, intersection, ...). For example, the output of a query-atoms-with-property node returns a set object of all atoms in the knowledge that have the given property. Conceptually, that object should represent a _set_ of all result objects of that query, and should NOT represent the set object that has all that properties.

### Example

For example, if `first` and `last` are properties (stated via `property : <f>`, the same `<f>`) in the knowledge, a query for all atoms that have the property `property : <f>` would yield both `first` and `last`. But if their constraint `<f>` would check that the subject has mutual exclusion on `first` and `last` (not both at the same time), then the output set creation would **fail** because it would try to construct `{first, last}`.

## Solution

We could model sets in Everything by `contains : <value>` properties, with `contains` being a property constructor with a constraint that accepts any value.

This way, the output of the query node would be `{contains : first, contains : last}`.

## Additional Notes

Currently we do not distinguish between sets as data and sets as objects, the building blocks of the system.

## Additional Design Problems

- Whether that constraint function should accept any object (i.e. you can mix: `{some_property, contains : 10}`; to be open) or check if all properties of the subject are actually of the form `contains : <value>` (the set model is closed) is a different important question for the implementation of this solution.
- Should be rename the set object to something like "intrinsic" or "composed"?
- Should we use different symbols to denote object sets (current denoted with `{...}`), something like `<...>` (currently used for metavariables, but can be changed to something like `#...#`). Big angle brackets are also an option, mirroring LEAN4 structure creation style.

## Implementation Problems

- We would need to provide an optimized model-set representation in the UI.
- Rewrite query nodes and everything.
- Provide duplicate nodes for merging object sets and model sets.

## Effects

- This would effectively allow for sets to be atoms, like `{alices_favourite_integers is contains : 10}`.
