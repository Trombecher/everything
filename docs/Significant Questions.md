# How do you prove that you can model anything in Everything?

# Should there be database-level versioning of knowledge?

One of Everything's goals is that it should replace any other data meta-model by providing a unified meta-model. At the database level, knowledge is stored — conceptually as one big composite. The user can use this knowledge foundation to model their own things.

Suppose all knowledge stored is versioned. That means that the database exposes an interface to checkout past versions/snapshots/revisions of the database. (I think _revisions_ is the best word for it.) There must be some mechanism to create new revisions, to commit transactions, and manage them. So there is associated knowledge stored with these revisions. But this imposes a static, unchangeable, intrinsic model of history keeping on the user and thus on the database (reason 1).

Since this _is_ in fact knowledge, it must be also versioned — and who is gonna version the versions (reason 2).

Note, that revisions may be used internally as an implementation detail to support MVCC patterns. Readers which query knowledge may and must receive outdated knowledge if during their read a writer commits a new revision of the database. The leading question may thus ask whether to expose these revisions via a public interface. If they are not exposed, as soon as revisions are not used by any reader, they are **disposed** and these slots may be overridden with new revisions.

Reasons 1 and 2 against public revisions may strongly encourage a non-revisioned approach. But one thing is not to forget: since one of Everything's goals is to replace file systems and version control systems, the user of the database must be able to _implement_ and _model_ the desired behaviour.

With this approach, we move the versioning up one level — to the user. This means that the user is responsible for implementing and maintaining a model (suitable for them) which models version control for the kind of data they intend to store. In other words, the user has to show interest in versioning. It is the user's responsibility to care.

## Example

Let Alice be an employee at Walmart. The user stores this fact in the database and models employment (for this usecase) to be singular. Now Alice works at Subway. The user then has to decide themself how they handle this situation. One possibility would be just to override. This may or may not be desireable. Another would be to create a log of where Alice has worked over time, but this is a model the user has to create.

---

Concluding, there should not be any intrinsic versioning going on. It should be the user's job to model it.

# Should child expressions of nodes required to be nodes?

# Should we only allow to state things about abstract objects in the root set of statements of knowledge?

Currently, it is allowed to state things about composite objects as the subject of the statement. Let this be model A. Let disallowing composite objects as subjects in statements be model B.

Here as the pros and the cons for model A.

## Pros

* We are even more flexible in what we can state. We can annotate composite objects and put them into relation with other objects.

* Infinine composite objects are possible: `({$LIST_ITEM: 0}, $LIST_TAIL, {$LIST_ITEM: 0})`. This example is an infinite list of zeroes.

* Polyglots are possible, i.e. objects that satisfy multiple (normally disjoint) definitions.

## Cons

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
