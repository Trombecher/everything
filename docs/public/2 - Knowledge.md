# Chapter 2 - Knowledge

We define _knowledge_ as a set of statements. A statement is a combination of an atom `%a%` with an object `%o%`, written `%a% is %o%`. Mathematically, it is a pair. Semantically, a statement states that an atom `%a%` has the property `%o%` and therefore is an element of the set of all objects that have this property.

## Validation

To prevent knowledge from reaching arbitrariness, Everything can validate knowledge. Knowledge `%K%` is _valid_ iff for every statement `%a% is %p% ∈ %K%`, `%p%` is a valid property of `%a%` in `%K%`. As defined below, this also requires `%p%` to be a valid object in `%K%`.

Models can restrict themselves by imposing arbitrary constraints on the subjects and values of constructed properties and on the subjects of non-constructed properties. These constraints apply wherever properties are used, whether in statements or composites. They are modelled directly in Everything with a functional computation system. This system can evaluate objects and is laid out in [the next chapter](./3%20-%20Computation.md).

Validation performs every required constraint evaluation in the complete knowledge `%K%` itself, not in a progressively validated subset. The statement being checked is therefore visible to its constraint, and statements have no validation order. A constraint accepts only when it evaluates to exactly `true`; every other result rejects the property. Since constraints are arbitrary computations, validation is not guaranteed to terminate.

The three levels of validation have different roles: knowledge validation checks statements, object validation checks recursive structure, and property validation checks the use of a property in a particular subject and, where applicable, value context.

### Properties Of Atoms And Composites

The properties (set) of an atom or composite `%o%` in (knowledge) `%K%` is defined to be

- `{%p% | %o% is %p% ∈ %K%}` if `%o%` is an atom; or
- `%o%` if `%o%` is a composite.

An object `%p%` is a property of an atom or a composite `%o%` in (knowledge) `%K%` iff

- `%o%` is an atom and `%o% is %p% ∈ %K%`; or
- `%o%` is a composite and `%p% ∈ %o%`.

It is forbidden to ask for properties of a construction.

### Valid Properties

In Everything, every _property_ is either non-constructed or constructed. You can construct properties via constructions with property constructors.

An object `%c%` is a _property constructor_ in knowledge `%K%` iff there exists exactly one object `%f%` such that `property_constructor ·· %f%` is a property of `%c%` in `%K%`. This `%f%` is the constructor's constraint. Both atoms and composites may be property constructors.

An object `%p%` is a **valid** property of an atom or a composite `%o%` in (knowledge) `%K%` iff `%p%` is a valid object in `%K%`, `%p%` is a property of `%o%` in `%K%`, and at least one of the following conditions holds:

- there exist objects `%c%` and `%v%` such that `%p% = %c% ·· %v%`,
  - `%c%` is a property constructor in `%K%` with constraint `%f%`, and
  - `⟨node_call_callee ·· ⟨node_call_callee ·· %f%, node_call_with ·· ⟨node_literal ·· %o%⟩⟩, node_call_with ·· ⟨node_literal ·· %v%⟩⟩` evaluates to `true` in `%K%`; or
- `%p%` is an atom or a composite,
  - there exists exactly one object `%f%` such that `non_constructed_property ·· %f%` is a property of `%p%` in `%K%`, and
  - `⟨node_call_callee ·· %f%, node_call_with ·· ⟨node_literal ·· %o%⟩⟩` evaluates to `true` in `%K%`.

The literal nodes pass the original subject and value to the constraint as data, even when those objects themselves look like nodes.

Constraints are unique rather than alternative. In particular, the constraint on `property_constructor` itself in the required base knowledge rejects multiple `property_constructor` declarations on the same object; validation may not choose whichever constraint returns `true`.

### Valid Objects

The motivation behind valid objects is to check the recursive structure of property constructions and to ensure that composites bear their members as valid properties. Formally, an object `%o%` is _valid_ in knowledge `%K%` iff one of the following holds:

- `%o%` is an atom;
- `%o%` is a construction `%c% ·· %v%`, `%c%` and `%v%` are valid objects, and `%c%` is a property constructor in `%K%`; or
- `%o%` is a composite and every property `%p%` of `%o%` is a valid property of `%o%` in `%K%`.

The composite case makes validity deep: since a valid property must also be a valid object, every member is recursively checked as an object, and every construction that is a member of a valid composite also has its constraint applied with that composite as the subject. Nested composites are checked the same way with themselves as the subject. A construction that is only used as a value is checked structurally: its constructor must be a property constructor and its value must be valid. Its constraint runs where a subject exists, namely in statements and in the composites that contain it as a member.

For example, suppose `age` is a property constructor whose constraint rejects `-1`, and `other` is another property constructor. The construction `age ·· -1` is still a valid object, but `⟨age ·· -1⟩` is not: the construction is a member of the composite, so the `age` constraint rejects that property use. In contrast, `other ·· (age ·· -1)` is a valid object as long as `other` and the nested construction are structurally valid. If that outer construction is used as a property, only `other`'s constraint is applied automatically to the outer use; the `age` constraint does not run merely because `age ·· -1` is nested in the value.

The constraint on property constructors used in composites and statements allows for validation, not only on the value, but on the subject as well.

