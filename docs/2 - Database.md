# 2 - Database

## Overview

A database consists of a set of statements. Statements are triplets of the form `(subject, tag, value)`, all objects. Statements found in the database are assumed true by the database. They state something a property `tag` with a value `value` about the `subject` object.

You can view the database as a graph with nodes being objects and the arrows connecting the subject via the tag to the value.

## Working Through An Example

Let's try to store and model parent-child relationships in a database. For that we need some example objects: Let `$Alice` and `$Bob` be abstract objects. Also, we need an abstract object `$IsParentOf` representing the relationship. Then we can store the statement `($Alice, $IsParentOf, $Bob)` in the database. (We could also have chosen an equivalent but contradirectional `$IsChildOf` relation; it all depends on how you model your data.)

But this statement alone is not enough. We need to tell the database that `IsParentOf` _can be used_ as a tag. For that we need a second statement: `($IsParentOf, $AXIOMATIC, {$CONTAINS: {}})`. Here we state that our object is axiomatic (i.e. can be used as a tag) with an associated value `{$CONTAINS: {}}`. This value is  just means _true_.

In this example, the value is just _true_. But in practise, this value is more powerful than that. This is a constraint that decides what relationsships are allowed on the tag. For example, we could modify our model to include a tag `$Person` and require objects `$A` and `$B` in every use of `$IsParentOf`, `($A, $IsParentOf, $B)`, to be people, i.e. to have the tag `$Person`. These constraints can be validated with the constraint value on `$Axiomatic` when declaring an object as a tag. First, the value gets called with the subject, and then the result gets called with the value of each use of the tag.

```
# Person is axiomatic and is valid for everything.
($Person, $AXIOMATIC, {$CONTAINS: {}})

# You can only use $IsParentOf with people.
($IsParentOf, $AXIOMATIC, function subject =>
    function value =>
        query (subject, $Person, ?)
        and query (value, $Person, ?))

# Alice and Bob are people
($Alice, $Person, {})
($Bob, $Person, {})

# Allowed
($Alice, $IsParentOf, $Bob)

# Not allowed because sky is not a person
($Sky, $IsParentOf, $Bob)
```

## Implementation In The Everything Data Model

In the next chapter, the Everything Data Model will be layed out. Statements map to composite objects and databases map to sets which are also implemented using composites.
