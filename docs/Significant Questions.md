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