Composites are therefore strong: a composite must be able to bear all of its members as properties. Computed collections avoid this requirement by using the canonical set form from [Chapter 4](./4%20-%20Common%20Models.md#sets): a composite whose members are all `contains ·· %item%` constructions. The constraint of `contains` accepts every value for every subject, so such a composite is valid whenever its items are. Query results and other computed collections are therefore valid objects, and collecting objects into a canonical set never runs their constraints. The open set model still applies to all atoms and composites; a set that has additional properties beyond its `contains` properties is validated like any other atom or composite.

## Examples

### Valid Knowledge

The following example extends the required base knowledge. Decimal integers are used for readability.

```
# `person` is a property that accepts all subjects (function always returns true).
person is non_constructed_property ·· ⟨node_function ·· ⟨node_literal ·· true⟩⟩

# `date_time`, `born`, and `child_of` are property constructors.
# The first two accept every subject and value, so their constraints are curried
# functions that accept both arguments before returning true.

date_time is property_constructor ·· ⟨
    node_function ·· ⟨
        node_function ·· ⟨node_literal ·· true⟩
    ⟩
⟩

born is property_constructor ·· ⟨
    node_function ·· ⟨
        node_function ·· ⟨node_literal ·· true⟩
    ⟩
⟩

child_of is property_constructor ·· ⟨
    # child
    node_function ·· ⟨
        # parent
        node_function ·· ⟨
            node_and_first ·· ⟨
                # child is a person
                node_has_property_subject ·· ⟨
                    node_parameter ·· 1
                ⟩,
                node_has_property_property ·· ⟨
                    node_literal ·· person
                ⟩
            ⟩,
            node_and_second ·· ⟨
                # parent is a person
                node_has_property_subject ·· ⟨
                    node_parameter ·· 0
                ⟩,
                node_has_property_property ·· ⟨
                    node_literal ·· person
                ⟩
            ⟩
        ⟩
    ⟩
⟩

David is person
David is born ·· ⟨date_time ·· 10234823948⟩
David is child_of ·· Anna

Anna is person
Anna is born ·· ⟨date_time ·· 45385034985039⟩
```

To validate `David is child_of ·· Anna`, object validation first observes that `child_of` and `Anna` are atoms and that `child_of` is a property constructor. Property validation then calls the unique `child_of` constraint with `David` and `Anna` as literal arguments. The constraint finds `David is person` and `Anna is person` in the complete knowledge and returns `true`.

Validating `David is born ·· ⟨date_time ·· 10234823948⟩` also checks the nested `date_time` construction. It must be a valid object and a valid property of its enclosing composite, and that composite is passed to the `date_time` constraint as the subject. The `born` constraint separately validates the outer construction with `David` as the subject.

### Invalid Knowledge

The following examples use the declarations above. Each illustrates a different reason validation fails:

- An undeclared constructor makes the property an invalid object:

  ```
  David is unknown_constructor ·· Anna
  ```

  `unknown_constructor` has no unique `property_constructor` constraint.

- A constructor may reject the subject even when the property is structurally valid:

  ```
  Rock is child_of ·· Anna
  ```

  `Rock` is not a person, so the `child_of` constraint does not return `true`.

- A constructor may independently reject the value:

  ```
  David is child_of ·· Rock
  ```

  Here the subject is a person, but the value is not.

- Accepting outer constraints cannot hide an invalid nested construction:

  ```
  David is born ·· ⟨date_time ·· (unknown_constructor ·· Anna)⟩
  ```

  Although `born` and `date_time` accept every subject and value, `unknown_constructor ·· Anna` is not a valid object. Recursive object validation therefore rejects the entire statement.

- Multiple constraints are not alternatives. For distinct `%f1%` and `%f2%`, the following declarations are rejected by the `property_constructor` constraint in the required base knowledge:

  ```
  child_of is property_constructor ·· %f1%
  child_of is property_constructor ·· %f2%
  ```

  Validation cannot choose whichever constraint accepts, even if one or both would return `true`.

## Statements About Composites

There might be cases where the user of the knowledge base might want to state something about objects that are not atoms. Everything disallows such statements.

### Why Everything Does That

There are a few reasons:

1. Statements about non-atoms would add information to things that are inherently identified only by their intrinsic structure. Non-atoms must not have any knowledge-level context-specific meaning.
2. A big family of such statements can be inferred/computed by the computation system (e.g. `3 is prime_number`) (and thus are redundant).
3. Such statements would introduce foot guns, like declaring the empty composite to have some property (thus not really being the empty composite anymore).
4. Implementation burden: composites and constructions would have an additional set of properties from the knowledge they are _used_ in.

More discussion of this topic can be found [here](../design/Disallowing%20Composite%20Subjects.md).

There are workarounds for some cases.

### Workarounds

- The statement you want to state may be computable by a function in the computation system. For example, there is no need to state `4 is even`, since you can compute evenness of integers.
- The statement you want to state may be reversible if the property is constructed with an atom value. For example, `10 is favourite_integer_of ·· Alice` may be (correctly) stated as `Alice is having_favourite_integer ·· 10` which works because `Alice` is an atom.
- NOT RECOMMENDED: if the statement involves an atom as the property, you can flip the statement: `a is b` could become `b is contained_in ·· a`. This is not recommended since there would be two definitions of set inclusion.
