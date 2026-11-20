# Chapter 3 - Computation

Everything has a computation system where functions and expressions can be evaluated. Expressions are modelled in Everything as objects. The whole system is pure and based on lambda calculus with additional node kinds.

Evaluation is knowledge-relative. Node recognition, atom property lookup, queries, and calling atom-defined functions all consult the knowledge that is supplied to the evaluator. A subset of the system can be evaluated in an empty knowledge, but anything that reads properties or queries atoms needs a knowledge context.

This chapter describes the implemented model. Node kinds that are recognized but not yet evaluable are marked **[not implemented]** or **[partially implemented]**.

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

Evaluation maps a node and a knowledge to a _result_. A result is one of:

- an evaluated object,
- a function value, which is a captured node that is kept as code rather than evaluated further,
- a deferred set operation: a union, map, or filter,
- the atom `undefined`.

Deferred set operations are built without iterating their inputs. They are iterated when an operation needs their elements, such as `unwrap`, `filter`, `map`, membership tests, or explicit iteration. Not every operation supports deferred results yet; see [Current Limitations](#current-limitations).

A result is a _set_ if it is an evaluated set, the values of an atom produced by [Values Of](#values-of), a function value, or a deferred set operation. An evaluated atom or construction is not a set. Nodes that take a set operand return `undefined` for other results; use [Values Of](#values-of) to make an atom's values explicit. A function value is a node object, and its values are the properties of its node representation.

`undefined` is a first-class atom. It is produced when the input of an operation does not have the required shape, for example when arithmetic receives a non-integer, when a call's callee is not a function, or when a parameter or function self-reference is out of range. Most operations propagate `undefined`.

Evaluation is big-step and pure. Operands are evaluated before the node is applied unless the node explicitly short-circuits (`and`, `or`, `if`) or builds a deferred set operation (`union`, `map`, `filter`). The order in which operands are evaluated is fixed by the evaluator, but it is an implementation detail and should not be relied upon.

## Truthiness

Truthiness is deliberately narrow:

- An object is _truthy_ iff it is equal to `true`.
- An object is _falsy_ iff it is equal to `false`.
- Every other object, including sets, constructions, and `undefined`, has _undefined truthiness_.

There is no general "non-empty means true" coercion in the language. Boolean operations therefore produce only `true`, `false`, or `undefined`; they do not treat a non-empty set as true. A deferred result or a function value also has undefined truthiness because it is not yet an evaluated object.

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
- if the callee is `undefined`, is not a function, or the callee object is not a valid node, the call evaluates to `undefined`.

The argument is bound to the function's parameter and the body is evaluated with the function context pushed. The context is popped when the body finishes. The argument is evaluated but not forced: a deferred set argument stays deferred inside the function.

## Literals

An object is a _literal_ iff it has the property `node_literal <object>`. A literal evaluates to `<object>` without interpreting or evaluating it. This prevents a value from being treated as a node. Literals are also the boundary that capture uses when it inserts runtime values into code.

## Logical And

An object is an _and node_ iff

- it has the property `node_and_first <left>` and
- it has the property `node_and_second <right>`.

It first evaluates `<left>`:

- if `<left>` is `false`, the result is `false` and `<right>` is not evaluated;
- if `<left>` has undefined truthiness, the result is `undefined` and `<right>` is not evaluated;
- if `<left>` is `true`, it evaluates `<right>` and coerces its truthiness to a boolean. The result is `true`, `false`, or `undefined` accordingly.

An and node therefore never returns the value of `<right>` itself.

## Logical Or

An object is an _or node_ iff

- it has the property `node_or_first <left>` and
- it has the property `node_or_second <right>`.

It first evaluates `<left>`:

- if `<left>` is `true`, the result is `true` and `<right>` is not evaluated;
- if `<left>` has undefined truthiness, the result is `undefined` and `<right>` is not evaluated;
- if `<left>` is `false`, it evaluates `<right>` and coerces its truthiness to a boolean. The result is `true`, `false`, or `undefined` accordingly.

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

It evaluates both children. If either result is `undefined`, the result is `undefined`. Otherwise the result is `true` iff the two results are structurally equal as objects.

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

Integers are arbitrary precision, so addition cannot overflow. The atom `arithmetic_overflow` is reserved but is not produced by the current implementation.

## Multiplication

An object is a _multiply node_ iff

- it has the property `node_multiply_first <left>` and
- it has the property `node_multiply_second <right>`.

It evaluates both children. If both results are integers, the result is their product. If either result is not an integer, the result is `undefined`.

As with addition, integers are arbitrary precision and the current implementation never produces `arithmetic_overflow`.

## Set Count

An object is a _count node_ iff it has the property `node_count <inner>`. It evaluates `<inner>` and, if the result is a set, returns its number of elements:

- for an evaluated set, the number of members;
- for an atom's values, the number of values;
- for a function value, the number of properties of its node representation, which is `1` for a function;
- deferred set results (union, map, filter) are sets, but their size is not implemented yet.

If the result is an atom, a construction, or `undefined`, the result is `undefined`. Use [Values Of](#values-of) to count the values of an atom: `count(values_of(<atom>))`.

## Values Of

An object is a _values-of node_ iff it has the property `node_values_of <inner>`. It evaluates `<inner>` and converts the result to the set of its values:

- for a set or function value, the result itself, unchanged (this includes deferred set results);
- for an atom, a deferred set of the values (properties) the atom has in the knowledge;
- for a construction, `undefined`.

If `<inner>` evaluates to `undefined`, then since `undefined` is itself an atom, the result is the set of values of the atom `undefined` in the knowledge, which may be non-empty. An atom without statements evaluates to the empty set. The atom case is deferred: counting, testing, or iterating the result reads the knowledge values directly and does not materialise a specialised set object. In contrast to [Set Count](#set-count), the conversion from an object to its values is explicit, so counting the values of an atom is written as `count(values_of(<atom>))`.

## Has Value

An object is a _has-value node_ iff it has the property `node_has_value <inner>`. It evaluates `<inner>` and returns `true` iff the result has at least one value:

- for a set, whether it has at least one member (this includes deferred set results, which short-circuit where possible);
- for an atom, whether it has at least one value in the knowledge;
- for a construction, `undefined`;
- for a function value, `true`.

If the result is `undefined`, the result is `undefined`. This is the existential counterpart of [Set Count](#set-count). It is equivalent to `count(values_of(<inner>)) > 0`, but it does not materialise the values of an atom.

## Set Union

An object is a _union node_ iff

- it has the property `node_union_first <left>` and
- it has the property `node_union_second <right>`.

It evaluates both children. If either result is not a set, the result is `undefined`. Otherwise the result is a deferred set that represents the union of the two results.

Conceptually, the result contains every element of the left result and every element of the right result. Iteration currently yields the left elements followed by the right elements without re-checking for duplicates, so overlapping operands can yield an element more than once. Membership tests are correct. The size of a union is not implemented yet.

## Filter

An object is a _filter node_ iff

- it has the property `node_filter_set <set>` and
- it has the property `node_filter_filter <filter>`.

It evaluates `<set>` and `<filter>`. If `<set>` is not a set, or if `<filter>` is not a function, the result is `undefined`. Otherwise the result is a deferred set.

When the result is iterated, each element of `<set>` is passed to `<filter>` as a literal argument. An element is retained iff the call evaluates to exactly `true`. Results that are `false` or have undefined truthiness are dropped. The filter function is called lazily, during iteration, not when the filter node is evaluated.

## Map

An object is a _map node_ iff

- it has the property `node_map_set <set>` and
- it has the property `node_map_mapper <mapper>`.

It evaluates `<set>` and `<mapper>`. If `<set>` is not a set, or if `<mapper>` is not a function, the result is `undefined`. Otherwise the result is a deferred set.

When the result is iterated, each element of `<set>` is passed to `<mapper>` as a literal argument and replaced by the call's result. The mapper is called lazily, during iteration. The result is not deduplicated: if two elements map to the same object, both are yielded. The size of a map result is not implemented yet.

## Every

An object is an _every node_ iff

- it has the property `node_every_set <set>` and
- it has the property `node_every_predicate <predicate>`.

It evaluates `<set>` and `<predicate>`:

1. If `<set>` is not a set, the result is `undefined`.
2. If `<set>` has no elements, the result is `true`. The predicate is not called in this case, so it may even be `undefined` or not a function.
3. If `<predicate>` is not a function, the result is `undefined`.
4. Otherwise each element `<o>` of `<set>` is passed to `<predicate>` in iteration order as `{node_call_callee <predicate>, node_call_with <o>}`. The first call result determines the outcome:
   - `true` continues with the next element;
   - `false` yields `false` and stops;
   - anything else yields `undefined` and stops.
5. If every call is `true`, the result is `true`.

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
2. If `<set>` has no elements, the result is `false`. The predicate is not called in this case, so it may even be `undefined` or not a function.
3. If `<predicate>` is not a function, the result is `undefined`.
4. Otherwise each element `<o>` of `<set>` is passed to `<predicate>` in iteration order as `{node_call_callee <predicate>, node_call_with <o>}`. The first call result determines the outcome:
   - `false` continues with the next element;
   - `true` yields `true` and stops;
   - anything else yields `undefined` and stops.
5. If every call is `false`, the result is `false`.

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
2. If `<set>` has no elements, the result is the value of `<initial>`. The folder is not called in this case, so it may even be `undefined`.
3. If `<folder>` is not a function, the result is `undefined`.
4. Otherwise the accumulator starts as the value of `<initial>`. For each element `<o>` of `<set>` in iteration order, apply `<folder>` pairwise: the first call receives the previous accumulator and returns a function, which is then called with `<o>`. The result of that call becomes the new accumulator.
5. The final accumulator is the result.

In contrast to [Map](#map) and [Filter](#filter), a fold is evaluated eagerly, and its result is the final accumulator rather than a deferred set. The accumulator may itself be a deferred set. If a folder application does not return a function, that step and all following steps evaluate to `undefined`, but folding continues over the remaining elements.

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

**[not implemented]** The intended semantics are: evaluate `<element>` and `<set>` and return whether `<element>` is a member of `<set>`. If `<set>` is not a set, the result is `undefined`. Membership testing already exists internally for evaluated and deferred sets; only the node form is not evaluable yet.

## Conditional Nodes

An object is an _if node_ iff

- it has the property `node_if_condition <condition>`,
- it has the property `node_if_then <then>`, and
- it has the property `node_if_else <else>`.

First the if node evaluates `<condition>`. If the result is truthy, it evaluates `<then>` and returns that result. If the result is falsy, it evaluates `<else>` and returns that result. If the condition has undefined truthiness, the result is `undefined` and neither branch is evaluated.

## Unwrap

An object is an _unwrap node_ iff it has the property `node_unwrap_set <set>`. It evaluates `<set>`. If the result is not a set, the result is `undefined`. Otherwise it iterates the result: if the result has exactly one element, that element is returned; if it has zero elements or more than one element, the result is `undefined`.

There is no default branch in the current implementation.

## Is Atom

An object is an _is-atom node_ iff it has the property `node_is_atom <inner>`. It evaluates `<inner>`. If the result is `undefined`, the result is `undefined`. Otherwise the result is `true` iff the result is an evaluated atom. Deferred set values and function values are not atoms and yield `false`.

## Is Construction

An object is an _is-construction node_ iff it has the property `node_is_construction <inner>`. It evaluates `<inner>`. If the result is `undefined`, the result is `undefined`. Otherwise the result is `true` iff the result is an evaluated construction. Deferred set values and function values yield `false`.

## Is Set

An object is an _is-set node_ iff it has the property `node_is_set <inner>`. It evaluates `<inner>`. If the result is `undefined`, the result is `undefined`. Otherwise the result is `true` iff the result is a set: an evaluated set, a deferred set result, or a function value. Atoms and constructions are not sets.

A function value is a node object and therefore a set in its own right; its elements are the properties of its node representation.

## Construction Constructor and Value

An object is a _construction-constructor node_ iff it has the property `node_constructor_of_construction <construction>`. It evaluates `<construction>` and returns the constructor of the resulting construction. If the result is not an evaluated construction, the result is `undefined`.

An object is a _construction-value node_ iff it has the property `node_value_of_construction <construction>`. It evaluates `<construction>` and returns the value of the resulting construction. If the result is not an evaluated construction, the result is `undefined`.

## New Construction

An object is a _new-construction node_ iff

- it has the property `node_new_construction_constructor <constructor>` and
- it has the property `node_new_construction_value <value>`.

**[not implemented]** The intended semantics are: evaluate both children and return the construction `<constructor> <value>`.

## Query Atoms With Property

An object is a _query-atoms-with-property node_ iff it has the property `node_atoms_with_property <property>`.

It evaluates `<property>` and returns the set of all atoms that have the result as a property in the knowledge. The result is a deferred set: size and membership queries are answered by the knowledge directly, and iteration yields the matching atoms in knowledge order. If `<property>` evaluates to a deferred set, the result is `undefined`.

## Query Atoms With Constructor

An object is a _query-atoms-with-constructor node_ iff it has the property `node_atoms_with_constructor <constructor>`.

It evaluates `<constructor>` and returns the set of all atoms that have at least one property that is a construction whose constructor equals the evaluated `<constructor>`. Each matching atom is yielded once, regardless of how many matching properties it has. The result is a deferred set: iteration yields the matching atoms in knowledge order. If `<constructor>` evaluates to a deferred set, it is materialized by iterating it before the query.

## Query Construction Values

An object is a _query-construction-values node_ iff

- it has the property `node_construction_values_object <object>` and
- it has the property `node_construction_values_property_constructor <constructor>`.

It evaluates `<object>` and `<constructor>`. If `<object>` is a construction, the result is `undefined`. Otherwise the result is a deferred set of all values `v` for which the values of `<object>` contain a construction `c v` whose constructor `c` equals the evaluated `<constructor>`. The values of an object are its values in the knowledge if it is an atom and its members if it is a set; `undefined` is an atom and is queried like any other atom. If `<constructor>` cannot be materialised (a deferred set), the result is `undefined`.

## Query Constructions

An object is a _query-constructions node_ iff it has the property `node_query_constructions <constructor>`.

It evaluates `<constructor>` and returns a deferred set of all pairs `(a, v)` such that the knowledge contains the statement `a is <constructor> v`. Each pair is yielded as the object `{atom a, value v}`, where `atom` and `value` are atoms that are property constructors in the base knowledge. The pairs are yielded in knowledge order: by atom, and for each atom by the value's structural order. If `<constructor>` evaluates to a deferred set, it is materialized by iterating it before the query. `undefined` is an atom and may be used as a constructor like any other atom.

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

evaluates to a deferred set. Iterating it yields `2`, `3`, and `4`.

### Conditional

```
{
    node_if_condition {node_literal false},
    node_if_then {node_literal 1},
    node_if_else {node_literal 2}
}
```

evaluates to `2`.

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
| Equal | `node_equal_first`, `node_equal_second` | implemented for evaluated objects |
| Less | `node_less_first`, `node_less_second` | implemented for integers |
| Add | `node_add_first`, `node_add_second` | implemented for integers |
| Multiply | `node_multiply_first`, `node_multiply_second` | implemented for integers |
| Count | `node_count` | partial: sets only; deferred sizes unsupported |
| Values of | `node_values_of` | implemented |
| Has value | `node_has_value` | implemented |
| Union | `node_union_first`, `node_union_second` | partial: no deduplication or size; set operands only |
| Filter | `node_filter_set`, `node_filter_filter` | partial: iteration and membership only; set operand required |
| Map | `node_map_set`, `node_map_mapper` | partial: iteration and membership only; set operand required |
| Every | `node_every_set`, `node_every_predicate` | implemented; expressible with `fold` |
| Any | `node_any_set`, `node_any_predicate` | implemented; expressible with `fold` |
| If | `node_if_condition`, `node_if_then`, `node_if_else` | implemented |
| Unwrap | `node_unwrap_set` | implemented for sets |
| Is atom | `node_is_atom` | implemented |
| Is construction | `node_is_construction` | implemented |
| Is set | `node_is_set` | implemented |
| Construction constructor | `node_constructor_of_construction` | implemented |
| Construction value | `node_value_of_construction` | implemented |
| New construction | `node_new_construction_constructor`, `node_new_construction_value` | not implemented |
| Contains | `node_contains_element`, `node_contains_set` | not implemented |
| Query atoms with property | `node_atoms_with_property` | implemented |
| Query atoms with constructor | `node_atoms_with_constructor` | implemented |
| Query construction values | `node_construction_values_object`, `node_construction_values_property_constructor` | implemented |
| Query constructions | `node_query_constructions` | implemented |
| Fold | `node_fold_set`, `node_fold_folder`, `node_fold_initial` | implemented |

## Current Limitations

- Deferred set values (union, map, filter) can be iterated and tested for membership, and are materialized when converted to an object or compared for equality. Counting is not implemented for union, map, and filter results; atoms-with-property, atoms-with-constructor, construction-values, and constructions results can be counted. Construction constructor/value extraction on deferred values yields `undefined`.
- Deferred values and function values have undefined truthiness, so `and`, `or`, `xor`, `not`, and `if` produce `undefined` when given one directly.
- Function values are iterated through the values of their node representation, which is materialised on demand. Because node conversion is lossy, annotations and unknown properties of the original object are not part of that set.
- Set-taking nodes require a set. Atoms and constructions are rejected with `undefined`; use `values_of` to pass an atom's values.
- `union` does not deduplicate and cannot currently report its size.
- Converting a node to an object is lossy: unknown properties and annotations are not preserved, and atom-defined nodes are normalized into sets.
- `arithmetic_overflow` is declared but unreachable because arithmetic uses arbitrary-precision integers.
