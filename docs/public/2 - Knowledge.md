# Chapter 2 - Knowledge

We define _knowledge_ as a set of statements. A _statement_ is a combination of an atom `%a%` with an object `%p%`, written `%a% is %p%`. Mathematically, it is a pair. Semantically, a statement states that an atom `%a%` has the property `%p%` and therefore is an element of the set of all objects that have this property.

## Properties Of Atoms And Composites

An object `%p%` is a property of an atom or a composite `%o%` in (knowledge) `%K%` iff

- `%o%` is an atom and `%o% is %p% ∈ %K%`; or
- `%o%` is a composite and `%p% ∈ %o%`.

The properties set of an atom or composite `%o%` in (knowledge) `%K%` is defined to be the set of all objects that are properties of `%o%`.

An object `%c%` is a _property constructor_ in knowledge `%K%` iff there exists an object `%f%` such that `property_constructor ·· %f%` is a property of `%c%` in `%K%`. This `%f%` is the property constructor's _constraint_. It follows that both atoms and composites may be property constructors. Knowledge validation condition 1 assures that there is at most one such `%f%`.

It is forbidden to ask for properties of a construction.

## Validation

Models can restrict themselves by imposing arbitrary constraints on the subjects and values of constructed properties and on the subjects of non-constructed properties. These constraints apply wherever properties are used, whether in statements or composites. They are modelled directly in Everything with a functional computation system. This system can evaluate objects and is laid out in [the next chapter](./3%20-%20Computation.md).

There are three levels of validation that have different roles: knowledge validation checks statements, object validation checks recursive structure, and property validation checks the use of a property in a particular subject and, where applicable, value context.

### Validation Of Knowledge

