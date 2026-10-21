# Chapter 3 - Computation

Everything has a computation system where functions and expressions can be evaluated. The expressions are modelled in Everything. The whole system is pure and based off lambda calculus with added nodes.

Generally, you need a knowledge to evaluate nodes/expressions in, because some nodes query that knowledge. You _could_ evaluate a subset of the system without knowledge context.

This chapter describes the intended model. Some nodes are declared but not fully implemented yet; those are noted below.

The following sections define node types and how they evaluated. Together, they define "evaluate" which is a function from an object to an object.

## Truthiness

- An object is _falsy_ iff it is the empty set `{}`.
- An object is _truthy_ iff it is not falsy.

## Functions

An object is a _function_ iff it has the property `node_function <body>`. The associated value of the construction, `<body>`, is the function body. Functions have exactly one parameter.

## Literals

An object is a _literal_ iff it has the property `node_literal <object>`. A literal evaluates to `<object>` without evaluating it. This is useful to prevent a value from being interpreted as a node.

## Parameter References

An object is a _parameter reference_ iff it has the property `node_parameter <depth>`. `<depth>` is a non-negative integer referencing the (runtime-)value of the parameter of an enclosing function. `<depth>` denotes the number of enclosing functions to skip (counting from within), before reaching the target function.

## Function Self-References

An object is a (function) _self-reference_ iff it has the property `node_function_self <depth>`. These nodes exist to support recursive functions without fix-point operators and evaluate to enclosing function objects. `<depth>` is a non-negative integer, referencing an enclosing function, just like parameter references do.

## Function Application

An object is a _function application/call_ iff

- it has the property `node_call_callee <callee>` and
- it has the property `node_call_with <value>`.

You can apply/call a function with a value using this node where `<callee>` is a sub-expression/node, the result of which will get called with the evaluation result of `<value>`.

## Logical And

An object is an _and node_ iff

- it has the property `node_and_first <left>` and
- it has the property `node_and_second <right>`.

It first evaluates `<left>`. If the result is falsy, the and-node is falsy. Otherwise it evaluates `<right>` and is truthy iff `<right>` is truthy.

## Logical Or

An object is an _or node_ iff

- it has the property `node_or_first <left>` and
- it has the property `node_or_second <right>`.

It first evaluates `<left>`. If the result is truthy, the or-node is truthy. Otherwise it evaluates `<right>` and is truthy iff `<right>` is truthy.

## Logical XOr

An object is an _xor node_ iff

- it has the property `node_xor_first <left>` and
- it has the property `node_xor_second <right>`.

It evaluates both `<left>` and `<right>`. The xor-node is truthy iff exactly one of the two results is truthy.

## Logical Not

An object is a _not-node_ iff it has the property `node_not <inner>`. It evaluates `<inner>` and is truthy iff the result is falsy.

## Equality

An object is an _equal node_ iff

- it has the property `node_equal_first <left>` and
- it has the property `node_equal_second <right>`.

It evaluates both children and is truthy iff their results are structurally equal.

## Less Than

An object is a _less-than node_ iff

- it has the property `node_less_first <left>` and
- it has the property `node_less_second <right>`.

It evaluates both children. If both results are integers, it is truthy iff `<left> < <right>`. Otherwise it compares the number of elements of the results.

## Addition

An object is an _add node_ iff

- it has the property `node_add_first <left>` and
- it has the property `node_add_second <right>`.

It evaluates both children. If both results are integers, it evaluates to their sum. If the sum overflows, it evaluates to `arithmetic_overflow`. If the results are not both integers, it evaluates to `undefined`.

## Multiplication

An object is a _multiply node_ iff

- it has the property `node_multiply_first <left>` and
- it has the property `node_multiply_second <right>`.

It evaluates both children. If both results are integers, it evaluates to their product. If the product overflows, it evaluates to `arithmetic_overflow`. If the results are not both integers, it evaluates to `undefined`.

## Set Count

An object is a _count node_ iff it has a single property `node_count <inner>`. It counts the number of elements in the evaluation result of `<inner>`. For constructions, it evaluates to 0.

## Set Union

An object is a _union node_ iff

- it has the property `node_union_first <left>` and
- it has the property `node_union_second <right>`.

It evaluates both children and evaluates to the set that contains all elements from the result of the left child and all elements from the result of the right child.

## Filter

An object is a _filter node_ iff

- it has a single property `node_filter_set <set>` and
- it has a single property `node_filter_filter <filter>`.

It evaluates `<set>` and keeps all objects from the result for which `{node_call_callee <filter>, node_call_with <o>}` evaluates to a truthy object (with `<o>` being an object from the set).

## Map

An object is a _map node_ iff

- it has a single property `node_map_set <set>` and
- it has a single property `node_map_mapper <mapper>`.

It first evaluates `<set>` and then maps each object of the result to the result of `{node_call_callee <mapper>, node_call_with <o>}` (with `<o>` being an object from the set). The map node then evaluates to the set of all objects returned from the calls.

## Every

An object is an _every node_ iff

- it has a single property `node_every_set <set>` and
- it has a single property `node_every_predicate <predicate>`.

It evaluates `<set>` and is truthy iff `{node_call_callee <predicate>, node_call_with <o>}` evaluates to a truthy object for every object `<o>` in the result. The empty set vacuously satisfies the predicate.

## Any

An object is an _any node_ iff

- it has a single property `node_any_set <set>` and
- it has a single property `node_any_predicate <predicate>`.

It evaluates `<set>` and is truthy iff `{node_call_callee <predicate>, node_call_with <o>}` evaluates to a truthy object for at least one object `<o>` in the result.

## Conditional Nodes

An object is an _if-node_ iff

- it has a single property `node_if_condition <condition>`,
- it has a single property `node_if_then <then>`, and
- it has a single property `node_if_else <else>`.

First, the if-node evaluates `<condition>`. If the result of that evaluation is a truthy object, then the if-node evaluates to result of the evaluation of `<then>`; otherwise to the result of the evaluation of `<else>`.

## Unwrap Or

An object is an _unwrap-or node_ iff

- it has the property `node_unwrap_or_set <set>` and
- it has the property `node_unwrap_or_default <default>`.

It evaluates `<set>`. If the result contains exactly one object, that object is returned. Otherwise the default expression is evaluated and returned.

## Is Atom

An object is an _is-atom node_ iff it has a single property `node_is_atom <inner>`. It evaluates `<inner>` and is truthy iff the result is an atom.

## Query Atoms With Property

An object is a _query-atoms-with-property-node_ iff

- it has a single property `node_atoms_with_property <property>`.

It evaluates `<property>` and evaluates to the set of all atoms that have the result as a property in the knowledge.

## Query Construction Values

An object is a _query-construction-values-node_ iff

- it has a single property `node_construction_values_set <set>` and
- it has a single property `node_construction_values_property_constructor <constructor>`.

It evaluates `<set>` and `<constructor>` and evaluates to the set of all values `v` for which the evaluated set contains a construction `c v` whose constructor `c` equals the evaluated property constructor.

The last two query nodes are declared but not fully implemented yet.
