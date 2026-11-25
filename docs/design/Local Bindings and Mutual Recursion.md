# Local Bindings and Mutual Recursion

This is a design note about local aliases (`let`-like blocks) in the computation
system and in the visual editor. It compares adding a new node kind against
treating the construct as sugar over the existing function-application encoding,
and it discusses mutually recursive local bindings separately, because they are
a genuinely different problem.

The short version:

- **Non-recursive local aliases** (for example `cubed = x * x * x`) should
  **not** become a node kind. They should be a surface form / UI projection over
  the already-existing `Call(Function(...), value)` encoding.
- **Mutually recursive local functions** are a separate feature. Do **not** try
  to recover them by pattern-matching the frontend, and do **not** assume that
  "collapsing until a self-reference appears" works in general. Either leave
  them as a user-level encoding, or add a dedicated, function-only `letrec`
  node. A generic `let` node does not solve them.

The important distinction is:

> A `let` that is semantically just a call is **notation**, not a new
> computation primitive.
>
> A `letrec` that binds mutually recursive functions is a **new binder**, and
> binders are the expensive part of this model.

## 1. What the current system already gives you

The only binder in the computation model is the function:

- `node_function : <body>`,
- `node_parameter : <depth>` for the argument,
- `node_function_self : <depth>` for the function itself,
- `node_call_callee` / `node_call_with` for application.

Calls are non-strict: the argument is closed over the caller's context with
`capture_argument` and evaluated only when the parameter is referenced
(`Argument::Unevaluated` in `compute/evaluate/context.rs`). The result is cached
in the argument thunk and shared by later references. That is already
call-by-need `let`:

```text
(λcubed. cubed + 1 / cubed) (x * x * x)
```

is exactly

```text
let cubed = x * x * x in cubed + 1 / cubed
```

with the same laziness and the same capture behaviour. Multiple sequential
bindings are just nested calls:

```text
let a = e1; b = e2 in body
≡ (λa. (λb. body) e2) e1
```

So the "inline function plus immediate application" option is not merely more
aligned with the model; it **is** the model's binding construct already.

There is an existing piece of UI intent to leverage: the explorer already stores
parameter names as the `parameter_name` annotation on function nodes. Binding
names therefore do not need a new semantic representation. They are
annotations, exactly like the current function parameter names, and the
evaluator already ignores them.

## 2. Why a new `let` node is not "just another redundant node"

The system already contains nodes that are extensionally redundant, for example
`Every`/`Any` versus `Fold`. But those have **distinct operational semantics**:
they short-circuit and have their own evaluation rules. The computation
documentation even flags them as candidates for removal.

A plain `let` node that lowers to a call has:

- the same reference-evaluation result,
- the same laziness,
- the same capture semantics,
- the same `undefined` cases,
- the same cost model (unless sharing is deliberately changed).

It adds only a second **structural encoding** of the same program. Because
Everything has no semantic normalization, that is a real cost:

- `let x = e in b` and `(λx. b) e` become two different objects.
- Structural equality, queries, deduplication, aliases, and agent-written
  corpora split into two representations.
- The agent DSL's principle of "sugar tier is pure desugaring; the printer
  re-recognizes the canonical pattern" is exactly designed to avoid this.
- Every layer listed in the node-management skill gets a new case for a
  construct that does not change the meaning of anything.

So the criterion should not be "is it redundant?" but:

> Does the node add a **distinct meaning**, or only a **distinct spelling**?

For non-recursive `let`, it is only a spelling. For `letrec`, it is a distinct
binder.

## 3. Concrete technical gotchas if you do add a `let` node

These are the parts that are easy to underestimate.

### 3.1 A source-preserving desugared node does not compose with capture

One might be tempted to do this:

- `Node::new` recognizes the `let` object,
- builds the structural `Call(Function(...), value)` node,
- wraps it in `Node::WithSource` so `into_object` still returns the `let`
  object.

That is **not safe** with the current capture implementation.

