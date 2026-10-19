# Computation

Everything has a computation system where functions and expression can be evaluated. The expressions are modeled in Everything. The whole system is pure and based off lambda calculus with added nodes.

Generally, you need a knowledge to evaluate nodes/expressions in, because some nodes query that knowledge. You _could_ evaluate a subset of the system without knowledge context.

## Truthyness

- An object is _falsy_ iff it is the empty object `{}`.
- An object is _truthy_ iff it is not falsy.

## Functions

An object is a _function_ iff it has the property `node_function <body>`. The associated value of the construction, `<body>`, is the function body. Functions have exactly one parameter.

## Parameter References

An object is a _parameter reference_ iff it has the property `node_parameter <depth>`. `<depth>` is a non-negative integer referencing the (runtime-)value of the parameter of an enclosing function. `<depth>` denotes the number of enclosing functions to skip (counting from within), before reaching the target function.

## Function Self-References

An object is a (function) _self-reference_ iff it has the property `node_function_reference <depth>`. These nodes exist to support recursive functions without fix-point operators and evaluate to enclosing function objects. `<depth>` is a non-negative integer, referencing an enclosing function, just like parameter references do.

## Function Application

An object is a _function application/call_ iff

- it has the property `node_call_callee <callee>` and
- it has the property `node_call_value <value>`.

You can apply/call a function with a value using this node where `<callee>` is a sub-expression/node, the result of which will get called with the evaluation result of `<value>`.

## Logical And

Shape: `{node_and_first <first>, node_and_second <second>}`.

## Logical Or

Shape: `{node_or_first <first>, node_or_second <second>}`.

## Logical XOr

Shape: `{node_xor_first <first>, node_xor_second <second>}`.

## Set Union

Shape: `{node_union_first <first>, node_union_second <second>}`.

## Set Count

An object is a _count node_ iff it has a single property `node_count <inner>`. It counts the number of elements in the evaluation result of `<inner>`. For constructions, it evaluates to 0.

## Filter

An object is a _filter node_ iff

- it has a single property `node_filter_set <set>` and
- it has a single property `node_filter_filter <filter>`.

It evaluates to the set of all objects from the evaluation result of `<set>` for which `{node_call_callee <filter>, node_call_value <o>}` evaluates to a truthy object (with `<o>` being an object from the set).

## Map

An object is a _map node_ iff

- it has a single property `node_map_set <set>` and
- it has a single property `node_map_mapper <mapper>`.

It first evaluates `<set>` and then maps each object of the result to the result of `{node_call_callee <mapper>, node_call_value <o>}` (with `<o>` being an obejct from the set). The map node then evaluates to the set of all objects returned from the calls.

## Conditional Nodes

An object is an _if-node_ iff

- it has a single property `node_if_condition <condition>`,
- it has a single property `node_if_then <then>`, and
- it has a single property `node_if_else <else>`.

First, the if-node evaluates `<condition>`. If the result of that evaluation is a truthy object, then the if-node evaluates to result of the evaluation of `<then>`; otherwise to the result of the evaluation of `<else>`.

## Query Atoms With Property

An object is a _query-atoms-with-property-node_ iff

- it has a single property `node_query_atoms_with_property <property>`.

It evaluates to the set of all atoms that have the result of the evaluation of `<property>` as a property.

## Query Values Of Constructions

An object is a _query-values-of-constructions-node_ iff

- it has a single property `query_values_of_constructions_target <target>` and
- it has a single property `query_values_of_constructions_constructor, <constructor>`.

It evaluates to the set of all objects that
