# Open Node Definitions

**Status: resolved.** Node definitions are _open_. An object is a node when its required properties match exactly one node kind; additional properties do not disqualify it. This document records the alternatives, the arguments on both sides, and why the current implementation is open.

The rule is specified in [`docs/public/3 - Computation.md`](../public/3%20-%20Computation.md#node-objects-and-recognition). This document is the design record for that rule.

## The Question

A node kind has a set of required properties. For example, an add node requires one `node_add_first` property and one `node_add_second` property. The question is what to do with properties that are not required by the kind:

- a **closed** definition requires every property of the object to belong to the recognized kind, so any additional property makes the object not a node;
- an **open** definition lets the required properties determine the kind while additional properties are ignored as far as recognition is concerned.

A node object is an atom or a composite. Its properties come from the knowledge for an atom, or from its members for a composite. A bare construction can never be a node.

## The Rule In The Current Model

An object is a valid node iff:

- it matches exactly one node kind;
- every required property has exactly one matching construction, except for the repeatable properties `node_new_set_item` and `node_new_composite_property`, which may occur any positive number of times;
- every child position itself parses as a node; and
- any other property is ignored, provided it does not use the reserved `node_*` namespace.

The additional properties are still part of the object. Parsing a node retains its source object, so converting the node back to an object reproduces the annotations exactly. Capture likewise preserves annotations and unknown properties on function bodies and nested nodes.

For example, this is an add node:

```
⟨
    node_add_first ·· ⟨node_literal ·· 1⟩,
    node_add_second ·· ⟨node_literal ·· 2⟩
⟩
```

This is the same add node with metadata attached:

```
⟨
    node_add_first ·· ⟨node_literal ·· 1⟩,
    node_add_second ·· ⟨node_literal ·· 2⟩,
    summary ·· "adds one and two",
    source ·· ⟨file ·· "main.everything", line ·· 4⟩,
    reviewed ·· true
⟩
```

Both match the add kind and evaluate to `3`. They are different objects, because the annotations are part of their structure, but they are the same program. Under a closed definition the second object would not be a node at all.

The same applies to atom-defined nodes. An atom can be a function and also carry knowledge:

```
sum is node_function ·· %body%
sum is summary ·· "adds its two arguments"
sum is author ·· Alice
```

Under an open definition `sum` remains a function. Under a closed definition every additional statement would make it stop being one.

## Arguments For Closed Definitions

### Canonical Structural Representations

A closed definition guarantees that each node kind has one structural shape. If an object has the add properties plus a `summary`, it is rejected instead of being accepted as an annotated add node. This gives a single canonical form per kind and makes structural equality of node objects align with equality of syntactic programs.

That is useful for content addressing, deduplication, hashing, and caching. Two textual spellings that produce the same node structure denote the same object; annotations cannot produce a second representation of the same kind.

### Early Detection Of Mistakes

A closed definition rejects unexpected properties. A misspelled `node_add_frist`, a stale property left behind by a refactoring, or a property intended for a different node kind becomes an error instead of silently being ignored. This can make malformed programs easier to catch at the point where they are written.

### No Reserved Namespace Required

Closedness eliminates the question of which extra properties are harmless. There is no need to reserve `node_*` or to explain that some additional properties are inert while others can change the recognized kind. The kind is exactly the full property set.

### Stronger Code Identity

If a system treats node objects as code identities, closedness makes that identity strict: any change to the object changes the code, and metadata cannot be attached without changing the object. For signed code, reproducible builds, or precise audit trails, that can be attractive.

The last point is also a weakness. Annotating code necessarily creates a different object in either model. Closedness does not preserve identity under annotation; it only refuses the annotation.

## Arguments For Open Definitions

### Code Can Carry Metadata

Nodes represent programs, and programs are objects in Everything. Users want to attach a summary, documentation, a source location, an author, review status, comments, or tooling results to code. An open definition lets that metadata live in the node object itself, next to the code it describes.

For a composite node this is natural: the annotation is simply another member. For an atom-defined function it is equally natural: the annotation is another statement about the atom. A closed definition would force metadata into a separate wrapper object, a side table, or a dedicated atom for every annotated expression. That loses locality and makes the stored code diverge from the expression the user wrote.

### Extensibility Without Changing Node Kinds

Open definitions let consumers add annotations without changing the language or the node kind's schema. A new tool can introduce `profiled_at` or `generated_from` properties, and older recognizers simply ignore them. Closed definitions would require every annotation property to be declared as part of every kind, and adding one would change what counts as valid code everywhere.

The alternative is to make annotation part of the language through a dedicated wrapper. That adds a level of nesting to the object model and forces every consumer to unwrap it before reaching the node. Openness keeps the annotation transparent to recognition.

### Semantic Program Equality Is Not Computable

The strongest argument for closed canonical forms is that they give one representative per program. But nodes are programs in a Turing-complete system, so semantic equivalence of two nodes is undecidable. No algorithm can decide whether two arbitrary programs compute the same function, and no canonicalizer can produce a unique representative for a program's meaning.

A closed definition can only canonicalize syntax. It can ensure that every object of a kind has exactly the required properties, but it cannot ensure that two syntactically different add nodes are the same node. The benefit of canonical forms is therefore much smaller than it first appears: it gives syntactic uniqueness, not semantic uniqueness. Restricting node objects to canonical shapes does not buy semantic comparison, and it does buy real costs.

### Structural Equality Remains Honest

Everything's promise is that a well-founded model makes structural equality and semantic equality the same relation. For programs, that promise cannot mean that every pair of semantically equivalent programs has the same object, because semantic equivalence is undecidable. It can only mean that equal objects denote equal programs.

Open definitions preserve that direction. An annotated add node and a bare add node are different objects; they are also different pieces of knowledge, because the annotation is real information. They happen to evaluate to the same value, which is a computed fact, not a failure of structural equality. The model does not need to pretend that all presentations of a program are one object.

### Consistency With The Open Object Model

Everything already has an open set model: a set may carry arbitrary non-`contains` properties, and those properties are ordinary knowledge rather than items. Atoms accumulate properties over time by design. Node definitions follow the same principle. A node kind identifies the computation, while the rest of the object remains available for knowledge about that computation.

Making nodes closed would give them a different, stricter notion of objecthood than the rest of the model. It would also make atom-defined nodes fragile, because any new statement about the atom could invalidate the node. Openness keeps node recognition stable as knowledge grows.

### Metadata Survives Evaluation

The implementation preserves annotations through parsing, function evaluation, capture, and round-tripping. A function object keeps all non-body properties, and nested nodes keep their unknown properties when a closure is captured. This means metadata attached to source code remains attached to the code that actually runs and to function values that escape their defining call. A closed model would either lose that metadata or require a parallel representation.

## The Cost Of Openness

Openness is not the same as accepting arbitrary objects. Recognition still enforces:

- exactly one node kind, never a combination of kinds;
- the exact number of required properties, except for the documented repeatable ones;
- recursively valid children in every child position; and
- the reserved `node_*` namespace, which prevents annotations from accidentally changing the recognized kind.

The remaining cost is that an unintended non-`node_*` property is ignored rather than rejected. This is a deliberate trade: the model treats unknown properties as knowledge, and validation of knowledge is separate from node recognition. A tool that wants strictness can still validate the properties it cares about or require a canonical encoding when it constructs nodes. The core language does not impose canonical syntax on every user.

## Verdict

Nodes are modelled as **open definitions**. The required properties identify the node kind; additional properties are metadata and do not affect recognition. This is consistent with the current implementation in `RawNode::new`, `Node::new`, capture, and `Node::into_object`.

The decisive arguments are:

- programs need to carry metadata, and nodes are objects in a knowledge base;
- semantic program equivalence is undecidable, so canonical forms only provide syntactic uniqueness, not the semantic uniqueness that would justify their cost;
- the open object and set models already allow objects to carry unrelated properties, and nodes should not be a special case;
- open definitions let the language and its tooling grow without redefining every node kind.

Closed definitions would provide a canonical property set and catch some mistakes, but they would make annotation impractical, make atom-defined nodes fragile under accumulating knowledge, and promise a semantic canonicality that a Turing-complete system cannot deliver.