`capture_with_mode` on `WithSource` recurses into the *structural* node and then
calls `source_with_captured_properties(source, original, captured, ...)`. That
helper derives the property constructors from `original.into_object()`. If
`original` is the structural `Call`, it looks for `node_call_callee` /
`node_call_with` in the source object, which contains `node_let_*` properties
instead. The rebuilt source becomes a mixture of the two encodings.

So either:

- `let` is **not a node** (pure sugar), or
- it is a **real `Node::Let`** with its own parse, capture, evaluate, and
  `into_object` arms.

There is no free hybrid.

### 3.2 A real `let` node needs a reference story

If `Node::Let` is a real node, how does the body refer to the bound value?

Two options:

1. **Reuse `node_parameter` and treat the let as an implicit function.**
   Evaluation can push a `FunctionContext` with a synthetic
   `Function { body }`, exactly like a call. This is simple but leaks the
   implicit function: `node_function_self : 0` inside the let body would refer to
   that synthetic function. That must either be documented or rejected. The
   object is also still structurally different from the call encoding.

2. **Add a distinct local-reference node** (`node_local : <depth>` or similar).
   This is cleaner conceptually but adds a *third* de Bruijn space next to
   parameters and function self-references. Capture, evaluation context, docs,
   tests, and the UI all need to understand it. This is a significant complexity
   jump for a construct that is otherwise a call.

Neither option is free, and both are only worth it if the node carries a
genuinely different semantics.

### 3.3 N-ary bindings are awkward in an unordered set model

Objects are sets; repeatable properties are unordered. A "block with N
bindings" needs a stable order and stable identity for each binding. Options:

- **Unary let nodes nested:** order is encoded by nesting; natural, but a
  multi-binding UI block must group the chain anyway.
- **N-ary let node:** needs an ordered binding representation (list/binding
  nodes) or name-based references. Name-based references break alpha-equivalence;
  ordered lists need more node kinds or a list encoding.

The UI grouping problem does not disappear just because a node is added; it
moves into the encoding.

### 3.4 Self-recursion is a real scoping trap

Suppose you write:

```text
let f = λx. ... f(x - 1) ... in ...
```

and lower it naively to:

```text
(λf. body) (λx. ... f(x - 1) ...)
```

In the binding value, a reference to `f` is **free relative to that value**. At
the call site, `capture_argument` closes the argument in the *caller's* context,
before the let binder exists. A parameter reference that would point at the
callee's own binder is therefore replaced by `undefined`.

So self-recursion must be encoded as:

```text
node_function_self : 0
```

inside the binding function, not as a reference to the let binding. This is
exactly the DSL's "recursion names the function at the definition site" rule.
Any sugar implementation must perform this rewrite, tracking function nesting
depth.

This also shows why a plain let is not the right place to solve recursion: the
interesting cases are precisely the ones the call encoding cannot see.

### 3.5 Caching is orthogonal

Because the evaluator caches parameter computation, the call encoding and a
`let` node are equivalent. That is an argument **against** adding a node just
for caching. The natural place to add sharing is `Argument` / `FunctionContext`
in the evaluator (call-by-need memoization), which is where it is implemented.
The existing call lowering then gets sharing automatically for every binding. A
`let` node would only be a marker saying "please cache here"; the cache would
still have to be implemented.

Caveats:

- Caching is a performance change, not a result change, for pure terminating
  programs.
- It changes closure representation and memory retention; the computation docs
  currently emphasize that captured unevaluated arguments remain finite objects.
  Be deliberate about that.
- If the agent protocol later has a fuel/step budget, sharing becomes observable
  through budget success/failure. That is an argument for implementing good
  call-by-need, not necessarily for a new node.

## 4. Mutual recursion: the hard case

### 4.1 Self-recursion already works

A single recursive local function needs no `letrec`:

```text
(λf. f(5)) (λx. ... (node_function_self : 0)(x - 1) ...)
```

