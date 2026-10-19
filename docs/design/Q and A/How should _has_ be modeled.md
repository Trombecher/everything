# How should `has` be modeled?

Consider the (natural language) statement "this plane has four wings". How could one model this? I propose

- option A as `this_plane is having_wing_count 4` and
- option B as `this_plane is having {wing_count 4}`.

## Analysis of option B

Option B is semantically closer to the natural language statement (and may model "has" more closely by modeling the "count" as a separate object). However, It may become ambiguous in which "role" the count is now associated with `this_plane`, as the plane may "have" more things. Consider `this_plane has boing`. Yes, `boing` may a manufacturer, but is it the manufacturer of `this_plane` or just the parts supplier? `having` may be too general for this.

Also, option may have poor query performance, as the associated value (4) is one object deeper than in option A.

Also, we maybe should declare an additional property `having_constraint <constraint>` which can impose constraints on things that an object has (depending on what things the object has).

## Analysis of option A

This option is merging wing count and the association into one property (constructor). Though, this may lead to creation of property constructors for every role that things associate with other things. The manufacturer (`manufacturer`) needs one (`has_manufacturer`), the supplier (`parts_supplier`) needs one (`has_supplier`), etc. But maybe that's the cost we have to make.

## Deepseek V4.1 Flash Answer and verdict

### What "has" already means in the model

The question is partly answered by the docs already. `docs/public/2 - Knowledge.md:3` defines a statement as `a is o` and says it means "atom `a` **has** the property `o`". So every knowledge statement is already a "has" assertion. There is no separate possession relation in the object model.

That means the real question is not "how do we model *has*?" but:

> When a subject has an attribute through a role, do we encode the role at the property-constructor level, or do we reify the role-value pair inside a generic "having" container?

So option A and option B differ structurally, but neither introduces a new "has" primitive.

### Analysis of the options

#### Option A: `this_plane is having_wing_count 4`

This is the model's native shape. `having_wing_count` is a `property_constructor`, and the subject is the actual plane when the property is checked (`Knowledge.md:15-19`).

Advantages:

- The constraint receives the real subject and the real value: `subject |-> value |-> (subject is plane and value is integer)`.
- No extra nesting; exact property lookup via `node_atoms_with_property` works directly.
- No set-equality problem; `having_wing_count 4` is a single canonical construction.
- No new ontological category is needed.
- This is consistent with the existing style in the docs: `born`, `child_of`, etc., are used directly, not wrapped in `has_`.

Cost:

- Constructor proliferation. But constructors are just atoms declared in knowledge, so this is a naming/organization cost, not a schema-migration cost.

#### Option B: `this_plane is having {wing_count 4}`

This looks like an association record, but it is still a unary property whose value happens to be a set. That creates several problems.

**1. The inner role loses its subject.**

Under `Knowledge.md:15-19`, a construction `wing_count 4` is a property only relative to a subject: the constraint is called with the external subject. Under `Knowledge.md:23-27`, however, a construction is valid only if its constraint is also satisfied when the construction is its own subject.

So for B to work, `wing_count` would have to be subject-agnostic, or validity would have to be redefined, or `having` would have to reach into the set and re-run each role constructor with `this_plane` as subject. In A, the subject is naturally in scope the whole time. This is the strongest technical objection to B.

There is a related latent inconsistency in the current docs: under the literal validity rule, a property like `child_of Anna` would need `child_of Anna is person` to be true in `K`, which the chapter 2 example does not provide. This should be settled independently, but B makes it much more visible.

**2. B is ambiguous between two designs.**

- *Singleton sets only*: `having {wing_count 4}` and a separate `having {manufacturer boing}`. Then B is just A with an extra wrapper and extra depth; it buys nothing and makes queries deeper.
- *Aggregated sets*: `having {wing_count 4, manufacturer boing}`. Then exact-property lookup fails, because `having {wing_count 4}` is not structurally equal to the larger set. You would need `node_construction_values` (`docs/public/3 - Computation.md:186-195`, currently unimplemented) plus a filter, and the association becomes harder to query.

So B has a "split personality": either redundant or inefficient.

**3. B interacts badly with open/closed definitions.**

If the set is open, extra roles change structural equality and break exact matching. If it is closed, you cannot aggregate. A avoids this dilemma because a construction's structure is exactly constructor + value.

**4. B's "reification" is only apparent.**

The set `{wing_count 4}` cannot itself be the subject of a statement, because subjects must be atoms (`Knowledge.md:3`; see also `docs/design/old model/Disallowing Composite Subjects.md`). So you cannot attach metadata to the association itself, e.g. "the wing count was 4 in 2020" or "this count came from source X", unless you mint an atom for the association. If true reification is the goal, B does not achieve it.

### New issues to consider

1. **Cardinality and uniqueness are not enforceable.** "The plane has four wings" implies a functional or cardinality constraint. Constraints are evaluated per `(subject, value)` pair; they cannot see the subject's other statements. Neither A nor B can enforce "at most one wing count" without a subject-properties query or a global validation pass.

2. **Multiplicity versus identity.** If you model four individual wings, knowledge is a set, so four structurally identical wing objects collapse into one. You need distinct atoms per wing, or a count value. A and B both choose a count, but the question never decides whether "has four wings" means a count, four parts, or an existential claim.

3. **"Has" is not one relation.** Part-whole ("has wings"), ownership ("has a car"), attribute ("has a color"), containment ("has items"), kinship ("has a child"), and measurement ("has a wing count") have different logical forms. A single generic `having` erases those distinctions; role-specific constructors preserve them.

4. **`having_constraint` is under-specified.** It is unclear what object it attaches to, whether it receives the subject, the set, or each role-value pair, and whether it can be changed by adding knowledge. Moving role validation into a dynamic set inspector makes the legal roles less discoverable and validation dependent on mutable knowledge.

5. **There is no subject-to-properties query.** `node_atoms_with_property` goes from property to atoms, but nothing yields "what does this plane have?" This affects A and B; B's nesting only helps once you have already reached the `having` property.

6. **Only atoms can "have".** If the plane is modeled as a composite value, it cannot be a statement subject at all. Option B does not fix this; it only hides it one level deeper. This ties directly into the composite-subject question.

7. **Temporal and provenance qualification.** "Has four wings" may need a time or source. Since sets and constructions cannot be subjects, B's container cannot carry that metadata. A doesn't solve it either, but B gives the impression that it does.

### Preferred option

**Option A, with better naming: use the role directly as the property constructor.**

```text
this_plane is wing_count 4
```

Use `having_wing_count 4` only if you specifically want the "possession" reading in the name; structurally it is the same option. Keep a generic `having` only for cases where you genuinely want an unstructured bag of roles, and in that case define it precisely as either a singleton wrapper or a separate role-constructor category with its own validation rules.

Reason: A preserves the model's core invariant that properties are subject-relative, lets constraints see the real subject and value, avoids the extra depth, keeps exact queries and structural equality simple, and does not require a second kind of "property" hidden inside a set. Option B only becomes attractive if you actually need dynamically extensible role bags, and even then it needs a clearer design than the current sketch: subject-agnostic role constructors, a well-defined `having_constraint`, and query support for nested sets.

## Verdict

Option A is better.
