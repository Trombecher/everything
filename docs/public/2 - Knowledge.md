# Chapter 2 - Knowledge

We define _knowledge_ as a set of statements. A statement is a combination of an atom `%a%` with an object `%o%`, written `%a% is %o%`. Mathematically, it is a pair. Semantically, a statement states that an atom `%a%` has the property `%o%` and therefore is an element of the set of all objects that have this property.

## Validation

To prevent knowledge from reaching arbitrariness, Everything can validate knowledge. Knowledge `%K%` is _valid_ iff for every statement `%a% is %p% ∈ %K%`, `%p%` is a valid object and `%p%` is a valid property of `%a%` in `%K%`.

Models can restrict themselves by imposing arbitrary constraints on values in property constructions and on composites where properties are used. These constraints are modelled directly in Everything with a functional computation system. This system can evaluate objects and is laid out in [the next chapter](./3%20-%20Computation.md).

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

An object `%p%` is a **valid** property of an atom or a composite `%o%` in (knowledge) `%K%` iff `%p%` is a property of `%o%` in `%K%` and at least one of the following conditions holds:

- there exist objects `%c%` and `%v%` such that `%p% = %c% ·· %v%`,
    - there exists an object `%f%` such that `property_constructor ·· %f%` is a property of `%c%` in `%K%`, and
    - `⟨node_call_callee ·· ⟨node_call_callee ·· %f%, node_call_with ·· %o%⟩, node_call_with ·· %v%⟩` evaluates to `true` in `%K%`; or
- `%p%` is an atom or a composite,
    - there exists an object `%f%` such that `property ·· %f%` is a property of `%p%` in `%K%`, and
    - `⟨node_call_callee ·· %f%, node_call_with ·· %o%⟩` evaluates to `true` in `%K%`.

### Valid Objects

The motivation behind valid objects is to check that all property constructions in that object are valid. Formally, an object `%o%` is _valid_ in knowledge `%K%` iff one of the following holds:

- `%o%` is an atom;
- `%o%` is a construction `%c% ·· %v%`, `%c%` and `%v%` are valid objects, and `%c%` is a property constructor in `%K%` (that is, `%c% is property_constructor ·· %f% ∈ %K%` for some object `%f%`); or
- `%o%` is a composite and every member `%m%` of `%o%` is a valid property of `%o%` in `%K%`.

The composite case makes validity deep: every construction in a valid composite has its constraint applied with that composite as the subject, and nested composites are checked the same way with themselves as the subject. A construction that is only used as a value is checked structurally: its constructor must be a property constructor and its value must be valid. Its constraint runs where a subject exists, namely in statements and in the composites that contain it as a member.

The constraint on property constructors used in composites and statements allows for validation, not only on the value, but on the subject as well.

Composites are therefore strong: a composite must be able to bear all of its members as properties. Data does not run into this requirement because data collections are sets (see [Chapter 4](./4%20-%20Common%20Models.md#sets)): a set is a composite whose members are all `contains ·· %item%` constructions, and the constraint of `contains` accepts every value for every subject. A set is valid whenever its items are, so query results and other computed collections are valid objects.

## Examples

```
# `person` is a property that accepts all subjects (function always returns true).
person is property ·· ⟨node_function ·· ⟨node_literal ·· true⟩⟩

# `date_time`, `born`, and `child_of` are property constructors.

date_time is property_constructor ·· ⟨node_function ·· ⟨node_literal ·· true⟩⟩

born is property_constructor ·· ⟨node_function ·· ⟨node_literal ·· true⟩⟩

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

## Reasoning About Multiple Knowledges

Reasoning about knowledge of two or more knowledge bases (implicitly) unions them into one knowledge base. Even if all knowledge bases are valid on their own, this does not mean that the union is valid.

It might be the case that two or more knowledge bases may model the same things with different atoms (because the creators worked independently, for example). Merging them may lead the maintainers to discover such cases. To uphold the model assumption of Everything, their only choice is to **rename all atoms with the same meaning to one id**. It also may be beneficial to ensure that the union is valid knowledge.

## Statements About Composites

There might be cases where the user of the knowledge base might want to state something about objects that are not atoms. Everything disallows such statements.

### Why Everything Does That

There are a few reasons:

1. Statements about non-atoms would add information to things that are inherently identified only by their intrinsic structure. Non-atoms must not have any knowledge-level context-specific meaning.
2. A big family of such statements can be inferred/computed by the computation system (e.g. `3 is prime_number`) (and thus are redundant).
3. Such statements would introduce foot guns, like declaring the empty composite to have some property (thus not really being the empty composite anymore).
4. Implementation burden: composites and constructions would have an additional set of properties from the knowledge they are _used_ in.

More discussion (using the old model) of this topic can be found [here](../design/old%20model/Disallowing%20Composite%20Subjects.md).

There are workarounds for some cases.

### Workarounds

- The statement you want to state may be computable by a function in the computation system. For example, there is no need to state `4 is even`, since you can compute evenness of integers.
- The statement you want to state may be reversible if the property is constructed with an atom value. For example, `10 is favourite_integer_of ·· Alice` may be (correctly) stated as `Alice is having_favourite_integer ·· 10` which works because `Alice` is an atom.
- NOT RECOMMENDED: if the statement involves an atom as the property, you can flip the statement: `a is b` could become `b is contained_in ·· a`. This is not recommended since there would be two definitions of set inclusion.
