# Chapter 6 - Design Decisions

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
