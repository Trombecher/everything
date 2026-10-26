# Node Literals and Required Child Nodes

**Status:** design review. The open-definition question is decided (nodes are open), but whether child
expressions must be nodes is still a proposal, not an implementation.

**Related:** [`Unresolved Questions.md`](./Q%20and%20A/Unresolved%20Questions.md),
[`Open versus closed definitions.md`](./Q%20and%20A/Open%20versus%20closed%20definitions.md),
[`3 - Computation.md`](../public/3%20-%20Computation.md),
`crates/everything_2/src/compute/`.

## The question

Should every non-leaf node require its child expressions to be nodes? If yes, then every literal
value used in a program tree has to be wrapped in a `node_literal` node.

The current model does not require this. A node's children are plain `Object`s. During evaluation a
child is interpreted as code iff `Node::new` recognises it in the current knowledge; otherwise it is
treated as an implicit literal (`crates/everything_2/src/compute/evaluate/mod.rs`, the `None` arm of
`Task::Evaluate`).

## Decision context: node definitions are open

Node definitions are intended to be **open**: a node may carry additional, unrelated properties such
as comments, annotations, links, or documentation without ceasing to be a node.

Two reasons were given:

1. Users must be able to attach their own metadata to a node without breaking the computation
   system.
2. Programs are Turing complete, so two node trees that compute the same function cannot in general
   be decided to be equal. Structural equality of programs is therefore not semantic equality. The
   main argument for closed definitions — preserving "structural equality iff semantic equality", as
   with natural numbers — does not apply to programs at all.

This is a decision about **annotations**, not about user-defined node *kinds*. The set of node kinds
is still the fixed set understood by the interpreter.

Consequences that matter for this review:

- Recognition is shallow and must ignore properties it does not know. It already does:
  `query_construction_values` only scans the node's own top-level properties (an atom's statements or
  a set's members).
- Extra properties are **opaque**. They are not code positions, so evaluation, capture, and
  validation must not descend into them.
- Because recognition uses reserved `node_*` property constructors, annotations must not reuse those
  constructors. Otherwise an annotation can change — or make ambiguous — the node kind. This needs to
  be documented as a reserved namespace.
- Since there is no canonical program equality to protect, there is no reason to force programs into
  one canonical structural form. Any canonicalisation we add is a syntactic convenience, not a
  semantic normal form.

## Assessment of the proposal

### The invariant is the right shape internally

Making children nodes gives the interpreter a real program tree instead of an object it repeatedly
re-interprets:

- **Capture gets a well-defined boundary.** Capture has to substitute free parameters in code while
  leaving data untouched. `node_literal` is exactly that boundary. Today
  `crates/everything_2/src/compute/capture.rs` only handles `Function` and `Parameter` and falls into
  `todo!` for everything else, so this is the natural moment to settle the traversal rule.
- **Evaluation and capture become structural matches.** `Node::And { first: Object, second: Object }`
  can become `And { first: Box<Program>, second: Box<Program> }`. Parsing happens once, the evaluator
  does not re-parse every child, and an exhaustive match makes a forgotten node kind a compile error
  instead of a runtime `todo!`.
- **The code/value boundary is explicit.** `literal` already behaves like a quote operator, and
  capture already wraps runtime parameter values in `Node::Literal` (`capture.rs`) precisely because a
  value spliced into code must not be reinterpreted as code. The proposal generalises that existing
  rule to the whole program tree.
- **The specification gets simpler.** "A program is a node tree; `literal` is a leaf" is easier to
  state and reason about than "if the object happens to match exactly one node shape, interpret it,
  otherwise treat it as an implicit literal."

### But malformed nodes must not silently become data

The dangerous way to implement strict children is to make `Node::new` return `None` when a child is
not a node. Because the object is then not a node at all, it evaluates to itself. A forgotten
`node_literal` would silently turn the whole expression into opaque data — the opposite of robust.
For example, `{node_add_first 1, node_add_second 2}` would stop being an add node instead of
reporting an error.

So strict children require three separate notions:

1. **node-shaped** — the required property constructors for one node kind are present;
2. **well-formed program** — every code-position child parses as a node;
3. **opaque data** — neither of the above.

Parsing should return `Result<Program, ParseError>`, not `Option<Node>`. Evaluation of a node-shaped
but ill-formed object should produce `undefined` (or a dedicated error atom), never fall through to
data. The current `xor_with!` logic in `Node::new` deliberately conflates "not a node" with "matches
more than one node kind"; that is fine for recognition of data, but it is not sufficient for
validating programs.

### Mandatory wrapping in the object form is expensive

If every literal in serialised knowledge must be wrapped, the cost is not one wrapper per literal
object in the AST; it is one wrapper per leaf in every program, including knowledge constraints.

