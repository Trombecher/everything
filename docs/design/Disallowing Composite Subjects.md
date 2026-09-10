# Disallowing composite objects as subjects in statements.

The current model of Everything disallows composite objects in statements. This is model B. Previously, it was allowed to do such. This is model A.

## Arguments in favour of model A

* We are even more flexible in what we can state. We can annotate composite objects and put them into relation with other objects.

* Infinine composite objects are possible: `({$LIST_ITEM: 0}, $LIST_TAIL, {$LIST_ITEM: 0})`. This example is an infinite list of zeroes.

* Polyglots are possible, i.e. objects that satisfy multiple (normally disjoint) definitions.

## Arguments in favour of model B

* In A, meaning and properties of composite objects extend to beyond what they are identified as. A composite object may represent something under one knowledge and something different in another (with the same definitions of the tags).

* Model B would have the advantage that expressions and computation using only composite objects would be **unambiguous**, i.e. the same in all knowledge bases because the properties of composite objects are unambiguous across all knowledge bases. And the database would not need to iterate over all knowledge to find the value(s) of some property on a composite object that is not there.

* In A there are more footguns: one can state `({}, $CONTAINS, {})` and now there is no false anymore. This means that every function which returned `{}` (and meant _false_) now returns _true_.

* Model B would also only disallow statements that have both a composite object as the subject and as the value, for every other statement (that has a composite object as subject and an abstract object as value), the direction of the statement can be reversed and now an abstract object is the subject. This would require a rewrite of the model to reverse the direction of tags.

* In A, definitions — such as the ones for integers — are unsound if they do not check for additional properties other than the ones required. Is `{$SUCCESSOR_OF: $ZERO, $CONTAINS: {}}` an integer (1)? If yes, then, because `{$SUCCESSOR_OF: $ZERO}` is also 1, two objects that represent the same integers would be structurally unequal but the things they represent would be equal, which violates a core principle of Everything.

  The only object that represents 1 should be `{$SUCCESSOR_OF: $ZERO}`. This obeys the principle "object is integer iff object represents mathematical integer".

* Consider `{$CONTAINS: 50}` and `{}` with `({}, $CONTAINS, 50)` in the knowledge. They are different objects because of structural equality but if you'd query them in A, they have the same properties.

* Model B would collapse the entire knowledge down to a set of abstract objects having properties, something like this:

  ```
  $Alice = {$PERSON: {}}
  $Bob = {$PERSON: {}}
  $MySet = {$CONTAINS: 10}
  ```

  This may be desireable

I even make the claim that any statements involving both composite subjects and values are redundant and computable/inferrable from the composite objects themselves.

## Verdict

Model B is better.