Everything can validate knowledge. Validation checks that a knowledge is consistent with its own declarations; it does not restrict which facts a knowledge may state (see [Limits of Validation](#limits-of-validation)). Knowledge `%K%` is _valid_ iff

1. for all atoms and composites `%o%` that occur in `%K%` there exists at most one object `%f%` such that `property_constructor ·· %f%` is a property of `%o%` in `%K%`;
2. for all atoms and composites `%o%` that occur in `%K%` there exists at most one object `%f%` such that `non_constructed_property ·· %f%` is a property of `%o%` in `%K%`; and
3. for every statement `%a% is %p% ∈ %K%`, `%p%` is a valid property of `%a%` in `%K%`.

The correctness of condition 3 relies on conditions 1 and 2, so they are to be checked first. The definitions in the following sections rely on conditions 1 and 2 being true.

As a consequence, the empty knowledge is valid.

### Validation Of Properties

In Everything, every _property_ is either non-constructed or constructed. You can construct properties via constructions with property constructors.

An object `%p%` is a **valid** property of an atom or a composite `%o%` in (knowledge) `%K%` iff `%p%` is a valid object in `%K%`, `%p%` is a property of `%o%` in `%K%`, and at least one of the following conditions holds:

- there exist objects `%c%` and `%v%` such that `%p% = %c% ·· %v%`,
  - `%c%` is a property constructor in `%K%` with constraint `%f%`, and
  - `⟨node_call_callee ·· ⟨node_call_callee ·· %f%, node_call_with ·· ⟨node_literal ·· %o%⟩⟩, node_call_with ·· ⟨node_literal ·· %v%⟩⟩` evaluates to `true` in `%K%`; or
- `%p%` is an atom or a composite,
  - there exists an object `%f%` such that `non_constructed_property ·· %f%` is a property of `%p%` in `%K%`, and
  - `⟨node_call_callee ·· %f%, node_call_with ·· ⟨node_literal ·· %o%⟩⟩` evaluates to `true` in `%K%`.

The literal nodes pass the original subject and value to the constraint as data, even when those objects themselves look like nodes.

### Validation Of Objects

The motivation behind valid objects is to check the recursive structure of property constructions and to ensure that composites bear their members as valid properties. Formally, an object `%o%` is _valid_ in knowledge `%K%` iff one of the following holds:

- `%o%` is an atom;
- `%o%` is a construction `%c% ·· %v%`, `%c%` and `%v%` are valid objects, and `%c%` is a property constructor in `%K%`; or
- `%o%` is a composite and every property `%p%` of `%o%` is a valid property of `%o%` in `%K%`.

The composite case makes validity deep: since a valid property must also be a valid object, every member is recursively checked as an object, and every construction that is a member of a valid composite also has its constraint applied with that composite as the subject. Nested composites are checked the same way with themselves as the subject. A construction that is only used as a value is checked structurally: its constructor must be a property constructor and its value must be valid. Its constraint runs where a subject exists, namely in statements and in the composites that contain it as a member.

For example, suppose `age` is a property constructor whose constraint rejects `-1`, and `other` is another property constructor. The construction `age ·· -1` is still a valid object, but `⟨age ·· -1⟩` is not: the construction is a member of the composite, so the `age` constraint rejects that property use. In contrast, `other ·· (age ·· -1)` is a valid object as long as `other` and the nested construction are structurally valid. If that outer construction is used as a property, only `other`'s constraint is applied automatically to the outer use; the `age` constraint does not run merely because `age ·· -1` is nested in the value.

Constraints therefore only guarantee something where there is a subject. If the meaning of a value relies on its constructor's constraint, that guarantee does not survive when the construction is used as a value: nesting the construction one level deeper is enough to skip the constraint, so any constraint can be dodged this way. The constraint of a property constructor says nothing about constructions of that constructor that merely occur as values.

The constraint on property constructors used in composites and statements allows for validation, not only on the value, but on the subject as well.

Composites are therefore strong: a composite must be able to bear all of its members as properties. Computed collections avoid this requirement by using the canonical set form from [Chapter 4](./4%20-%20Common%20Models.md#sets): a composite whose members are all `contains ·· %item%` constructions. The constraint of `contains` accepts every value for every subject, so such a composite is valid whenever its items are. Query results and other computed collections are therefore valid objects, and collecting objects into a canonical set never runs their constraints. The open set model still applies to all atoms and composites; a set that has additional properties beyond its `contains` properties is validated like any other atom or composite.

### Limits of Validation

Validation protects the meaning of the property constructors and non-constructed properties that a knowledge uses. It does not prevent a knowledge from introducing new ones or from stating arbitrary facts.

The reason is that a declaration is just a property: for an atom it comes from a statement, and for a composite it is one of its members. Since composites have exactly their members as properties, a composite that contains `property_constructor ·· %f%` is itself a property constructor with constraint `%f%`. A composite that contains `non_constructed_property ·· %f%` is itself a non-constructed property with constraint `%f%`. Such composites can be written inline, so a declaration does not need to be a separate statement about an atom. The inline constructor has no separate atom name, but it is a property constructor in exactly the same sense as a declared one.

For example, let `%f%` be `⟨node_function ·· ⟨node_literal ·· true⟩⟩`, which returns `true` for every subject, and let `%g%` be `⟨node_function ·· ⟨node_function ·· ⟨node_literal ·· true⟩⟩⟩`, which returns such a function for every subject. Then the following statement is valid without any prior declaration of `%f%` or of the composite:

```
a is ⟨non_constructed_property ·· %f%⟩
```

The composite is a non-constructed property of `a`, because it contains `non_constructed_property ·· %f%`, and `%f%` accepts `a`. The composite is the property of the statement, not its subject; only the subject of a statement must be an atom. Likewise, a property can carry its own constructor inline:

```
a is ⟨property_constructor ·· %g%⟩ ·· v
```

Here `%g%` is a function that accepts every subject and value, so it accepts `a` and `v`. The declaration of the constructor is part of the object that uses it.

Even without inline declarations, anyone who can add statements can declare a new constructor whose constraint accepts everything and then use it. Validation cannot distinguish a well-motivated declaration from a permissive one; both are declarations that the knowledge makes.

What validation does guarantee is narrower: every property constructor and every non-constructed property has at most one constraint, constraints are applied wherever properties are used with a subject, and a composite must be able to bear all of its members. These checks keep a knowledge consistent with its own declarations. They are not an access-control boundary over who may add which declarations or facts. That boundary belongs to the process that controls writing to the knowledge.

## Base For Valid, Non-Empty Knowledge

For a non-empty knowledge `%K%`, just one statement requires the existance of other statements. Let that statement be `%a% is %p% ∈ %K%`. Valid knowledge requires that `%p%` be a valid property of `%a%` in `%K%`.

In the case that `%p%` is an atom or a composite,

- it is required that there exists an object `%f%` such that `non_constructed_property ·· %f%` is a property of `%p%` in `%K%`.
- This implies that (A) `non_constructed_property` must be a property constructor in `%K%`.
- This implies that `property_constructor` must be a property constructor itself in `%K%`, which leads to (B) `property_constructor is property_constructor ·· %h% ∈ %K%`.

In the case that `%p%` is a construction, it is required that the constructor of `%p%` be a property constructor in `%K%` which leads to the same condition, B, being required.

What follows that, in every case, `property_constructor is property_constructor ·· %h% ∈ %K%` is required for every non-empty `%K%`. If there exists a statement in `%K%` that includes an atom or a composite as a property, then `non_constructed_property is property_constructor ·· %g% ∈ %K%` is required.

### How Implementation Handle This

For implementations, it is attractive to move the checking of conditions 1 and 2 of knowledge validation into condition 3.

To encode knowledge validation condition 1 directly within knowledge, `%h%` must be a constraint that ensures that there is at most one value for constructions with the property constructor `property_constructor` on the subject where `property_constructor` is used as the property constructor in a property. Because this equivalence is undecidable, implementations may require `%h%` to be some implementation-defined object. The same with condition 2.

What emerges is an alternative definition of valid knowledge by

- enforcing `property_constructor is property_constructor ·· %h% ∈ %K%` and `non_constructed_property is property_constructor ·· %g% ∈ %K%`, uniquely; and
- `%h%` and `%g%` being equal to some implementation defined objects which check for uniqueness.
- Then, condition 3 is required.

### Other Conditions

Implementation may impose additional conditions for knowledge to be valid. For node objects of the computation system to be valid in the knowledge, each property constructor must be declared in the knowledge.

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
