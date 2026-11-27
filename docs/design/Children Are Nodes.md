# Children Are Nodes

**Status: resolved.** Every child position of a node is itself a node. An object that is to be used as plain data in a child position must be quoted with a literal node, `⟨node_literal ·· %object%⟩`. This document records the alternatives, the arguments on both sides, and why the requirement is kept.

The rule is specified in [Chapter 3](../public/3%20-%20Computation.md#children-are-nodes). This document is the design record for that rule.

Examples use decimal integers and `"..."` for text as readable shorthand; both are ordinary objects in the model.

## The Question

A node kind declares required properties, and the value of such a property is a _child expression_. For example, an add node has `node_add_first` and `node_add_second`, and an if node has `node_if_condition`, `node_if_then`, and `node_if_else`. The evaluator must decide what an object in such a position means.

Two models are possible:

- **Explicit literalization.** Every child position must parse as a node. Plain data is introduced with a literal node: `⟨node_literal ·· %object%⟩`.
- **Implicit literalization.** A child that does not parse as a node is treated as its own literal. This is _auto-literalization_.

The second model is not one design but a family. The variants differ in what happens inside a child that is not itself a node:

1. **Shallow auto-literalization.** The child object is taken as a value. Node-shaped objects nested inside it remain inert.
2. **Implicit structural construction.** A non-node composite or construction is treated like a `NewComposite` or `NewConstruction` node, so its parts are evaluated and a new object is built.
3. **Recursive mixed traversal.** The evaluator walks the child object, evaluates subobjects that happen to be nodes, and keeps everything else as data. This is the variant that tries to evaluate nested code without an explicit constructor.

The current model is the first option in the first list: explicit literalization.

## Narrower Alternatives

A fallback does not have to accept every failed recognition. Three narrower rules are sometimes proposed.

### Fallback Only For Objects That Cannot Be Nodes

Constructions can never be nodes, and a composite whose members contain no `node_*` construction can never become one. Accepting those raw objects would remove some wrappers. But it creates an exception to the homogeneous rule, it does not help with atoms whose recognition is knowledge-dependent, and it still needs `node_literal` for node-shaped data. The wrapper count is small, and surface syntax can remove it without changing the evaluator. The exception buys little.

### Fallback Only For Objects With No Recognized Node Property

This is the narrowest reading of "matches no node kind": accept the child as data only when it has no recognized node property at all. Constructions and composites without `node_*` members qualify, but so do atoms, and an atom can become an atom-defined node when knowledge grows. The same child object would then change from data to code. The rule is also fragile: a typo in the only `node_*` property of an intended node moves it from "error" to "data", while a partial typo leaves other recognized properties and keeps it an error.

### Fallback Only When The Parent Obviously Expects Data

There is no marker in the parent that says a child is data. Adding one is just `node_literal` under another name. If the marker is inferred from the child's shape, it is the general auto-literalization again.

## The Rule In The Current Model

An object is a valid node iff:

- it matches exactly one node kind;
- every required property has exactly one matching construction, except for the repeatable properties `node_new_set_item` and `node_new_composite_property`, which may occur any positive number of times;
- every child position itself parses as a node; and
- any other property is ignored, provided it does not use the reserved `node_*` namespace.

Child positions are parsed recursively. The literal node is the only place where an arbitrary object enters a node tree without being parsed as a node. Its `node_literal` value is not parsed and not evaluated; the literal evaluates to that object. This is the quotation boundary of the evaluator.

A bare construction cannot be a node. A construction in a child position must therefore be quoted as well.

Additional properties that are not part of the recognized kind are not child positions. They are annotations and other metadata; they may contain arbitrary objects without a literal wrapper, provided they do not use the reserved `node_*` namespace. The requirement applies to the declared children of the node kind, not to every object that happens to be attached to the node.

For example, this is an add node with explicit data:

```
⟨
    node_add_first ·· ⟨node_literal ·· 1⟩,
    node_add_second ·· ⟨node_literal ·· 2⟩
⟩
```

The same node may carry arbitrary metadata without quoting it, because `summary` and `source` are not child positions:

```
⟨
    node_add_first ·· ⟨node_literal ·· 1⟩,
    node_add_second ·· ⟨node_literal ·· 2⟩,
    summary ·· "adds one and two",
    source ·· ⟨file ·· "main.everything", line ·· 4⟩
⟩
```

Code can be returned as data by quoting it. The payload may itself be a valid node; it is still data:

```
⟨node_literal ·· ⟨
    node_add_first ·· ⟨node_literal ·· 1⟩,
    node_add_second ·· ⟨node_literal ·· 2⟩
⟩⟩
```

## Arguments For The Requirement

### A Node Tree Contains Only Nodes

Except for the payload of a literal, every object that the evaluator treats as program is a node. Every child has a recognized node kind, and that kind defines how it is evaluated. There is no second case of "raw object that the parent must interpret". Program semantics are therefore uniform: the meaning of a node is a function of its kind and its node children. A reader or tool can traverse the tree and know that every position is code. The alternative introduces a second kind of position whose meaning depends on recognition failure.

### The Code/Data Boundary Is Explicit

`node_literal` says "this object is data". The boundary is visible in the object, not inferred from its shape. The evaluator does not have to guess whether a composite is a record or an expression. This matters especially with [open node definitions](./Open%20Node%20Definitions.md): a data composite may accidentally contain `node_*` properties and match a node kind. A literal wrapper is the reliable way to say that it is data. Auto-literalization only handles objects that match no kind; it does not provide a way to use a node-shaped object as data. That still requires `node_literal`. So the alternative adds a second, implicit boundary without removing the explicit one.

### Recognition Failures Stay Errors

In the current model, a child that should be a node but is malformed makes the parent invalid, and the error reports which property failed. The parser distinguishes "matches no node kind", "looks like a kind but is missing a required property", "has a duplicate property", "matches several kinds", and "has an invalid child". These distinctions exist so that mistakes can be reported precisely.

Under a broad fallback, every one of these failures would become data. An intended node with a missing property would silently become a literal, and the mistake would surface later as `undefined` or as a wrong value. A narrower fallback could accept only objects that have no recognized node property at all and keep the other failures as errors. That is extra machinery, and it still hides the case where a misspelling leaves no recognized node property. The current rule has one answer for every failure: a child must be a node.

### The Interpretation Does Not Depend On Knowledge

Node recognition for atoms is knowledge-relative. An atom can become a function by gaining `node_function ·· %body%`. Under auto-literalization, whether a child is code or data is decided by recognition. The same child object can therefore be data in one knowledge and code in another, and reparsing a stored program after the knowledge grows can change its meaning.

The current model keeps this stable. A literal is always data, regardless of what the knowledge says about its payload. Adding knowledge can make a previously invalid program valid, but it does not silently reinterpret a valid program's data as code.

### Capture Has A Well-Defined Boundary

Functions are closures, and capture replaces free parameter and self references in the body. A literal is opaque to capture: quoted code is not rewritten. This is what makes it possible to return code as data. If a child could be an arbitrary object, capture would have to decide whether to descend into it:

- if it descends, it rewrites data, and a function can no longer return a quoted object that contains a parameter-shaped node;
- if it does not descend, code nested in data is not captured and can escape with dangling references.

The recursive mixed variant has no principled answer. The explicit literal is the answer: descend through node children, stop at literals. Capture also escapes substituted runtime values into literals precisely so that a value which happens to look like a node is not reinterpreted as code. Auto-literalization cannot replace that, because it only wraps values that fail recognition; a runtime value that _is_ node-shaped would still be interpreted as code.

### Code As Data Still Needs A Literal

The question "how do you return code as data?" has only one answer in either model: quote it. `⟨node_literal ·· %code%⟩` evaluates to `%code%` without evaluating it. Without a literal node, any object that matches a node kind is code, so there is no way to return it as data. Auto-literalization handles only non-node data, which is the easy case. The hard case—data that looks like code—still needs the explicit wrapper.

Since the literal node must exist anyway, making it the _only_ boundary is simpler than maintaining an implicit boundary for some objects and an explicit one for others. Quotation does not make code unreachable: a literal that contains a function object can still be called, because a call evaluates its callee and then parses the result as a node. Re-entry is explicit, which is the point.

### Dynamic Construction Is Already Explicit

`NewConstruction`, `NewSet`, and `NewComposite` build objects from evaluated children at runtime. They are the dynamic counterpart of a literal: a literal quotes a static object, while the New* nodes construct a new object from evaluated parts. If a program needs a data structure whose fields are computed, it uses the appropriate New* node, not an implicit traversal of a raw object.

This is already explicit and compositional. Auto-literalization would add a third path—raw objects in child positions—that overlaps with both literal and New*, and whose behavior depends on recognition. The current model has one rule: quote static data with a literal; build dynamic data with a New* node; everything else is code.

### Round-Tripping And Tooling

A parsed node retains its source object, and converting it back reproduces that object exactly. The source already contains the literal wrappers, so the round-trip is exact. Under an implicit fallback, the parsed tree would either have to rewrite the source by inserting wrappers, so the round-trip no longer reproduces the input, or introduce a second raw-data representation that serializes differently. Both options complicate the invariant. Similarly, tools that inspect a node tree can rely on every child being a node; they do not need a second code path for raw data.

### Homogeneous Implementation

`Node::new` returns a tree whose children are nodes, not arbitrary objects. The only place an arbitrary object enters is the literal node, and it is a leaf. Evaluation, capture, and serialization all match on nodes and never on an unparsed child. This is a small, local boundary that keeps the evaluator total. The alternative would spread a second case through recognition, evaluation, and capture.

## Arguments Against The Requirement

### Verbosity

At the object level, every data argument needs a wrapper. The difference is visible:

```
⟨node_add_first ·· ⟨node_literal ·· 1⟩, node_add_second ·· ⟨node_literal ·· 2⟩⟩
⟨node_add_first ·· 1, node_add_second ·· 2⟩
```

The second form is easier to read, and data-heavy programs accumulate wrappers. This is the strongest practical argument for auto-literalization.

### Discoverability

A new user will write plain objects in child positions. The current model rejects them with "child is not a node", which is correct but not self-explanatory. Auto-literalization makes the obvious program work. The requirement asks the user to learn a concept—quotation—before writing their first expression with data.

### Everything Is An Object

The model's slogan is that code and data are the same kind of thing. A wrapper around data can look like a special case that undermines the uniformity. If every object is a value, why can an object not appear directly where an expression is expected?

### Tools And Generated Code

A tool that assembles a node from a list of values must remember to quote each value. If it forgets, parsing fails. Auto-literalization would make the tool's output more forgiving. The same applies to code generation and to interactive editors that insert values into a program.

### Static And Dynamic Data Use Different Mechanisms

Static data uses `node_literal`; dynamic data uses `NewConstruction`, `NewSet`, and `NewComposite`. Two mechanisms for one idea can feel redundant. The alternative would let a raw object stand for static data in the same positions where a New* node builds dynamic data.

### Nested Code In Data Is Inert

With a literal, code inside quoted data is not evaluated. There is no general `eval` node in the current model. A program that wants to store an expression and evaluate it later must use a call for a function, or wait for a dedicated evaluation node. Recursive mixed traversal would make templates and embedded expressions work without an explicit constructor. This is a real expressiveness argument for the alternative, although it is also the reason the alternative is dangerous.

## Why The Costs Are Acceptable

The costs above are real, but they are smaller than they look, and the alternatives pay for them elsewhere.

### Metadata Does Not Need Wrappers

The requirement applies to child positions, not to additional properties. Annotations, source locations, comments, and tooling results can be arbitrary objects attached to a node without literalization. The annotation case that motivates open node definitions is unaffected.

### One Wrapper At The Data Boundary

A record passed as one child is quoted once:

```
⟨node_literal ·· ⟨name ·· "Alice", age ·· 42⟩⟩
```

The inner values are data inside the literal and are not child positions. The wrapper count is proportional to the number of data _arguments_, not the size of the data.

### Surface Syntax Belongs In A Surface Language

The object encoding does not have to be the human interface. A surface language can write `1 + 2`, `f(x)`, or `[1, 2, 3]` and emit the explicit literal wrappers. That is sugar, and it can be as ergonomic as the alternative without changing the evaluator's invariant. This is the right place to solve verbosity. A surface parser can know the syntactic category of an expression—literal, identifier, call, record—so it does not need to guess from node recognition, and it does not make the code/data distinction knowledge-dependent.

The distinction matters: sugar that inserts literal nodes before evaluation preserves the rule, while evaluator fallback weakens it. The first changes only how programs are written; the second changes what a node tree is allowed to contain.

### New* Nodes Are The Dynamic Counterpart

`node_literal` quotes a static object; `NewConstruction`, `NewSet`, and `NewComposite` construct an object from evaluated parts. This is the usual distinction between quotation and construction. It is not two mechanisms for the same case; it is the static and dynamic forms of the same idea. A program that computes a data structure uses New*; a program that embeds a fixed data structure uses a literal.

### Error Messages Can Teach The Boundary

A rejection can say that the child is not a node and suggest wrapping it in `node_literal` if the intent was data. The current error already identifies the property that failed. With a good message, the discoverability cost is a one-time lesson, while the explicit boundary remains visible in every program.

### Recursive Mixed Traversal Has No Coherent Rule

The apparent expressiveness of evaluating code inside data comes at the price of not being able to store data that contains node-shaped objects. If a parameter-shaped object inside a record is evaluated, the record is not data. If it is not evaluated, the embedded code does not run. An explicit literal chooses one behavior; an implicit traversal must choose per object shape, which is exactly the accidental interpretation the model avoids. A dedicated evaluation node would be a clearer way to add the missing expressiveness later.

### The Wrapper Is An Object, Not A Conversion

The literal node is itself an object in the object model. Quoting does not change the data's identity or copy it into a different type; it adds a node that evaluates to it. "Everything is an object" is preserved. What is not preserved is the idea that the evaluator should guess whether an object is code or data. The model says: same object language, explicit quotation.

## Verdict

Child positions are required to be nodes. Plain data in a child position is quoted with `⟨node_literal ·· %object%⟩`; dynamic data is built with `NewConstruction`, `NewSet`, or `NewComposite`; everything else in a child position is code.

The decisive arguments are:

- a node tree contains only nodes, so evaluation has one uniform case and program semantics are clear;
- the code/data boundary is explicit and remains reliable even for data that happens to match a node kind;
- malformed children remain errors instead of silently becoming data;
- whether a child is code does not depend on the knowledge base;
- capture has a precise boundary: it descends through node children and stops at literals, which is also what makes returning code as data possible;
- a literal node is unavoidable for node-shaped data, and dynamic construction is already explicit, so auto-literalization would add a second implicit path rather than replace anything;
- source objects round-trip exactly, and tools can rely on a homogeneous node tree.

The ergonomic cost is real, but it is confined to child positions and can be moved into a surface syntax that emits literal nodes. The alternative's convenience comes from making recognition failure meaningful, which is the same thing as making malformed programs silent and making data interpretation depend on shape and knowledge. The explicit boundary is the smaller and more predictable design.