Atoms can be nodes via knowledge (`foo is node_function ...`), so atoms cannot be assumed to be
leaves. Every ordinary atom, integer, set, or `{{}}` used as a child needs an explicit literal.
Common knowledge such as

```
person is property {{}}
child_of is property_constructor subject |-> value |-> (subject is person and value is person)
```

turns into deeply nested `node_literal` wrappers. This is a significant ergonomic regression for
hand-written knowledge, and it silently breaks existing objects unless the format is versioned.

There is also a semantic question. Everything's meaning is knowledge-relative. An object that is data
in knowledge `K` may be a node in a merged knowledge `K'`. Requiring `node_literal` everywhere bakes
the authoring-time knowledge into the program and prevents it from ever being reinterpreted. Whether
that freezing is desired is a real semantic decision, not just a syntax choice. Explicit
`node_literal` is useful as an intentional escape hatch; making it mandatory makes every value
frozen by default.

### Open nodes rule out a lossy AST

This is the strongest additional constraint from the openness decision. A `Program`/`Expr` tree that
stores only the known node fields would drop annotations and unknown properties as soon as the tree
is converted back to an object. That matters because:

- capture reconstructs function objects, so reconstructing a node must not discard its metadata;
- a user must be able to add a comment to a function body and still get that function back after
  capture;
- canonicalisation, if it exists, must be an additive rewrite, not a re-serialisation of a closed
  tree.

The internal representation should therefore be **lossless with respect to the original object**: keep
the originating object (or its unknown properties) together with the parsed known children. Only the
known child positions are traversed as code; everything else is preserved verbatim.

A workable shape is:

```rust
struct Program {
    /// The original object, including annotations and unknown properties.
    origin: Object,
    kind: ProgramKind,
}

enum ProgramKind {
    Literal(Object), // leaf: payload is data and is never traversed
    Function { body: Box<Program> },
    Parameter { depth: u32 },    // leaf
    FunctionSelf { depth: u32 }, // leaf
    Add {
        first: Box<Program>,
        second: Box<Program>,
    },
    // ...
}
```

Converting back to an object copies `origin` and replaces only the recognised child property values.
`Literal` is the only place where arbitrary data is allowed inside the tree.

### Atom-defined nodes are the hard case

For a set node, "copy the origin and patch the known children" works. For an **atom** node, the node
properties live in knowledge, not in the object, so they cannot be patched into a copy. Capture
already normalises an atom function into a set function
(`Object::new_node(Node::Function { ... })` in `capture.rs`), which loses the atom's identity and any
knowledge-level annotations.

This is an existing lossy behaviour, but open nodes make it more visible. A decision is needed:

- accept that capturing an atom-defined function normalises it into a set and loses its atom-level
  metadata; or
- give closures a representation that preserves the original function object, at the cost of a more
  complicated capture/closure design.

## Recommendation

1. **Adopt the invariant internally.** Introduce a typed, lossless program representation whose
   non-leaf nodes always have `Program` children and whose `Literal` variant is a leaf. This gives the
   evaluator, capture, and error reporting the benefits above without changing the meaning of
   existing knowledge.

2. **Keep the object form lenient initially.** When parsing an object into a program, a non-node child
   may still become a `Literal` implicitly. An explicit `node_literal` remains available when the
   author wants to protect a node-shaped value from being interpreted as code. This preserves
   ergonomics and keeps meaning knowledge-relative.

3. **Separate recognition from validation.** `Node::new` may keep its current shallow, "exactly one
   node kind" recognition. Add a separate `parse`/`validate` returning `Result<Program,
   ParseError>`. A node-shaped object with an ill-formed child should evaluate to `undefined` or
   report an error, not silently degrade to data.

4. **Make the typed representation lossless.** Preserve unknown properties and annotations when
   capturing or rewriting nodes. Traverse only the known code positions; never traverse `Literal`
   payloads or annotations. Document that `node_*` constructors are reserved and cannot be used for
   annotations.

5. **If a strict object form is wanted later, make it a canonicalisation pass, not a recognition
   rule.** Such a pass would insert explicit `node_literal` wrappers at code leaves while preserving
   every other property. It should be optional and it should not be presented as a semantic normal
   form; Turing-complete programs have no such normal form.

6. **Decide the atom-function/closure question.** Whether an atom-defined function keeps its identity
   across capture is orthogonal to literals, but the lossless-program design will expose it.

## Summary

The proposed invariant is sound, and it is compatible with open node definitions: extra properties are
not code positions and can be ignored by the interpreter while surviving transformations. The
valuable part of the proposal is the typed internal tree with `node_literal` as an explicit leaf; the
object-level requirement to spell out every literal is a separate, much costlier choice.

The main risk to avoid is implementing strict children as "not a node", which converts authoring
mistakes into silent data. Strictness belongs in a validating parser returning errors, not in the
recogniser returning `None`.
