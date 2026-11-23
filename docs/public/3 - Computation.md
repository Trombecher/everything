# Chapter 3 - Computation

Everything has a computation system where functions and expressions can be evaluated. Expressions are modelled in Everything as objects. The whole system is pure and based on lambda calculus with additional node kinds.

Evaluation is knowledge-relative. Node recognition, atom property lookup, queries, and calling atom-defined functions all consult the knowledge that is supplied to the evaluator. A subset of the system can be evaluated in an empty knowledge, but anything that reads properties or queries atoms needs a knowledge context.

This chapter describes the implemented model. It distinguishes the reference evaluation, which defines the meaning of a program, from the implementation's evaluation strategy.

## Node Objects and Recognition

An object is a valid _node_ iff it matches exactly one node kind described in this chapter and every child position parses as a node. Node matching is shallow and structural:

- A node object is an atom or a set. The required properties are read from the atom's values in the knowledge or, for a set, from the set's members. A bare construction cannot be a node.
- For every required property of a node kind, the object must have exactly one matching construction. For example, an add node needs exactly one `node_add_first` value and exactly one `node_add_second` value. A required property with zero or several values means the kind does not match.
- If the object matches no node kind, it is not a node. If it matches more than one kind, it is also not a valid node; node kinds may not be combined.
- Additional properties that are not part of the recognized kind are ignored. This makes node definitions _open_: annotations and other metadata may be attached without changing the node kind. The `node_*` constructors are reserved and must not be used for annotations, because they can change the recognized kind.

Recognition only examines the object's own top-level properties (the atom's statements or the set's members). It does not descend into values; that happens during child parsing below.

### Children Are Nodes

Every child position of a node is itself parsed as a node, recursively. There is no implicit literal fallback:

- Every child must itself parse as a node. If any child fails, the whole object is not a valid node.
- To use plain data as a child, it must be wrapped in a literal node: `{node_literal <object>}`.

The object encoding produced by the implementation therefore always wraps literals explicitly. For example, `1 + 2` is encoded as:

```
{
    node_add_first {node_literal 1},
    node_add_second {node_literal 2}
}
```

Node recognition is not itself an evaluator operation. The evaluator takes an already parsed node together with a knowledge. Conversion from an object to a node can fail; conversion from a node back to an object produces a set of construction properties and does not preserve unknown properties or annotations. It also normalizes atom-defined nodes into sets, losing the atom's identity.

## Evaluation Results

Evaluation maps a node and a knowledge to a _result_. A result is:

- an atom,
- a construction,
- a set,
- a function value, which is a captured node that is kept as code rather than evaluated further.

Sets do not distinguish duplicates and have no intrinsic order. The atom `undefined` is a normal atom and may appear anywhere an atom may appear.

