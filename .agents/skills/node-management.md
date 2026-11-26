---
name: Node Management
description: Keep Everything's computation nodes consistent when adding, changing, renaming, or removing a node kind. Covers atoms, recognition, evaluation, deferred sets, capture, tests, docs, and validation.
---

# Node Management

Use this skill whenever a `Node` kind is added, removed, renamed, or has its
semantics changed. A node kind is implemented in several cooperating layers. A
change is only complete when every layer, its tests, and the computation docs
agree.

This skill is part of the codebase. If the procedure has changed — because you
changed it, discovered a missing step, or found that the instructions no longer
match the code — update this skill in the same change. See
[Maintaining This Skill](#maintaining-this-skill).

Read these first:

- `crates/everything/src/compute/nodes/mod.rs` — the `Node` enum and object
  encoding.
- `crates/everything/src/compute/nodes/kind.rs` — `NodeKind`, `NodeProperty`,
  their atoms and display names.
- `crates/everything/src/compute/nodes/raw/mod.rs` — node recognition.
- `crates/everything/src/compute/nodes/raw/diagnose.rs` — recognition error
  explanations.
- `crates/everything/src/compute/evaluate/mod.rs` — the evaluator.
- `crates/everything/src/compute/evaluate/context.rs` — evaluation context and
  the `Thunk` used for call-by-need arguments.
- `crates/everything/src/compute/capture/mod.rs` — closure capture.
- `crates/everything/src/compute/lazy/mod.rs` — deferred set results and `SetValue`.
- `crates/everything/src/model/mod.rs` — `Object` and `AtomOrComposite`.
- `crates/everything/src/knowledge/mod.rs` — property and item queries.
- `docs/public/3 - Computation.md` — the public node specification.

## Mental Model

A node is not a separate type in the database. It is an object shape:

- A composite or atom is a node iff its properties match exactly one node kind.
- Required properties are read with `Knowledge::query_construction_values`.
- Children are themselves nodes; plain data must be wrapped in `node_literal`.
- Parsing is layered: `RawNode` recognises the kind and extracts top-level
  property values, then `Node::new` recursively parses children into a
  structural `Node`, wrapped together with the original source object.
- Not every `Node` variant is a database node kind. `WithSource` preserves the
  parsed object, and `Thunk` shares an unevaluated call argument for
  call-by-need; both are internal and are never produced by `RawNode`.
- `Node::into_object` is the inverse of parsing. It must reproduce the
  canonical object encoding of the kind.
- Evaluation is a demand-driven stack machine. Set-valued results may be
  deferred in `LazySet` instead of being materialised.
- A construction is not a set and has no properties. Consequently,
  `Knowledge` property/item queries take `AtomOrComposite`, and set operations
  live on `SetValue`; no runtime "is a set" check is needed before using them.
  When an operand can be a construction, convert it with
  `LazyObject::into_set()` and treat `None` as `undefined`.

## Fast Path

For a new node kind, touch these in order. Code paths are relative to
`crates/everything/src/`:

1. `model/atoms/mod.rs` — declare new `NODE_*` property atoms.
2. `compute/nodes/kind.rs` — `NodeKind`, `NodeProperty`, atom mapping,
   requirement, repeatability, display names.
3. `compute/nodes/raw/mod.rs` — `RawNode` variant and recognition.
4. `compute/nodes/raw/diagnose.rs` — add the kind's `kind!(...)` spec.
5. `compute/nodes/mod.rs` — `Node` variant, child parsing, `into_object`.
6. `compute/capture/mod.rs` — capture arm.
7. `compute/evaluate/mod.rs` — `Task` variant(s), scheduling, stack effect.
8. `compute/lazy/mod.rs` — only if the node returns a deferred set.
9. `compute/operations.rs` — pure helpers, if the node needs one.
10. Tests: `compute/nodes/tests.rs`, a new
    `compute/evaluate/tests/<name>.rs`, plus `compute/lazy/tests.rs` and
    `compute/capture/tests.rs` when relevant.
11. Docs: `docs/public/3 - Computation.md` and the node sugar table in
    `docs/design/agent-dsl.md`.
12. Run `cargo fmt`, `cargo test`, and `cargo clippy`.

Let the compiler help: `cargo check` after adding the enum variants points at
every non-exhaustive `match` in `Node`, `RawNode`, `NodeKind`, `NodeProperty`,
`Task`, `LazyObject`, `SetValue`, `LazySet`, and `LazySetItems`.

## Step 1 — Specify the Semantics

Before writing code, write down:

- The exact property atoms, e.g. `node_<kind>_<role>`.
- Which properties are required, optional, or repeatable.
- What each child evaluates to, and what happens when a child is not the
  expected object type (`undefined`).
- Whether the result is eager or a deferred set.
- Short-circuiting and evaluation-order rules.
- Interaction with functions, capture, and the evaluation context.

Look at the existing sections in `docs/public/3 - Computation.md` for the
expected shape: definition of the node kind, then evaluation, then
well-formedness/laziness caveats where relevant.

Write a design note under `docs/design/` if the semantics are non-obvious or
contested, and update `docs/design/Q and A/Unresolved Questions.md` if the
change resolves or invalidates an open question.

## Step 2 — Atoms (`crates/everything/src/model/atoms/mod.rs`)

Every node property needs a stable atom id and a documentation name:

```rust
NODE_MY_FIRST = "node_my_first" = 1234...;
NODE_MY_SECOND = "node_my_second" = 5678...;
```

Rules:

- Atom ids are global `u128` identifiers. Generate them with time plus
  randomness (ULIDs are the repository convention). The CLI crate
  (`crates/everything_cli/src/main.rs`) prints sample ids.
- Never reuse, renumber, or "tidy" an existing id. Persisted knowledge stores
  ids, not names.
- `declare_known_atoms!` runs compile-time duplicate checks for ids and names;
  keep them passing.
- `node_*` is a reserved namespace. Do not use it for annotations.
- Name the constant after the property and match the `NodeProperty::Display`
  string in the name literal (e.g. `NODE_MY_FIRST` ↔ `"node_my_first"`). The
  name literal is what `Atom::Debug` prints for known atoms.
- Normally no change is needed in `base/mod.rs`. Node recognition reads
  construction values directly; only touch the base knowledge if the atom is
  meant to be a global property/property constructor.

## Step 3 — Kind Metadata (`crates/everything/src/compute/nodes/kind.rs`)

Add the kind and each property to all relevant places:

- `NodeKind` variant plus its `Display` arm. The display name is diagnostic
  prose, e.g. `"function self-reference"`; it does not have to match the atom
  name.
- `NodeProperty` variant plus:
  - `atom()` → the `NODE_*` constant;
  - `requirement()` → `PropertyRequirement::Value` normally,
    `PropertyRequirement::Depth` for a non-negative `u32` depth;
  - `allows_multiple()` → `true` only for a repeatable property
    (currently `NewSetItem` and `NewCompositeProperty`);
  - `Display` → the exact `node_*` atom name, matching the atom's name literal.

Use names that describe the property role, following existing pairs such as
`AndFirst`/`AndSecond`, `SetMapSet`/`SetMapMapper`, and
`ConstructionValuesPropertyConstructor`. There is no automatic derivation, so
be consistent with neighbouring kinds.

If you add a new `PropertyRequirement` variant, update its `Display` in
`compute/nodes/kind.rs`, the validation in `diagnose.rs`, and any relevant
error text in `compute/nodes/error.rs`.

## Step 4 — Recognition (`crates/everything/src/compute/nodes/raw/mod.rs`)

1. Add the `RawNode` variant holding the raw `Object` child values.
2. Add a recognition block in `RawNode::new` using the existing helpers:
   - single value: `construction_value(object, knowledge, atom)`, which wraps
     `Knowledge::query_construction_values` (itself `None` for constructions);
   - fixed pair/triple: `pair_values` / `triple_values`;
   - repeatable property: `many_values` (returns `None` for zero values);
   - depth property: convert to `u32` as in `node_parameter_depth`.
3. Wrap it in `xor_with!(NodeKind::<Kind>, ...)`.

Recognition invariants:

- A helper must return `None` when the property is absent.
- The order of `xor_with!` calls determines which pair is reported in
  `InvalidNodeError::Ambiguous`. Keep it consistent with the existing order.
- Two kinds must not share all their required properties. Adding a property
  that already belongs to another kind can silently make every object of one
  kind ambiguous.
- Recognition only inspects top-level properties. Child parsing belongs to
  `Node::new`.
- A required property must have exactly one value unless it is repeatable;
  repeatable properties must have at least one value.

## Step 5 — Recognition Diagnostics (`crates/everything/src/compute/nodes/raw/diagnose.rs`)

Add a `kind!(NodeKind::<Kind>, [NodeProperty::..., ...])` entry to `KINDS`:

- List all required properties, including repeatable ones.
- Keep the order of kinds and properties aligned with `RawNode::new`; the
  comment in the file requires this.
- `diagnose` chooses the spec with the most present/satisfied properties, so
  overlapping kinds can change which error is reported. Add or adjust error
  tests if this happens.

## Step 6 — Structural Node (`crates/everything/src/compute/nodes/mod.rs`)

Add a `Node` variant:

- Write a doc comment that specifies evaluation, including at least one
  `undefined` case.
- In `Node::new`, map the `RawNode` variant to the `Node` variant and parse
  every child with `child!(NodeProperty::..., &value)`. Do not parse opaque
  data such as literal payloads or depth integers.
- In `Node::into_object`, encode the node as a composite of constructions using the
  same property atoms. Use `Composite::from([...]).into()` for a single value,
  follow the pair/triple patterns for multiple values, and handle variadic
  properties explicitly.
- `WithSource` preserves the object a node was parsed from, so `into_object`
  reproduces annotations and unknown properties. The structural encoding is
  still the canonical form produced by construction and must round-trip.
- Do not add a wildcard match arm; exhaustiveness is the safety net for later
  changes.

## Step 7 — Capture (`crates/everything/src/compute/capture/mod.rs`)

Add a `Node::<Kind>` arm that recursively captures every child:

- Use the same `additional_depth` for ordinary children.
- Use `additional_depth + 1` only for a function body.
- Capture all children of variadic nodes.
- Keep the returned `Node` structurally identical apart from captured
  parameter/self references.

Parameter references are substituted by the bound value or by the argument
thunk. An unevaluated argument is shared as `Node::Thunk`, not copied; a thunk
whose value is already cached becomes a `Node::Literal`. This is what makes
call-by-need work, so do not rebuild an argument expression when capturing a
parameter reference.

If a child is not captured, closures that escape a call keep stale
references. Test this when the node can contain functions.

## Step 8 — Evaluation (`crates/everything/src/compute/evaluate/mod.rs`)

The evaluator is an explicit stack machine. Two places matter:

### `resolve_evaluate_node`

- Add a `Node::<Kind>` arm.
- Push the task that consumes the children first, then push child evaluations.
  The task stack is LIFO, so push children in reverse operand order to have
  them pop in source order. `NewSet` shows the variadic pattern: push the
  `NewSet { count }` task, then iterate `items.rev()` pushing `Task::Evaluate`.
- For short-circuiting nodes, push a partial task such as `PartialAnd`,
  `PartialOr`, `PartialIf`, or `PartialSetDifference`, and schedule the second
  child only after the first result is known.

### `evaluate`

- Add the matching `Task` variant and implement its stack effect.
- Pop in the reverse order of the pushes; the first scheduled child is the
  last popped. Pair tasks follow `let right = pop(); let left = pop();`.
- Push exactly one result for every consumed child, or return `UNDEFINED`
  without leaving stale values on the stack.
- For set-taking nodes, convert each operand once with `LazyObject::into_set()`
  and match on the resulting `Option<SetValue>`. `None` means the operand is a
  construction, so the node evaluates to `UNDEFINED`. Do not guard with
  `is_set()` and do not call set operations on `LazyObject`.
- Store set operands in `LazySet` fields as `Box<SetValue>`, never as
  `Box<LazyObject>`.
- Use `LazyObject::Eager(...)` for materialised values. Build deferred set
  descriptions only for set-valued lazy results.
- Use `operations::*` for reusable pure semantics instead of duplicating
  logic.
- The final checks at the end of `evaluate` require the stack and evaluation
  context to be empty apart from the result.

## Step 9 — Deferred Sets (`crates/everything/src/compute/lazy/mod.rs`)

Only needed when the node evaluates to a set that should not be materialised.
The lazy layer uses two types:

- `LazyObject` is the evaluator's result: `Eager(Object)`, `Node(Node)`, or
  `Set(LazySet)`. An eager object may be a construction.
- `SetValue` is a value that is a set: `Eager(AtomOrComposite)` or
  `Set(LazySet)`. Property and item operations live here and are total.

Additions:

- If the node returns a new kind of deferred set, add a `LazySet` variant.
- `SetValue::item_count`, `has_at_least_one_item`, `has_item`, and
  `has_property` → implement the new variant. These methods cannot fail,
  because a construction is not a `SetValue`.
- `SetValue::into_items` → add a `LazySetItems` variant, or wrap an existing
  iterator in `Deduplicated` when the operation may yield duplicates.
- `LazySetItems::next` → implement iteration.
- `SetValue::into_properties` → only if the result is consumed as properties
  rather than items.
- `LazyObject::into_set` already converts eager atoms/composites and deferred
  sets, and returns `None` only for a construction. Keep it total: never add a
  panic or `expect` for a construction.

Store set operands inside `LazySet` and `LazySetItems` as `Box<SetValue>`, so a
construction cannot reach the set algebra. A function value
(`LazyObject::Node`) is a set too — its object encoding is an atom or composite
— and is materialised by `into_set`; `LazyObject::is_set` reports it as one. Do
not break that convention.

## Step 10 — Tests

Repository rule from `AGENTS.md`: no inline `mod tests { ... }` in the file
under test. Put tests in a sibling `tests.rs` (or a `tests/` directory where
the module already uses one).

### Node parsing — `crates/everything/src/compute/nodes/tests.rs`

- Add the new variant to `node_round_trip` so
  `into_object → Node::new → into_object` is identical.
- Add display assertions for the new `NodeKind` and `NodeProperty` if the
  names are user-visible in errors.
- If you add a new property requirement:
  - missing property test → `InvalidNodeError::MissingProperty`;
  - duplicate property test → `DuplicateProperty` (skip for repeatable
    properties);
  - invalid depth/value test → `InvalidProperty`;
  - ambiguity test if the kind can overlap another kind.
- Keep `node_round_trip_preserves_extra_properties` working; add a case for
  the new kind if it has unusual optional properties.

### Evaluation — `crates/everything/src/compute/evaluate/tests/`

- Create `tests/<kind_snake_case>.rs` and register it alphabetically in
  `tests/mod.rs`.
- Use `crate::base::BASE` and the helpers in `tests/common.rs`
  (`integer`, `integer_set`, `set_of`, `composite_of`, `identity_function`,
  predicate helpers, etc.).
- `evaluate` returns `LazyObject`. Use the `set(&result)` helper from
  `tests/common.rs` to assert set semantics through `SetValue`; it panics in
  the test if the result is a construction.
- Cover:
  - the normal result, including membership, item count, and iteration order;
  - each operand type that must produce `Atom::UNDEFINED`;
  - deferred results: `item_count` counts distinct values, iteration does not
    duplicate, `has_item` agrees with iteration;
  - short-circuit behaviour: use a second child that would produce
    `undefined` or diverge and assert it was not evaluated;
  - nested deferred operands and materialisation boundaries;
  - evaluation context: parameters, self-references, and escaping closures;
  - `undefined` children as ordinary values where the semantics say so.
- Add capture tests in `compute/capture/tests.rs` when the new node can
  contain a function that escapes a call.

### Lazy operations — `crates/everything/src/compute/lazy/tests.rs`

Add tests for the new `LazySet` variant, constructing operands as `SetValue`
(the `eager_set` helper returns one) and covering:

- `item_count` deduplicates;
- `into_items` / `iter` semantics;
- `has_item` / `has_at_least_one_item`;
- consistency between item count, membership, and iteration;
- snapshot behaviour if the operation reads the knowledge.

Add a `LazyObject::into_set` test when the node can produce a construction:
the conversion must return `None` rather than panic.

## Step 11 — Documentation

### `docs/public/3 - Computation.md`

This is the specification, not a changelog. Update it as part of the same
change:

- Add a section for the node kind, with the “An object is a … node iff …”
  formulation and the exact property atoms.
- Specify evaluation, including every `undefined` case and short-circuit.
- Add an example under “Examples” when it clarifies the encoding.
- Update “Node Objects and Recognition” if the recognition rules change
  (for example, a new repeatable property changes the sentence that lists the
  repeatable properties).
- Update “Evaluation Strategy” if the node is deferred, short-circuits, or
  materialises values.
- Update the “Implementation Status” table with the encoding and any
  limitations.
- Update “Current Limitations” when behaviour is intentionally partial.
- Check anchor links and cross-references.

### `docs/design/agent-dsl.md`

The “Node sugar table” must stay one-to-one with the `Node` enum. Add a row
with the surface syntax and a short name, and follow the additive-only
evolution rule: new nodes are new keywords, existing spellings never change
meaning.

### Other docs

- `docs/design/` — add or update a design note when semantics are contested.
- `docs/spec/File Format v1.md` — change only if the node introduces a
  format-level reserved representation; ordinary node atoms do not.
- `docs/public/1 - Objects.md` and `docs/public/2 - Knowledge.md` — update
  only if the node changes the object or knowledge model, not merely its
  computation.

## Renaming and Removing Nodes

- **Rename a node kind or property**: change the Rust names and display
  strings, but keep the atom id stable. `NodeProperty::Display` is the
  user-facing `node_*` name; diagnostics and docs must use the new name.
- **Remove a node kind**: remove every variant and match arm across `Node`,
  `RawNode`, `NodeKind`, `NodeProperty`, `Task`, `LazyObject`, `SetValue`,
  `LazySet`, and `LazySetItems`; remove its tests, docs section, status table
  row, and agent-DSL row.
- **Do not delete atom ids.** Old objects may still contain them. Reusing an
  id would silently change the meaning of persisted knowledge.
- **Check for reinterpretation**: after removing a kind, objects that used it
  may match another kind or become invalid. Decide and document which is
  intended, and add a test.
- **Semantic changes** to an existing kind require the same test and doc
  updates as a new kind. Treat the public computation chapter as the
  contract.

## Maintaining This Skill

Treat this file like code: it must stay true to the repository.

Update it in the same change whenever any of the following happens:

- You add, remove, rename, or reorder a step because the implementation
  changed.
- You discover a required file, function, invariant, or test that this skill
  does not mention, or that it describes incorrectly.
- A validation command, test layout, documentation location, or repository
  convention changes.
- You find a stale path, symbol, anchor, or cross-reference.
- You change the scope of the skill (for example, it starts covering node
  removal as well as addition), in which case also update the frontmatter
  `description`.

When updating:

1. Keep the Fast Path, the detailed steps, the Review Checklist, and the
   frontmatter description consistent with each other.
2. Prefer editing the existing step over appending a warning at the end; the
   skill should read as the current process, not a history of it.
3. If a step moved to a new file or symbol, update every reference, including
   the “Read these first” list.
4. If you only followed the process and it was correct, no change is needed.
   Do not rewrite this skill gratuitously.

The same rule applies to contributors: if this skill told you to do something
that turned out to be wrong or incomplete, fix the skill as part of your pull
request, not in a follow-up.

## Validation

Run from the repository root:

```sh
cargo fmt --all --check
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```

The library has `#![deny(clippy::pedantic)]`, so new code must be clippy-clean;
prefer adding narrow `#[allow(...)]` attributes with a reason only where the
surrounding code already does so.

For a quick loop while iterating:

```sh
cargo check
cargo test -p everything --lib
```

## Review Checklist

- [ ] New atoms have unique ids, unique name literals, and stable values.
- [ ] `NodeKind`, `NodeProperty`, `RawNode`, `Node`, `Task`, `LazyObject`,
      `SetValue`, `LazySet`, and `LazySetItems` are all updated
      (compiler-checked).
- [ ] `diagnose.rs` mirrors `RawNode::new`.
- [ ] `into_object` reproduces the canonical encoding; round-trip test passes.
- [ ] Every child is parsed, captured, and evaluated in the right order.
- [ ] Set-taking tasks convert operands with `LazyObject::into_set()` and map
      `None` to `UNDEFINED`; set algebra stores `SetValue`, not `LazyObject`.
- [ ] Deferred sets obey set semantics for item count, iteration, and
      membership.
- [ ] Tests cover success, `undefined`, short-circuit, laziness, and capture.
- [ ] `docs/public/3 - Computation.md` has the section, status row, and
      lazy/limitation notes.
- [ ] The agent-DSL sugar table has a row for the new kind.
- [ ] This skill was updated if any procedure, file, command, convention, or
      scope changed while doing the work.
- [ ] `cargo fmt`, `cargo test`, and `cargo clippy` pass.