The outer let binding provides the function value; the inner self-reference
provides recursion. The existing `capture_argument` preserves self-references
inside the argument, because they are bound inside it. So ordinary `let` plus
self-reference is enough for self-recursion.

### 4.2 Top-level mutual recursion already works

If the functions are atoms in the knowledge rather than local bindings, mutual
recursion is trivial: `even`'s body refers to the atom `odd`, which has
`node_function` in the knowledge and therefore parses as a function node. The
knowledge provides the shared namespace.

This is the pragmatic escape hatch: **local mutual recursion is only needed when
the definitions specifically must not be global knowledge atoms.**

### 4.3 Local mutual recursion cannot be solved by naive collapsing

For:

```text
letrec f = λx. ... g ...
       g = λx. ... f ...
```

there is no finite substitution that turns both into ordinary self-references.
Inlining `g` into `f` makes `f` refer to `f`, but `g` still needs to exist
wherever it is used; inlining it everywhere either diverges or duplicates the
cycle. One of the following is needed:

1. **A tagged dispatcher**: one self-recursive function `d` with a selector tag;
   `f` becomes `d(0)`, `g` becomes `d(1)`; recursive calls go through `d`. This
   is expressible in the current core using self-reference.
2. **A fixpoint combinator**: define `fix` in the knowledge and encode the
   mutual knot with a selector/tuple. Also expressible, but noisy.
3. **Open recursion**: pass the other functions as parameters and thread them
   through all call sites. Explicit but invasive.
4. **A native `letrec` binder**: all function bindings are simultaneously in
   scope; the evaluator provides the recursive environment directly.

"Collapse until hitting a self-reference" works for a *single* recursive
function. For mutual recursion it is not a general algorithm; it is a
dispatcher/fixpoint encoding in disguise.

There is also a structural-model constraint: **cyclic values are not
representable in a well-founded object model.** `letrec x = {x}` cannot be a
finite object. Recursion must live in function syntax, where the body is finite
and only the evaluation cycles. Therefore any native recursive-binding feature
should be restricted to **function bindings**, not arbitrary values.

### 4.4 If a `letrec` node is added, design it as a separate, function-only binder

A coherent native design would be:

- bindings must be functions (or function-valued expressions),
- all bindings are in scope simultaneously,
- references to them use a local-reference representation (either a new
  `node_local` depth space, or a carefully documented extension of parameter
  depths),
- evaluation creates the recursive function environment before evaluating the
  body,
- capture treats each binding body as a function body at the appropriate depth,
- unguarded recursion (`letrec f = f`) has a defined outcome (likely
  divergence, or rejection at parse time),
- the object encoding has a stable order and identity for bindings.

That is a significant semantic addition. It is justified if local mutual
recursion is a product requirement. It is not justified merely to render
`let cubed = ...` nicely.

## 5. Option 1 vs option 2, honestly

| Concern | New `let` node | Sugar/view over calls |
|---|---|---|
| Semantics | Same as call, unless sharing/recursion is added | Exactly the call semantics |
| Duplicate encodings | Yes; no normalization in the model | No; one canonical object |
| Evaluator/capture changes | New kind, task, capture arm, depth rules | None |
| Docs/tests/DSL surface | New node everywhere | Sugar specification + UI tests |
| UI implementation | Simple decode, but N-ary binding encoding is awkward | Pattern grouping, chain rebuilding, selection paths |
| Intent preserved | Explicit in the object | Implicit in names, or an annotation |
| Self-recursion | Needs a rule anyway | Rewrite to `node_function_self` |
| Mutual recursion | Only if designed as `letrec` | Not solved; needs encoding or a separate node |
| Caching | Not automatic; still needs evaluator support | Automatic once arguments are memoized |
| Structural equality/queries | Two objects for one meaning | One object for one meaning |
| Agent DSL/corpus | Two canonical spellings | Printer can re-sugar the canonical pattern |

The UI complexity of option 2 is real, but it is **bounded and testable**: a
recognizer over `Call(Function, value)` chains, a view that maps rows to edits,
and a rebuild function. The explorer already projects objects into visual
blocks; infix `+`, `if`, and function blocks are all projections over object
shapes. A `let` block is another projection.