A result is a _set result_ if it is a set or a function value. Atoms and constructions are not sets. Nodes that take a set operand return `undefined` for other results; use [Values Of](#values-of) to make an atom's values explicit. A function value is a node object, and its values are the properties of its node representation.

`undefined` is produced when the input of an operation does not have the required shape, for example when arithmetic receives a non-integer, when a call's callee is not a function, or when a parameter or function self-reference is out of range. Most operations propagate `undefined`.

## Reference Evaluation

The meaning of a node is defined by a _reference evaluation_ that builds every set eagerly. A program (a node together with a knowledge) is _well-formed_ if the reference evaluation terminates. Its result may be the atom `undefined`; producing `undefined` is an ordinary, specified result.

Set iteration order is not specified. Different implementations or versions may visit elements in a different order, so programs whose result depends on it are not portable. This affects [fold](#fold), [every](#every), and [any](#any) when their function is not order-insensitive; see [Correctness and associativity](#correctness-and-associativity).

The implementation uses demand-driven evaluation instead; see [Evaluation Strategy](#evaluation-strategy). Apart from iteration order and the implementation gaps listed under [Current Limitations](#current-limitations), it produces the reference result for every well-formed program. Laziness changes how much work is done, not the result.

If the reference evaluation does not terminate, the program is not well-formed and has no specified result. The implementation may diverge, or it may happen to terminate because a divergent subcomputation was never demanded. This is not guaranteed and may change between implementations or versions. It is unrelated to the atom `undefined`, which is an ordinary value.

## Truthiness

Truthiness is deliberately narrow:

- An object is _truthy_ iff it is equal to `true`.
- An object is _falsy_ iff it is equal to `false`.
- Every other object, including sets, constructions, and `undefined`, has _undefined truthiness_.

There is no general "non-empty means true" coercion in the language. Boolean operations therefore produce only `true`, `false`, or `undefined`; they do not treat a non-empty set as true. A function value also has undefined truthiness because it is kept as code rather than evaluated.

## Functions

An object is a _function_ iff it has the property `node_function <body>`. The associated value `<body>` is the function body. Functions have exactly one parameter. Multiple arguments are encoded by currying: a function returns another function that takes the next argument.

Evaluating a function does not evaluate its body. It produces a function value: a copy of the function node in which free references to enclosing functions have been captured (see [Captures](#captures)). The body is evaluated only when the function is called.

## Parameter References

An object is a _parameter reference_ iff it has the property `node_parameter <depth>`. `<depth>` is a non-negative integer that must fit into an unsigned 32-bit integer. It references the runtime value of the parameter of an enclosing function:

- `0` is the innermost enclosing function,
- `1` is the next enclosing function, and so on.

If the depth does not identify an enclosing function at evaluation time, the parameter reference evaluates to `undefined`.

## Function Self-References

An object is a (function) _self-reference_ iff it has the property `node_function_self <depth>`. `<depth>` has the same meaning and restrictions as for parameter references. These nodes support recursive functions without a fix-point operator.

A self-reference evaluates to the enclosing function value itself, not to the result of calling it. The function value can then be called with a normal call node. An out-of-range depth evaluates to `undefined`.

## Captures

Functions are closures. When a function is evaluated inside an enclosing function call, every free parameter reference and free function self-reference in its body is replaced by a literal holding the current value or function. References to functions and parameters bound inside the function remain unchanged.

Capturing a parameter value escapes it into a literal, so a value that happens to look like a node is not reinterpreted as code when the function is later called. This is also why functions can escape the call that created them.

Evaluation of a function does not capture runtime parameter values from functions that have not been called yet; capture happens when the function value is produced. Nested functions therefore capture the environment of the call in which they are evaluated.

## Function Application

An object is a _function application/call_ iff

- it has the property `node_call_callee <callee>` and
- it has the property `node_call_with <value>`.

A call evaluates the argument `<value>` and then the callee `<callee>`. It then interprets the callee result as a function:

- if the callee result is a function value, it is used directly;
- otherwise the callee result is parsed as a node in the current knowledge, and it must be a `node_function` node;
- if the callee is not a function, or the callee object is not a valid node, the call evaluates to `undefined`.

The argument is bound to the function's parameter and the body is evaluated with the function context pushed. The context is popped when the body finishes.

## Literals

An object is a _literal_ iff it has the property `node_literal <object>`. A literal evaluates to `<object>` without interpreting or evaluating it. This prevents a value from being treated as a node. Literals are also the boundary that capture uses when it inserts runtime values into code.

## Logical And

An object is an _and node_ iff

- it has the property `node_and_first <left>` and
- it has the property `node_and_second <right>`.

It evaluates both children and combines their truthiness:

- if `<left>` is `false`, the result is `false`;
- if `<left>` has undefined truthiness, the result is `undefined`;
- if `<left>` is `true`, the result is `true` if `<right>` is truthy, `false` if it is falsy, and `undefined` if its truthiness is undefined.

An and node therefore never returns the value of `<right>` itself.

## Logical Or

An object is an _or node_ iff

- it has the property `node_or_first <left>` and
- it has the property `node_or_second <right>`.

It evaluates both children and combines their truthiness:

- if `<left>` is `true`, the result is `true`;
- if `<left>` has undefined truthiness, the result is `undefined`;
- if `<left>` is `false`, the result is `true` if `<right>` is truthy, `false` if it is falsy, and `undefined` if its truthiness is undefined.

An or node therefore never returns the value of `<right>` itself.

## Logical XOr

An object is an _xor node_ iff

- it has the property `node_xor_first <left>` and
- it has the property `node_xor_second <right>`.

It evaluates both children. The result is `true` iff exactly one of the two results is truthy, and `false` iff both are truthy or both are falsy. If either result has undefined truthiness, the result is `undefined`.

## Logical Not

An object is a _not node_ iff it has the property `node_not <inner>`. It evaluates `<inner>` and returns the opposite boolean. If `<inner>` has undefined truthiness, the result is `undefined`.

## Equality

An object is an _equal node_ iff

- it has the property `node_equal_first <left>` and
- it has the property `node_equal_second <right>`.

It evaluates both children. The result is `true` iff the two results are structurally equal as objects.

## Less Than

An object is a _less-than node_ iff

- it has the property `node_less_first <left>` and
- it has the property `node_less_second <right>`.

It evaluates both children. If both results are integers, the result is `true` iff `<left> < <right>`, and `false` otherwise. If either result is not an integer, the result is `undefined`. There is no ordering for sets, constructions, or other atoms.

## Addition

An object is an _add node_ iff

- it has the property `node_add_first <left>` and
- it has the property `node_add_second <right>`.

It evaluates both children. If both results are integers, the result is their sum. If either result is not an integer, the result is `undefined`.

Integers are arbitrary precision, so addition cannot overflow.

## Multiplication

An object is a _multiply node_ iff

- it has the property `node_multiply_first <left>` and
- it has the property `node_multiply_second <right>`.

It evaluates both children. If both results are integers, the result is their product. If either result is not an integer, the result is `undefined`.

As with addition, integers are arbitrary precision and cannot overflow.

## Set Count

An object is a _count node_ iff it has the property `node_count <inner>`. It evaluates `<inner>` and, if the result is a set, returns its number of elements:

- for a set, the number of members;
- for the values of an atom, the number of values;
- for a function value, the number of properties of its node representation, which is `1` for a function.

If the result is an atom or a construction, the result is `undefined`. Use [Values Of](#values-of) to count the values of an atom: `count(values_of(<atom>))`.

## Values Of

An object is a _values-of node_ iff it has the property `node_values_of <inner>`. It evaluates `<inner>` and converts the result to the set of its values:

- for a set or function value, the result itself;
- for an atom, the set of values (properties) the atom has in the knowledge;
- for a construction, `undefined`.

An atom without statements evaluates to the empty set. In contrast to [Set Count](#set-count), the conversion from an object to its values is explicit, so counting the values of an atom is written as `count(values_of(<atom>))`.

## Has Value

An object is a _has-value node_ iff it has the property `node_has_value <inner>`. It evaluates `<inner>` and returns `true` iff the result has at least one value:

- for a set, whether it has at least one member;
- for an atom, whether it has at least one value in the knowledge;
- for a construction, `undefined`;
- for a function value, `true`.

This is the existential counterpart of [Set Count](#set-count). It is equivalent to `count(values_of(<inner>)) > 0`.

## Set Union

An object is a _union node_ iff

- it has the property `node_union_first <left>` and
- it has the property `node_union_second <right>`.

It evaluates both children. If either result is not a set, the result is `undefined`. Otherwise the result is the union of the two sets.

## Set Intersection

An object is an _intersection node_ iff

- it has the property `node_intersection_first <left>` and
- it has the property `node_intersection_second <right>`.

It evaluates both children. If either result is not a set, the result is `undefined`. Otherwise the result is the intersection of the two sets.

## Set Difference

An object is a _difference node_ iff

- it has the property `node_difference_first <left>` and
- it has the property `node_difference_second <right>`.

It evaluates both children. If the left result is the empty set, the result is the empty set. Otherwise, if either result is not a set, the result is `undefined`; if both are sets, the result is the set of elements of the left result that are not elements of the right result.

Node recognition requires both children to parse as nodes.

## Filter

An object is a _filter node_ iff

- it has the property `node_filter_set <set>` and
- it has the property `node_filter_filter <filter>`.

It evaluates `<set>` and `<filter>`. If `<set>` is not a set, or if `<filter>` is not a function, the result is `undefined`. Otherwise the result is the set of elements of `<set>` for which the call of `<filter>` with the element as a literal argument evaluates to exactly `true`. Results that are `false` or have undefined truthiness are excluded.

## Map

An object is a _map node_ iff

- it has the property `node_map_set <set>` and
- it has the property `node_map_mapper <mapper>`.

It evaluates `<set>` and `<mapper>`. If `<set>` is not a set, or if `<mapper>` is not a function, the result is `undefined`. Otherwise the result is the set of results of calling `<mapper>` with each element of `<set>` as a literal argument. Because the result is a set, equal mapped values are not distinguished.

## Every

An object is an _every node_ iff

- it has the property `node_every_set <set>` and
- it has the property `node_every_predicate <predicate>`.

It evaluates `<set>` and `<predicate>`:

1. If `<set>` is not a set, the result is `undefined`.
2. If `<set>` has no elements, the result is `true`; the predicate is not used.
3. If `<predicate>` is not a function, the result is `undefined`.
4. Otherwise the elements are visited in the implementation's iteration order. Each element `<o>` is passed to `<predicate>` as a literal argument via `{node_call_callee <predicate>, node_call_with <o>}`. The first call whose result is not `true` determines the outcome: `false` yields `false`, and anything else yields `undefined`.
5. If every call yields `true`, the result is `true`.

`every` is the short-circuiting fold

```text
fold(<set>, x |-> y |-> x and (predicate y), true)
```

and therefore inherits the [correctness caveat for folds](#correctness-and-associativity). It is independent of iteration order only if `<predicate>` returns `true` or `false` for every element. Otherwise a `false` before an undefined result yields `false`, while an undefined result before a `false` yields `undefined`.

## Any

An object is an _any node_ iff

- it has the property `node_any_set <set>` and
- it has the property `node_any_predicate <predicate>`.

It evaluates `<set>` and `<predicate>`:

1. If `<set>` is not a set, the result is `undefined`.
2. If `<set>` has no elements, the result is `false`; the predicate is not used.
3. If `<predicate>` is not a function, the result is `undefined`.
4. Otherwise the elements are visited in the implementation's iteration order. Each element `<o>` is passed to `<predicate>` as a literal argument via `{node_call_callee <predicate>, node_call_with <o>}`. The first call whose result is not `false` determines the outcome: `true` yields `true`, and anything else yields `undefined`.
5. If every call yields `false`, the result is `false`.

`any` is the short-circuiting fold

```text
fold(<set>, x |-> y |-> x or (predicate y), false)
```

and therefore inherits the [correctness caveat for folds](#correctness-and-associativity). It is independent of iteration order only if `<predicate>` returns `true` or `false` for every element. Otherwise a `true` before an undefined result yields `true`, while an undefined result before a `true` yields `undefined`.

Both `every` and `any` can be expressed with `fold`; whether they will remain primitive nodes is still undecided.

## Fold

An object is a _fold node_ iff

- it has the property `node_fold_set <set>`,
- it has the property `node_fold_folder <folder>`, and
- it has the property `node_fold_initial <initial>`.

It evaluates `<set>`, `<folder>`, and `<initial>`:

1. If `<set>` is not a set, the result is `undefined`.
2. If `<set>` has no elements, the result is the value of `<initial>`; the folder is not used.
3. If `<folder>` is not a function, the result is `undefined`.
4. Otherwise the accumulator starts as the value of `<initial>`. For each element `<o>` of `<set>` in iteration order, apply `<folder>` pairwise: the first call receives the previous accumulator and returns a function, which is then called with `<o>`. The result of that call becomes the new accumulator.
5. The final accumulator is the result.

In contrast to [Map](#map) and [Filter](#filter), a fold returns the final accumulator rather than a set. The accumulator may itself be a set. If a folder application does not return a function, the result is `undefined`; remaining elements do not change that.

In object form, each step is:

```
{
    node_call_callee {
        node_call_callee <folder> {node_literal <accumulator>}
    },
    node_call_with {node_literal <element>}
}
```

### Correctness and associativity

For a fold to be independent of the order in which the set is traversed, its folding operation must be **associative**: for all values `x`, `y`, and `z`,

```text
(x ⊕ y) ⊕ z = x ⊕ (y ⊕ z)
```

Only then does folding a set yield the same result for every order and grouping of its elements. Otherwise the result is determined by the set's iteration order, which is an implementation detail.

The `and` node is not associative on all inputs. For example:

```text
false and undefined = false
undefined and false = undefined
```

Consequently, `and` is associative only when every operand has truthiness `true` or `false`; in particular, when no folded value is `undefined` or another non-boolean value. Equivalently, the mapper or folder must never produce `undefined` truthiness for an element.

Whether a particular folder or mapper satisfies this property depends on the behaviour of arbitrary nodes. Since the computation system is Turing complete, the property is undecidable in general: no algorithm can decide it for all nodes, and the evaluator cannot check or guarantee it. Correctness of a `fold` is therefore the programmer's responsibility. Without a proof of associativity, the evaluator can only produce the result for the chosen iteration order.

## Contains

An object is a _contains node_ iff

- it has the property `node_contains_element <element>` and
- it has the property `node_contains_set <set>`.

It evaluates both children and returns `true` iff `<element>` is a member of `<set>`. If `<set>` is not a set, the result is `undefined`.

## Conditional Nodes

An object is an _if node_ iff

- it has the property `node_if_condition <condition>`,
- it has the property `node_if_then <then>`, and
- it has the property `node_if_else <else>`.

It evaluates all three children. If `<condition>` is truthy, the result is `<then>`; if it is falsy, the result is `<else>`; if it has undefined truthiness, the result is `undefined`.

## Unwrap

An object is an _unwrap node_ iff it has the property `node_unwrap_set <set>`. It evaluates `<set>`. If the result is not a set, the result is `undefined`. Otherwise, if the result has exactly one element, that element is returned; if it has zero elements or more than one element, the result is `undefined`.

There is no default branch in the current implementation.

## Is Atom

An object is an _is-atom node_ iff it has the property `node_is_atom <inner>`. It evaluates `<inner>`. The result is `true` iff the result is an atom. Sets and function values are not atoms and yield `false`.

## Is Construction

An object is an _is-construction node_ iff it has the property `node_is_construction <inner>`. It evaluates `<inner>`. The result is `true` iff the result is a construction. Sets and function values yield `false`.

## Is Set

An object is an _is-set node_ iff it has the property `node_is_set <inner>`. It evaluates `<inner>`. The result is `true` iff the result is a set or a function value. Atoms and constructions are not sets.

A function value is a node object and therefore a set in its own right; its elements are the properties of its node representation.

## Construction Constructor and Value

An object is a _construction-constructor node_ iff it has the property `node_constructor_of_construction <construction>`. It evaluates `<construction>` and returns the constructor of the resulting construction. If the result is not a construction, the result is `undefined`.

An object is a _construction-value node_ iff it has the property `node_value_of_construction <construction>`. It evaluates `<construction>` and returns the value of the resulting construction. If the result is not a construction, the result is `undefined`.

## New Construction

An object is a _new-construction node_ iff

- it has the property `node_new_construction_constructor <constructor>` and
- it has the property `node_new_construction_value <value>`.

It evaluates both children and returns the construction `<constructor> <value>`.

## Query Atoms With Property

An object is a _query-atoms-with-property node_ iff it has the property `node_atoms_with_property <property>`.

It evaluates `<property>` and returns the set of all atoms that have the result as a property in the knowledge.

## Query Atoms With Constructor

An object is a _query-atoms-with-constructor node_ iff it has the property `node_atoms_with_constructor <constructor>`.

It evaluates `<constructor>` and returns the set of all atoms that have at least one property that is a construction whose constructor equals the evaluated `<constructor>`. Each matching atom appears once, regardless of how many matching properties it has.

## Query Construction Values

An object is a _query-construction-values node_ iff

- it has the property `node_construction_values_object <object>` and
- it has the property `node_construction_values_property_constructor <constructor>`.

It evaluates `<object>` and `<constructor>`. If `<object>` is a construction, the result is `undefined`. Otherwise the result is the set of all values `v` for which the values of `<object>` contain a construction `c v` whose constructor `c` equals the evaluated `<constructor>`. The values of an object are its values in the knowledge if it is an atom and its members if it is a set.

## Query Constructions

An object is a _query-constructions node_ iff it has the property `node_query_constructions <constructor>`.

It evaluates `<constructor>` and returns the set of all pairs `(a, v)` such that the knowledge contains the statement `a is <constructor> v`. Each pair is represented as the object `{atom a, value v}`, where `atom` and `value` are atoms that are property constructors in the base knowledge.

## Examples

The following examples use decimal integers for readability. As shown in [Children Are Nodes](#children-are-nodes), data in a child position is wrapped in `node_literal`.

### Addition

```
{
    node_add_first {node_literal 1},
    node_add_second {node_literal 2}
}
```

evaluates to `3`.

### Identity Function and Call

The identity function is:

```
{node_function {node_parameter 0}}
```

Calling it with `42`:

```
{
    node_call_callee {node_function {node_parameter 0}},
    node_call_with {node_literal 42}
}
```

evaluates to `42`.

### Map

Mapping `x + 1` over `{1, 2, 3}`:

```
{
    node_map_set {node_literal {1, 2, 3}},
    node_map_mapper {
        node_function {
            node_add_first {node_parameter 0},
            node_add_second {node_literal 1}
        }
    }
}
```

evaluates to the set `{2, 3, 4}`.

### Conditional

```
{
    node_if_condition {node_literal false},
    node_if_then {node_literal 1},
    node_if_else {node_literal 2}
}
```

evaluates to `2`.

## Evaluation Strategy

This section describes implementation choices. With the exception of iteration order, they do not affect the result of a well-formed program (see [Reference Evaluation](#reference-evaluation)), but they determine how much work is done and can matter for programs whose evaluation does not terminate.

The implementation is demand-driven:

- [Union](#set-union), [intersection](#set-intersection), [difference](#set-difference), [map](#map), [filter](#filter), and the query nodes build a description of a set without iterating their operands. The members are computed when an operation needs them: by [count](#set-count), [unwrap](#unwrap), [equality](#equality), [new construction](#new-construction), or an operation that visits the elements, such as [fold](#fold), [every](#every), [any](#any), [map](#map), or [filter](#filter).
- [Has value](#has-value), [every](#every), and [any](#any) stop as soon as the answer is known. For example, the has-value result of a [map](#map) only depends on whether the source set has an element, so the mapper is not called.
- [And](#logical-and) and [or](#logical-or) do not evaluate `<right>` when `<left>` already determines the result. [If](#conditional-nodes) evaluates only the selected branch. [Difference](#set-difference) with an empty left result does not evaluate `<right>`.
- Iteration order is not specified. It can be observed through [fold](#fold), [every](#every), and [any](#any) when their function is not order-insensitive; see [Correctness and associativity](#correctness-and-associativity).

Because evaluation is demand-driven, a program that does not terminate under the reference evaluation may still terminate here, for example when testing whether a mapped set is non-empty without applying a divergent mapper. This is not guaranteed.

Where an operation needs a value as an object or as a node, the current implementation can reject a lazily represented set that the reference evaluation would accept in its eager form. See [Current Limitations](#current-limitations).

## Implementation Status

| Node kind | Encoding | Status |
| --- | --- | --- |
| Function | `node_function` | implemented |
| Literal | `node_literal` | implemented |
| Parameter | `node_parameter` | implemented |
| Function self-reference | `node_function_self` | implemented |
| Call | `node_call_callee`, `node_call_with` | implemented |
| And | `node_and_first`, `node_and_second` | implemented |
| Or | `node_or_first`, `node_or_second` | implemented |
| XOr | `node_xor_first`, `node_xor_second` | implemented |
| Not | `node_not` | implemented |
| Equal | `node_equal_first`, `node_equal_second` | implemented for all values |
| Less | `node_less_first`, `node_less_second` | implemented for integers |
| Add | `node_add_first`, `node_add_second` | implemented for integers |
| Multiply | `node_multiply_first`, `node_multiply_second` | implemented for integers |
| Count | `node_count` | implemented for sets |
| Values of | `node_values_of` | implemented |
| Has value | `node_has_value` | implemented |
| Union | `node_union_first`, `node_union_second` | implemented; set operands only |
| Intersection | `node_intersection_first`, `node_intersection_second` | implemented; set operands only |
| Difference | `node_difference_first`, `node_difference_second` | implemented; set operands only |
| Filter | `node_filter_set`, `node_filter_filter` | implemented; set operand required |
| Map | `node_map_set`, `node_map_mapper` | implemented; set operand required |
| Every | `node_every_set`, `node_every_predicate` | implemented; expressible with `fold` |
| Any | `node_any_set`, `node_any_predicate` | implemented; expressible with `fold` |
| If | `node_if_condition`, `node_if_then`, `node_if_else` | implemented |
| Unwrap | `node_unwrap_set` | implemented for sets |
| Is atom | `node_is_atom` | implemented |
| Is construction | `node_is_construction` | implemented |
| Is set | `node_is_set` | implemented |
| Construction constructor | `node_constructor_of_construction` | implemented |
| Construction value | `node_value_of_construction` | implemented |
| New construction | `node_new_construction_constructor`, `node_new_construction_value` | implemented |
| Contains | `node_contains_element`, `node_contains_set` | implemented |
| Query atoms with property | `node_atoms_with_property` | implemented |
| Query atoms with constructor | `node_atoms_with_constructor` | implemented |
| Query construction values | `node_construction_values_object`, `node_construction_values_property_constructor` | implemented |
| Query constructions | `node_query_constructions` | implemented |
| Fold | `node_fold_set`, `node_fold_folder`, `node_fold_initial` | implemented |

## Current Limitations

- [Query atoms with property](#query-atoms-with-property) and [query construction values](#query-construction-values) currently reject a lazily represented operand with `undefined`, while an eagerly evaluated operand with the same value is accepted. This is an implementation gap: the reference evaluation does not distinguish them.
- Node parsing positions ([call](#function-application), [map](#map), [filter](#filter), [every](#every), [any](#any), [fold](#fold)) currently reject lazily represented values. An eagerly evaluated value that encodes a node is accepted. This is an implementation gap.
- Function values are iterated through the values of their node representation, which is materialised on demand. Because node conversion is lossy, annotations and unknown properties of the original object are not part of that set.
- Set-taking nodes require a set. Atoms and constructions are rejected with `undefined`; use `values_of` to pass an atom's values.
- Converting a node to an object is lossy: unknown properties and annotations are not preserved, and atom-defined nodes are normalized into sets.
