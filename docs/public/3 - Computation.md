# Chapter 3 - Computation

Everything has a computation system where functions and expressions can be evaluated. Expressions are modelled in Everything as objects. The whole system is pure and based on lambda calculus with additional node kinds.

Evaluation is knowledge-relative. Node recognition, atom property lookup, queries, and calling atom-defined functions all consult the knowledge that is supplied to the evaluator. A subset of the system can be evaluated in an empty knowledge, but anything that reads properties or queries atoms needs a knowledge context.

This chapter describes the implemented model. It distinguishes the reference evaluation, which defines the meaning of a program, from the implementation's evaluation strategy.

## Node Objects and Recognition

An object is a valid _node_ iff it matches exactly one node kind described in this chapter and every child position parses as a node. Node matching is shallow and structural:

- A node object is an atom or a composite. The required properties are read from the atom's values in the knowledge or, for a composite, from the composite's members. A bare construction cannot be a node.
- For every required property of a node kind, the object must have exactly one matching construction, unless the property is repeatable. For example, an add node needs exactly one `node_add_first` value and exactly one `node_add_second` value. The repeatable properties are `node_new_set_item`, which a [new-set node](#new-set) may have any positive number of times, and `node_new_composite_property`, which a [new-composite node](#new-composite) may have any positive number of times. A required property with zero values, or with several values when it is not repeatable, means the kind does not match.
- If the object matches no node kind, it is not a node. If it matches more than one kind, it is also not a valid node; node kinds may not be combined.
- Additional properties that are not part of the recognized kind are ignored. This makes node definitions _open_: annotations and other metadata may be attached without changing the node kind. The `node_*` constructors are reserved and must not be used for annotations, because they can change the recognized kind. The design rationale is recorded in [Open Node Definitions](../design/Open%20Node%20Definitions.md).

Recognition only examines the object's own top-level properties (the atom's statements or the composite's members). It does not descend into values; that happens during child parsing below.

### Children Are Nodes

Every child position of a node is itself parsed as a node, recursively. There is no implicit literal fallback:

- Every child must itself parse as a node. If any child fails, the whole object is not a valid node.
- To use plain data as a child, it must be wrapped in a literal node: `⟨node_literal ·· %object%⟩`.

The design rationale, including the arguments for and against implicit literalization, is recorded in [Children Are Nodes](../design/Children%20Are%20Nodes.md).

The object encoding produced by the implementation therefore always wraps literals explicitly. For example, `1 + 2` is encoded as:

```
⟨
    node_add_first ·· ⟨node_literal ·· 1⟩,
    node_add_second ·· ⟨node_literal ·· 2⟩
⟩
```

Node recognition is not itself an evaluator operation. The evaluator takes an already parsed node together with a knowledge. Conversion from an object to a node can fail. A parsed node retains the object it was parsed from, so converting it back to an object reproduces that object exactly, including properties that are not part of the node kind.

## Reference Evaluation

The meaning of a node is defined by a _reference evaluation_. It is strict in data: when a canonical set is evaluated, all of its items are evaluated. Function application is non-strict: a call binds its argument as an unevaluated thunk, and the argument is evaluated only when the parameter is referenced. Control nodes follow their short-circuiting rules. A program (a node together with a knowledge) is _well-formed_ if:

- the reference evaluation terminates for every iteration order,
- its result is the same for every iteration order, and
- every object the evaluation materialises is valid in the knowledge (see [Chapter 2](./2%20-%20Knowledge.md)).

Its result may be the atom `undefined`; producing `undefined` is an ordinary, specified result.

The validity condition extends knowledge validation to dynamically created objects: a program that materialises an invalid object has no specified result. As with termination and order-independence, this is a semantic precondition that the evaluator does not check; implementations may check materialised objects eagerly or lazily, which only changes how much work is done.

A well-formed program may contain subterms that do not terminate, provided those subterms are never demanded. For example, feeding a non-terminating argument to a function that ignores its parameter is well-formed and produces the function's result.

For every well-formed program, the implementation produces that result. In particular, a well-formed program cannot observe sharing or iteration order; implementation choices only change how much work is done.

The implementation additionally defers set operations; see [Evaluation Strategy](#evaluation-strategy).

If a program is not well-formed, it has no specified result. The implementation may diverge, or it may happen to terminate because a divergent subcomputation was never demanded, or it may produce a result that depends on iteration order. This is not guaranteed and may change between implementations or versions. It is unrelated to the atom `undefined`, which is an ordinary value.

## Truthiness

Truthiness is deliberately narrow:

- An object is _truthy_ iff it is equal to `true`.
- An object is _falsy_ iff it is equal to `false`.
- Every other object, including sets, constructions, and `undefined`, has _undefined truthiness_.

There is no general "non-empty means true" coercion in the language. Boolean operations therefore produce only `true`, `false`, or `undefined`; they do not treat a non-empty set as true.

## Functions

An object is a _function_ iff it has the property `node_function ·· %body%`. The associated value `%body%` is the function body. Functions have exactly one parameter. Multiple arguments are encoded by currying: a function returns another function that takes the next argument.

Evaluating a function does not evaluate its body. It produces a function object: a copy of the original in which free references to enclosing functions have been captured (see [Captures](#captures)). All other properties of the function and of nodes inside its body, including annotations and unknown properties, are preserved. The body is evaluated only when the function is called.

An atom-defined function materialises into a composite when it is evaluated: the atom's properties become the members of the composite, with the `node_function` value replaced by the captured body. An atom cannot be given a new body, so it cannot remain an atom.

## Parameter References

An object is a _parameter reference_ iff it has the property `node_parameter ·· %depth%`. `%depth%` is a non-negative integer that must fit into an unsigned 32-bit integer. It references the argument bound to the parameter of an enclosing function:

- `0` is the innermost enclosing function,
- `1` is the next enclosing function, and so on.

If the bound argument is an already evaluated value, the parameter reference evaluates to that value. Otherwise it evaluates the bound argument expression. The result of that evaluation is cached in the binding and reused by later references to the same parameter. If the depth does not identify an enclosing function at evaluation time, the parameter reference evaluates to `undefined`.

## Function Self-References

An object is a (function) _self-reference_ iff it has the property `node_function_self ·· %depth%`. `%depth%` has the same meaning and restrictions as for parameter references. These nodes support recursive functions without a fix-point operator.

A self-reference evaluates to the enclosing function object itself, not to the result of calling it. The function object can then be called with a normal call node. An out-of-range depth evaluates to `undefined`.

## Captures

Functions are closures. When a function is evaluated inside an enclosing function call, every free parameter reference and free function self-reference in its body is replaced by the binding from the enclosing call. References to functions and parameters bound inside the function remain unchanged.

A parameter binding that is an already evaluated value is escaped into a literal, so a value that happens to look like a node is not reinterpreted as code when the function is later called. A parameter binding that is an unevaluated argument is substituted as a shared thunk; it is still not evaluated at capture time, and several references to the captured parameter share one evaluation. An already evaluated thunk is escaped into a literal just like an evaluated value. This is also why functions can escape the call that created them. Capturing a function transforms the original body object: substituted references are replaced in place, while annotations and unknown properties of the body and its nested nodes are preserved. Capturing an atom-defined function produces a set, because the captured body replaces the atom's `node_function` value.

Evaluation of a function does not capture runtime parameter values from functions that have not been called yet; capture happens when the function object is produced. Nested functions therefore capture the environment of the call in which they are evaluated.

## Function Application

An object is a _function application/call_ iff

- it has the property `node_call_callee ·· %callee%` and
- it has the property `node_call_with ·· %argument%`.

A call evaluates `%callee%`. It does not evaluate `%argument%`. The argument is closed over the caller's context and bound to the function's parameter as an unevaluated thunk. The callee result is interpreted as a function:

- the callee result is parsed as a node in the current knowledge, and it must be a `node_function` node;
- if the callee is not a function, or the callee object is not a valid node, the call evaluates to `undefined` and the argument is not evaluated.

The body is evaluated with the function context pushed. The context is popped when the body finishes. A parameter reference evaluates the bound argument. The argument is evaluated at most once, when the parameter is first referenced, and the result is cached for later references.

## Literals

An object is a _literal_ iff it has the property `node_literal ·· %object%`. A literal evaluates to `%object%` without interpreting or evaluating it. This prevents a value from being treated as a node. Literals are also the boundary that capture uses when it inserts runtime values into code.

## Logical And

An object is an _and node_ iff

- it has the property `node_and_first ·· %left%` and
- it has the property `node_and_second ·· %right%`.

It evaluates `%left%`. If `%left%` is `false`, the result is `false`; if it has undefined truthiness, the result is `undefined`. If `%left%` is `true`, it evaluates `%right%` and returns its truthiness coerced to a boolean.

An and node therefore never returns the value of `%right%` itself.

## Logical Or

An object is an _or node_ iff

- it has the property `node_or_first ·· %left%` and
- it has the property `node_or_second ·· %right%`.

It evaluates `%left%`. If `%left%` is `true`, the result is `true`; if it has undefined truthiness, the result is `undefined`. If `%left%` is `false`, it evaluates `%right%` and returns its truthiness coerced to a boolean.

An or node therefore never returns the value of `%right%` itself.

## Logical XOr

An object is an _xor node_ iff

- it has the property `node_xor_first ·· %left%` and
- it has the property `node_xor_second ·· %right%`.

It evaluates both children. The result is `true` iff exactly one of the two results is truthy, and `false` iff both are truthy or both are falsy. If either result has undefined truthiness, the result is `undefined`.

## Logical Not

An object is a _not node_ iff it has the property `node_not ·· %inner%`. It evaluates `%inner%` and returns the opposite boolean. If `%inner%` has undefined truthiness, the result is `undefined`.

## Equality

An object is an _equal node_ iff

- it has the property `node_equal_first ·· %left%` and
- it has the property `node_equal_second ·· %right%`.

It evaluates both children. The result is `true` iff the two results are structurally equal as objects.

## Less Than

An object is a _less-than node_ iff

- it has the property `node_less_first ·· %left%` and
- it has the property `node_less_second ·· %right%`.

It evaluates both children. If both results are integers, the result is `true` iff `%left% < %right%`, and `false` otherwise. If either result is not an integer, the result is `undefined`. There is no ordering for sets, constructions, or other atoms.

## Addition

An object is an _add node_ iff

- it has the property `node_add_first ·· %left%` and
- it has the property `node_add_second ·· %right%`.

It evaluates both children. If both results are integers, the result is their sum. If either result is not an integer, the result is `undefined`.

Integers are arbitrary precision, so addition cannot overflow.

## Multiplication

An object is a _multiply node_ iff

- it has the property `node_multiply_first ·· %left%` and
- it has the property `node_multiply_second ·· %right%`.

It evaluates both children. If both results are integers, the result is their product. If either result is not an integer, the result is `undefined`.

As with addition, integers are arbitrary precision and cannot overflow.

## Sets

Several node kinds take or produce _sets_. The set model is open (see [Chapter 4](./4%20-%20Common%20Models.md#sets)): **every atom and every composite is a set.** The _items_ of a set are the objects `%item%` for which it has the property `contains ·· %item%`: the statements of an atom in the knowledge, or the members of a composite. A construction is not a set. `{...}` is syntactic sugar for the canonical composite whose members are the corresponding `contains` constructions, and `{}` is the empty composite.

Whenever this chapter says that a node produces a set, the result is a canonical set composite built from the items. The other properties of an operand are not items of that operand and do not participate when it is used as a set; [Properties Of](#properties-of) makes the properties of an object explicit as items.

Because the constraint of `contains` accepts every value for every subject, the `contains` properties of a set place no demands on their items beyond their own validity. Collecting computed items into a canonical set therefore never runs the constraints of the collected objects.

## Set Count

An object is a _set-count node_ iff it has the property `node_set_count ·· %inner%`. It evaluates `%inner%`. If the result is an atom or a composite, it returns the number of items of that result. If the result is a construction, the result is `undefined`.

Most atoms and composites have no items. Use [Properties Of](#properties-of) to count the properties of an object: `⟨node_set_count ·· ⟨node_properties_of ·· %object%⟩⟩` is the number of properties of the object.

## Properties Of

An object is a _properties-of node_ iff it has the property `node_properties_of ·· %inner%`. It evaluates `%inner%` and returns the canonical set whose items are all properties of the result:

- for an atom, the properties it has in the knowledge;
- for a composite, all of its members;
- for a deferred set, the `contains ·· %item%` constructions corresponding to its items, since those are the properties of its composite representation;
- for a construction, `undefined`.

Unlike [Set Count](#set-count), this makes the properties of an object explicit as items. In particular, the items of the result are the properties themselves, not the items of the original object. Counting the properties of an atom or composite is therefore written as `⟨node_set_count ·· ⟨node_properties_of ·· %object%⟩⟩`. An atom or composite without properties evaluates to the empty set.

## Has Value

An object is a _has-value node_ iff it has the property `node_has_value ·· %inner%`. It evaluates `%inner%` and returns `true` iff the result has at least one value:

- for a composite, whether it has at least one item;
- for an atom, whether it has at least one property in the knowledge;
- for a construction, `undefined`.

This is the existential counterpart of [Set Count](#set-count). For an atom it is equivalent to `⟨node_set_count ·· ⟨node_properties_of ·· %inner%⟩⟩ > ⟨node_literal ·· 0⟩`. For a composite it is not equivalent, because [Properties Of](#properties-of) counts all members while a composite has a value only if it has a `contains` item.

## Has Property

An object is a _has-property node_ iff

- it has the property `node_has_property_subject ·· %subject%` and
- it has the property `node_has_property_property ·· %property%`.

It evaluates `%subject%`. If the result is a construction, the result is `undefined` and `%property%` is not evaluated. Otherwise it evaluates `%property%` and returns `true` iff `%property%` is a property of `%subject%`:

- for a composite, whether it is one of the composite's members;
- for an atom, whether it is a statement about the atom in the knowledge.

A deferred set is a composite whose members are `contains` constructions, so only those constructions are its properties. This is the property counterpart of [Set Contains](#set-contains); use [Properties Of](#properties-of) to obtain an object's properties as a set.

## Set Union

An object is a _set-union node_ iff

- it has the property `node_set_union_first ·· %left%` and
- it has the property `node_set_union_second ·· %right%`.

It evaluates both children. If either result is a construction, the result is `undefined`. Otherwise the result is the canonical set of the items of the left and the right result.

## Set Intersection

An object is an _set-intersection node_ iff

- it has the property `node_set_intersection_first ·· %left%` and
- it has the property `node_set_intersection_second ·· %right%`.

It evaluates both children. If either result is a construction, the result is `undefined`. Otherwise the result is the canonical set of the items that are items of both results.

## Set Difference

An object is a _set-difference node_ iff

- it has the property `node_set_difference_first ·· %left%` and
- it has the property `node_set_difference_second ·· %right%`.

It evaluates `%left%`. If the result is a construction, the result is `undefined`. If it has no items, the result is the empty set and `%right%` is not evaluated. Otherwise it evaluates `%right%`; if that result is a construction, the result is `undefined`; otherwise the result is the canonical set of items of the left result that are not items of the right result.

Node recognition requires both children to parse as nodes.

## Set Filter

An object is a _set-filter node_ iff

- it has the property `node_set_filter_set ·· %set%` and
- it has the property `node_set_filter_filter ·· %filter%`.

It evaluates `%set%` and `%filter%`. If `%set%` is a construction, or if `%filter%` is not a function, the result is `undefined`. Otherwise the result is the set of items of `%set%` for which the call of `%filter%` with the item as a literal argument evaluates to exactly `true`. Results that are `false` or have undefined truthiness are excluded.

## Set Map

An object is a _set-map node_ iff

- it has the property `node_set_map_set ·· %set%` and
- it has the property `node_set_map_mapper ·· %mapper%`.

It evaluates `%set%` and `%mapper%`. If `%set%` is a construction, or if `%mapper%` is not a function, the result is `undefined`. Otherwise the result is the set of results of calling `%mapper%` with each item of `%set%` as a literal argument. Because the result is a set, equal mapped values are not distinguished.

## Set Every

An object is an _set-every node_ iff

- it has the property `node_set_every_set ·· %set%` and
- it has the property `node_set_every_predicate ·· %predicate%`.

It evaluates `%set%` and `%predicate%`:

1. If `%set%` is a construction, the result is `undefined`.
2. If `%set%` has no items, the result is `true`; the predicate is not used.
3. If `%predicate%` is not a function, the result is `undefined`.
4. Otherwise the items are visited in the set's iteration order. Each item `%o%` is passed to `%predicate%` as a literal argument via `⟨node_call_callee ·· %predicate%, node_call_with ·· %o%⟩`. The first call whose result is not `true` determines the outcome: `false` yields `false`, and anything else yields `undefined`.
5. If every call yields `true`, the result is `true`.

`every` is the short-circuiting fold

```text
⟨
    node_set_fold_set ·· %set%,
    node_set_fold_folder ·· ⟨
        node_function ·· ⟨
            node_function ·· ⟨
                node_and_first ·· ⟨node_parameter ·· 1⟩,
                node_and_second ·· ⟨
                    node_call_callee ·· %predicate%,
                    node_call_with ·· ⟨node_parameter ·· 0⟩
                ⟩
            ⟩
        ⟩
    ⟩,
    node_set_fold_initial ·· ⟨node_literal ·· true⟩
⟩
```

and therefore inherits the [well-formedness caveat for folds](#correctness-and-associativity). Its result does not depend on iteration order unless the predicate produces both `false` and a result with undefined truthiness: a `false` visited first yields `false`, while an undefined-truthiness result visited first yields `undefined`. In particular, the result is order-independent if the predicate returns `true` or `false` for every element. If the result depends on iteration order, the program is not well-formed.

## Set Any

An object is an _set-any node_ iff

- it has the property `node_set_any_set ·· %set%` and
- it has the property `node_set_any_predicate ·· %predicate%`.

It evaluates `%set%` and `%predicate%`:

1. If `%set%` is a construction, the result is `undefined`.
2. If `%set%` has no items, the result is `false`; the predicate is not used.
3. If `%predicate%` is not a function, the result is `undefined`.
4. Otherwise the items are visited in the set's iteration order. Each item `%o%` is passed to `%predicate%` as a literal argument via `⟨node_call_callee ·· %predicate%, node_call_with ·· %o%⟩`. The first call whose result is not `false` determines the outcome: `true` yields `true`, and anything else yields `undefined`.
5. If every call yields `false`, the result is `false`.

`any` is the short-circuiting fold

```text
⟨
    node_set_fold_set ·· %set%,
    node_set_fold_folder ·· ⟨
        node_function ·· ⟨
            node_function ·· ⟨
                node_or_first ·· ⟨node_parameter ·· 1⟩,
                node_or_second ·· ⟨
                    node_call_callee ·· %predicate%,
                    node_call_with ·· ⟨node_parameter ·· 0⟩
                ⟩
            ⟩
        ⟩
    ⟩,
    node_set_fold_initial ·· ⟨node_literal ·· false⟩
⟩
```

and therefore inherits the [well-formedness caveat for folds](#correctness-and-associativity). Its result does not depend on iteration order unless the predicate produces both `true` and a result with undefined truthiness: a `true` visited first yields `true`, while an undefined-truthiness result visited first yields `undefined`. In particular, the result is order-independent if the predicate returns `true` or `false` for every element. If the result depends on iteration order, the program is not well-formed.

## Set Fold

An object is a _set-fold node_ iff

- it has the property `node_set_fold_set ·· %set%`,
- it has the property `node_set_fold_folder ·· %folder%`, and
- it has the property `node_set_fold_initial ·· %initial%`.

It evaluates `%set%`, `%folder%`, and `%initial%`:

1. If `%set%` is a construction, the result is `undefined`.
2. If `%set%` has no items, the result is the value of `%initial%`; the folder is not used.
3. If `%folder%` is not a function, the result is `undefined`.
4. Otherwise the accumulator starts as the value of `%initial%`. For each item `%o%` of `%set%` in iteration order, apply `%folder%` pairwise: the first call receives the previous accumulator and returns a function, which is then called with `%o%`. The result of that call becomes the new accumulator.
5. The final accumulator is the result.

In contrast to [Set Map](#set-map) and [Set Filter](#set-filter), a fold returns the final accumulator rather than a set. The accumulator may itself be a set. If a folder application does not return a function, the result is `undefined`; remaining items do not change that.

In object form, each step is:

```
⟨
    node_call_callee ·· ⟨
        node_call_callee ·· %folder%,
        node_call_with ·· ⟨node_literal ·· %accumulator%⟩
    ⟩,
    node_call_with ·· ⟨node_literal ·· %item%⟩
⟩
```

### Correctness and associativity

A fold is well-formed only if its result is independent of the order in which the set is traversed. A sufficient condition is that the folding operation is **associative**: for all values `x`, `y`, and `z`,

```text
(x ⊕ y) ⊕ z = x ⊕ (y ⊕ z)
```

An associative folder guarantees the same result for every order and grouping of the items. Associativity is sufficient but not necessary: a non-associative folder can still happen to produce the same result for every order of a particular set.

If the result depends on iteration order, the program is not well-formed (see [Reference Evaluation](#reference-evaluation)). The result is then determined by the set's iteration order, which is not specified.

The `and` node is not associative on all inputs. For example:

```text
false and undefined = false
undefined and false = undefined
```

Consequently, `and` is associative only when every operand has truthiness `true` or `false`. A folded value with undefined truthiness can therefore make the fold order-dependent.

Whether a particular folder or mapper satisfies this property depends on the behaviour of arbitrary nodes. Since the computation system is Turing complete, the property is undecidable in general: no algorithm can decide it for all nodes, and the evaluator cannot check or guarantee it. Correctness of a `fold` is therefore the programmer's responsibility. If order-independence cannot be established, the fold is not well-formed, and the evaluator gives no guarantee about the result.

## Set Contains

An object is a _set-contains node_ iff

- it has the property `node_set_contains_element ·· %element%` and
- it has the property `node_set_contains_set ·· %set%`.

It evaluates `%set%` first. If `%set%` is a construction, the result is `undefined` and `%element%` is not evaluated. Otherwise it evaluates `%element%` and returns `true` iff `%element%` is an item of `%set%`.

## Conditional Nodes

An object is an _if node_ iff

- it has the property `node_if_condition ·· %condition%`,
- it has the property `node_if_then ·· %then%`, and
- it has the property `node_if_else ·· %else%`.

It evaluates `%condition%`. If `%condition%` is truthy, it evaluates and returns `%then%`; if it is falsy, it evaluates and returns `%else%`; if it has undefined truthiness, the result is `undefined`.

## Set Unwrap

An object is an _set-unwrap node_ iff it has the property `node_set_unwrap_set ·· %set%`. It evaluates `%set%`. If the result is a construction, the result is `undefined`. Otherwise, if the result has exactly one item, that item is returned; if it has zero items or more than one item, the result is `undefined`.

There is no default branch in the current implementation.

## Is Atom

An object is an _is-atom node_ iff it has the property `node_is_atom ·· %inner%`. It evaluates `%inner%`. The result is `true` iff the result is an atom. Composites are not atoms and yield `false`.

## Is Construction

An object is an _is-construction node_ iff it has the property `node_is_construction ·· %inner%`. It evaluates `%inner%`. The result is `true` iff the result is a construction. Atoms and composites yield `false`.

## Is Composite

An object is an _is-composite node_ iff it has the property `node_is_composite ·· %inner%`. It evaluates `%inner%`. The result is `true` iff the result is a composite. Atoms and constructions yield `false`.

## Construction Constructor and Value

An object is a _construction-constructor node_ iff it has the property `node_constructor_of_construction ·· %construction%`. It evaluates `%construction%` and returns the constructor of the resulting construction. If the result is not a construction, the result is `undefined`.

An object is a _construction-value node_ iff it has the property `node_value_of_construction ·· %construction%`. It evaluates `%construction%` and returns the value of the resulting construction. If the result is not a construction, the result is `undefined`.

## New Construction

An object is a _new-construction node_ iff

- it has the property `node_new_construction_constructor ·· %constructor%` and
- it has the property `node_new_construction_value ·· %value%`.

It evaluates both children and returns the construction `%constructor% ·· %value%`.

## New Set

An object is a _new-set node_ iff it has at least one property `node_new_set_item ·· %value%`. This property is repeatable: it may occur any positive number of times.

It evaluates every `%value%` child exactly once and returns the canonical set of their results. The items may be atoms, composites, or constructions. If several children evaluate to equal objects, that object appears only once, because the result is a set. A child that evaluates to `undefined` contributes the atom `undefined` as an ordinary item.

The node recognition rule that a required property must have exactly one value does not apply to `node_new_set_item`; having zero values means the object is not a new-set node.

### Correctness and evaluation order

The reference evaluation specifies only the set of child results, not an order in which the children are evaluated. The implementation evaluates the children in a concrete order.

For a well-formed program this order cannot be observed: every child terminates and the result is the set of their results, independent of the order. However, if the program is not well-formed—for example if a child does not terminate, or if a child reads a value whose evaluation depends on the order in which children are visited—then the implementation may diverge on one child before reaching another, or may produce a result that depends on the chosen order. As with [fold](#correctness-and-associativity), the evaluator cannot check well-formedness, so it remains the programmer's responsibility.

## New Composite

An object is a _new-composite node_ iff it has at least one property `node_new_composite_property ·· %value%`. Like `node_new_set_item`, this property may occur any positive number of times.

It evaluates every `%value%` child exactly once and returns the canonical composite whose members are their results. The results become members of the composite directly; unlike a [new set](#new-set), they are not wrapped in `contains` constructions. If several children evaluate to equal objects, that object appears only once, because the result is a canonical composite. A child that evaluates to `undefined` contributes the atom `undefined` as an ordinary member.

The result is a composite in the open set model. It therefore has no set items unless a child result is itself a construction `contains ·· %item%`; use [Properties Of](#properties-of) to make its members explicit as items.

The node recognition rule that a required property must have exactly one value does not apply to `node_new_composite_property`; having zero values means the object is not a new-composite node.

### Correctness and evaluation order

The reference evaluation specifies only the composite formed by the child results, not an order in which the children are evaluated. The implementation evaluates the children in a concrete order.

For a well-formed program this order cannot be observed: every child terminates and the result is the composite of their results, independent of the order. However, if the program is not well-formed—for example if a child does not terminate, or if a child reads a value whose evaluation depends on the order in which children are visited—then the implementation may diverge on one child before reaching another, or may produce a result that depends on the chosen order. As with [fold](#correctness-and-associativity), the evaluator cannot check well-formedness, so it remains the programmer's responsibility.

## Query Atoms With Property

An object is a _query-atoms-with-property node_ iff it has the property `node_atoms_with_property ·· %property%`.

It evaluates `%property%` and returns the set of all atoms that have the result as a property in the knowledge.

## Query Atoms With Constructor

An object is a _query-atoms-with-constructor node_ iff it has the property `node_atoms_with_constructor ·· %constructor%`.

It evaluates `%constructor%` and returns the set of all atoms that have at least one property that is a construction whose constructor equals the evaluated `%constructor%`. Each matching atom appears once, regardless of how many matching properties it has.

## Query Construction Values

An object is a _query-construction-values node_ iff

- it has the property `node_construction_values_object ·· %object%` and
- it has the property `node_construction_values_property_constructor ·· %constructor%`.

It evaluates `%object%` and `%constructor%`. If `%object%` is a construction, the result is `undefined`. Otherwise the result is the set of all values `%v%` for which the values of `%object%` contain a construction `%c% ·· %v%` whose constructor `%c%` equals the evaluated `%constructor%`. The values of an object are its values in the knowledge if it is an atom and its members if it is a composite.

## Query Constructions

An object is a _query-constructions node_ iff it has the property `node_query_constructions ·· %constructor%`.

It evaluates `%constructor%` and returns the set of all pairs `(%a%, %v%)` such that the knowledge contains the statement `%a% is %constructor% ·· %v%`. Each pair is represented as the object `⟨atom ·· %a%, value ·· %v%⟩`, where `atom` and `value` are atoms that are property constructors in the base knowledge.

## Examples

The following examples use decimal integers for readability. As shown in [Children Are Nodes](#children-are-nodes), data in a child position is wrapped in `node_literal`.

### Addition

```
⟨
    node_add_first ·· ⟨node_literal ·· 1⟩,
    node_add_second ·· ⟨node_literal ·· 2⟩
⟩
```

evaluates to `3`.

### Identity Function and Call

The identity function is:

```
⟨node_function ·· ⟨node_parameter ·· 0⟩⟩
```

Calling it with `42`:

```
⟨
    node_call_callee ·· ⟨node_function ·· ⟨node_parameter ·· 0⟩⟩,
    node_call_with ·· ⟨node_literal ·· 42⟩
⟩
```

evaluates to `42`.

### Set Map

Mapping `x + 1` over `{1, 2, 3}`:

```
⟨
    node_set_map_set ·· ⟨node_literal ·· {1, 2, 3}⟩,
    node_set_map_mapper ·· ⟨
        node_function ·· ⟨
            node_add_first ·· ⟨node_parameter ·· 0⟩,
            node_add_second ·· ⟨node_literal ·· 1⟩
        ⟩
    ⟩
⟩
```

evaluates to the set `{2, 3, 4}`.

### New Set

Constructing `{1, 2}` from two child expressions:

```
⟨
    node_new_set_item ·· ⟨node_literal ·· 1⟩,
    node_new_set_item ·· ⟨node_literal ·· 2⟩
⟩
```

evaluates to the set `{1, 2}`.

### New Composite

Constructing the composite `⟨1, 2⟩` from two child expressions:

```
⟨
    node_new_composite_property ·· ⟨node_literal ·· 1⟩,
    node_new_composite_property ·· ⟨node_literal ·· 2⟩
⟩
```

evaluates to the composite `⟨1, 2⟩`. Unlike the [new-set example](#new-set), the members are inserted directly and are not wrapped in `contains` constructions.

### Conditional

```
⟨
    node_if_condition ·· ⟨node_literal ·· false⟩,
    node_if_then ·· ⟨node_literal ·· 1⟩,
    node_if_else ·· ⟨node_literal ·· 2⟩
⟩
```

evaluates to `2`.

## Evaluation Strategy

This section describes implementation choices. They do not affect the result of a well-formed program (see [Reference Evaluation](#reference-evaluation)), but they determine how much work is done and can matter for programs that are not well-formed.

The implementation is demand-driven:

- [Union](#set-union), [intersection](#set-intersection), [difference](#set-difference), [map](#set-map), [filter](#set-filter), [properties of](#properties-of), and the query nodes build a description of a set without iterating their operands. The items are computed when an operation needs them: by [count](#set-count), [unwrap](#set-unwrap), [equality](#equality), [new construction](#new-construction), when a value is required as an object or a node, or when an operation visits the items, such as [fold](#set-fold), [every](#set-every), [any](#set-any), [map](#set-map), or [filter](#set-filter).
- A call does not evaluate its argument. The argument is closed over the caller's context and kept as an unevaluated thunk. It is evaluated when the parameter is first referenced, and the result is cached for later references (call-by-need) and for code that captured the parameter. Items that are already evaluated when a function is called internally, such as the items passed by [map](#set-map), [filter](#set-filter), [every](#set-every), [any](#set-any), and [fold](#set-fold), are kept as values instead of being wrapped as thunks.
- Because arguments are pure, sharing the result of an argument evaluation does not change the result, only how much work is done.
- [Has value](#has-value) stops as soon as the answer is known. For example, the has-value result of a [map](#set-map) only depends on whether the source set has an item, so the mapper is not called.
- [Has property](#has-property) evaluates the subject first. If the subject is a construction, the result is `undefined` and the property child is never evaluated.
- A [new set](#new-set) evaluates all of its children and materialises the resulting set. Its items are therefore not represented lazily.
- A [new composite](#new-composite) evaluates all of its children and materialises the resulting composite. The child results become its members directly; they are not wrapped as `contains` items.
- Captured functions are cached as parsed nodes instead of being materialised as their object form. This avoids reparsing on call. Parsed nodes retain the object they were parsed from, so materialising a captured function reproduces its body exactly, including annotations and unknown properties. Atom-defined functions materialise into composites; see [Functions](#functions).
- Capturing an unevaluated argument shares its thunk in the function body instead of evaluating it or copying its closed expression. A thunk contains a finite closed expression and an optional cached result, so function objects remain finite even when they capture divergent code. Capturing an argument that is already evaluated escapes its value into a literal.

Because evaluation is demand-driven, a program that is not well-formed may still terminate here, for example when testing whether a mapped set is non-empty without applying a divergent mapper. This is not guaranteed.

Where an operation needs a value as an object or as a node, lazily represented values are materialised first. This can force a set that would otherwise not be iterated, such as a query operand or a value that is parsed as a node.

The implementation uses a concrete iteration order internally; a well-formed program cannot observe which one.

## Implementation Status

| Node kind                    | Encoding                                                                           | Status                               |
| ---------------------------- | ---------------------------------------------------------------------------------- | ------------------------------------ |
| Function                     | `node_function`                                                                    | implemented                          |
| Literal                      | `node_literal`                                                                     | implemented                          |
| Parameter                    | `node_parameter`                                                                   | implemented                          |
| Function self-reference      | `node_function_self`                                                               | implemented                          |
| Call                         | `node_call_callee`, `node_call_with`                                               | implemented                          |
| And                          | `node_and_first`, `node_and_second`                                                | implemented                          |
| Or                           | `node_or_first`, `node_or_second`                                                  | implemented                          |
| XOr                          | `node_xor_first`, `node_xor_second`                                                | implemented                          |
| Not                          | `node_not`                                                                         | implemented                          |
| Equal                        | `node_equal_first`, `node_equal_second`                                            | implemented for all values           |
| Less                         | `node_less_first`, `node_less_second`                                              | implemented for integers             |
| Add                          | `node_add_first`, `node_add_second`                                                | implemented for integers             |
| Multiply                     | `node_multiply_first`, `node_multiply_second`                                      | implemented for integers             |
| Set count                    | `node_set_count`                                                                   | implemented                          |
| Properties of                | `node_properties_of`                                                               | implemented                          |
| Has value                    | `node_has_value`                                                                   | implemented                          |
| Has property                 | `node_has_property_subject`, `node_has_property_property`                         | implemented                          |
| Set union                    | `node_set_union_first`, `node_set_union_second`                                     | implemented                          |
| Set intersection             | `node_set_intersection_first`, `node_set_intersection_second`                       | implemented                          |
| Set difference               | `node_set_difference_first`, `node_set_difference_second`                           | implemented                          |
| Set filter                   | `node_set_filter_set`, `node_set_filter_filter`                                     | implemented                          |
| Set map                      | `node_set_map_set`, `node_set_map_mapper`                                           | implemented                          |
| Set every                    | `node_set_every_set`, `node_set_every_predicate`                                    | implemented; expressible with `set fold` |
| Set any                      | `node_set_any_set`, `node_set_any_predicate`                                        | implemented; expressible with `set fold` |
| If                           | `node_if_condition`, `node_if_then`, `node_if_else`                                | implemented                          |
| Set unwrap                   | `node_set_unwrap_set`                                                              | implemented                          |
| Is atom                      | `node_is_atom`                                                                     | implemented                          |
| Is construction              | `node_is_construction`                                                             | implemented                          |
| Is composite                 | `node_is_composite`                                                                | implemented                          |
| Construction constructor     | `node_constructor_of_construction`                                                 | implemented                          |
| Construction value           | `node_value_of_construction`                                                       | implemented                          |
| New construction             | `node_new_construction_constructor`, `node_new_construction_value`                 | implemented                          |
| New set                      | `node_new_set_item` (one or more)                                                  | implemented                          |
| New composite                | `node_new_composite_property` (one or more)                                        | implemented                          |
| Set contains                 | `node_set_contains_element`, `node_set_contains_set`                               | implemented                          |
| Query atoms with property    | `node_atoms_with_property`                                                         | implemented                          |
| Query atoms with constructor | `node_atoms_with_constructor`                                                      | implemented                          |
| Query construction values    | `node_construction_values_object`, `node_construction_values_property_constructor` | implemented                          |
| Query constructions          | `node_query_constructions`                                                         | implemented                          |
| Set fold                     | `node_set_fold_set`, `node_set_fold_folder`, `node_set_fold_initial`               | implemented                          |

## Current Limitations

- Well-formedness is not checked by the evaluator. Termination, independence from iteration order, and validity of the materialised objects are semantic preconditions, so a program that is not well-formed may appear to observe laziness, iteration order, or missing validation.
- [New composite](#new-composite) inserts its child results without validating them as properties of the resulting composite. Object and property validation is a semantic precondition, as for other constructed values.
- Set-taking nodes accept every atom and every composite, as the set model is open. Constructions are rejected with `undefined`; use `node_properties_of` to pass an object's properties as a set.