The UI complexity of option 1 is **not** zero either: a multi-binding block
still needs chain grouping if unary lets are used, and a native N-ary let needs
an ordered binding encoding. And the semantic/duplication cost is paid on top.

## 6. Recommendation

### For local aliases: use sugar, not a node

Concretely:

1. Define the surface block:

   ```text
   let a = e1
       b = e2
   in body
   ```

   with **sequential** scope (like JavaScript `let` / Scheme `let*`), because
   that is what nested calls express naturally. Do not silently make it
   Haskell-style recursive; that is a different feature.

2. Lower to:

   ```text
   (λa. (λb. body) e2) e1
   ```

   References to earlier bindings remain ordinary `node_parameter` depths.
   References to the current binding inside its own *function* value become
   `node_function_self` at the appropriate depth.

3. Store binding names as `parameter_name` annotations on the synthetic
   functions. The UI already resolves parameter depths to names, so references
   render as `a`, `b`, etc. with no extra mapping.

4. In the explorer, add a pure recognizer that groups a chain of
   `Call(Function, value)` nodes into a `LetBlock`. Render it as a block;
   `setObject` rebuilds the chain. Keep the recognizer as a `Block` variant, not
   a fake `NodeType`, so the node model stays one-to-one with real node kinds.

5. If grouping by pattern alone feels too implicit, add a **non-semantic
   display annotation** (for example "show as let block") on the outermost node.
   That preserves the author's display intent without inventing a computation
   node. The annotation is only a hint; the structure is still the truth. This
   is the same philosophy as "annotations are knowledge."

6. Add the same `let` sugar to the DSL spec, with the same desugaring, and let
   the printer re-sugar canonical beta-redex chains. This keeps desktop, agents,
   and docs on one notation.

7. If caching matters, implement call-by-need in `Argument`/`FunctionContext`.
   The sugar then benefits automatically. Do not encode cache points as a node
   until there is a concrete sharing problem that call-level caching cannot
   solve.

**Self-recursion** falls out of this: rewrite self-name references to
`node_function_self` inside the binding value. No new node.

### For mutual recursion: decide explicitly

- If it is **not** a product requirement: document the encodings (top-level
  `def` atoms, tagged dispatcher, fixpoint) and leave it to the user. The
  current model is expressive enough; it is just not ergonomic.
- If it **is** a product requirement: add a **separate `letrec` node**,
  function-bindings only, simultaneous scope, native binder semantics. Do not
  try to detect a dispatcher encoding in the frontend and pretend it is a
  letrec block; that is fragile and hides the real encoding. Do not make a
  generic `let` node do double duty.

### If a first-class `let` is worth it anyway

Then make it a **real node**, not a source-preserving desugaring:

- unary, nested lets (avoids ordered N-ary binding encoding),
- lazy value, evaluated/captured in the outer context,
- body under an additional binder,
- explicit decision whether the binder is an implicit function (reuse
  `node_parameter`, document self-reference behaviour) or a new local reference
  kind,
- real `capture` and `evaluate` arms,
- `into_object` reproduces the let encoding,
- accept that `let` and the equivalent call are two structurally different but
  semantically equal objects.

That is a legitimate design if the persistent AST construct itself is the
product. It is not necessary for the visual alias feature.

## 7. The two decisions to pin down first

1. **Is local mutual recursion a must-have, or is "mutual recursion lives in
   top-level knowledge atoms" acceptable?**
   This is the only part that genuinely pushes toward a new node.

2. **Is one multi-binding block as a single selection/edit unit required, or is
   a nested chain of single bindings acceptable?**
   If the former, a real N-ary node may be more uniform for the editor. If the
   latter, the sugar/view route is much cheaper.

The default recommendation is: **sugar for `let`, user-encoded or separate-node
`letrec`, and call-by-need memoization rather than a cache-marker node.**
