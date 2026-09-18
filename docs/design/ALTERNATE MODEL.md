# Alternate Model For Everything

This just occurred to me.

## Example 1 (with _has_)

Statement:

> David has an age of 42.

In this statement,

- _David_ is an abstract object,
- _has_ associates David with an _age_, and
- the age associated has the value _42_.

In the current model, we have to choose to

- either encode _age_ into the association (_has age_) or
- encode it into the associated value (_age 42_).

So this lowers down to either

- `(David, has_age, 42)` or
- `(David, has, {age: 42})`.

The second option is uncommon because it moves the interpretation of associated values from the association into the values themselves.

## Example 2 (with _is_)

Consider this statement:

> David is a person.

In this statement,

- _David_ is an abstract object and
- David is (associated with) being a person.

There are, again, two ways of lowering this. We have to choose

- either to make _being a person_ an abstract object and associate it with David or
- to treat _person_ as a domain, David is part of.

This lowers down to

- either `(David, is_person, {})` or
- to `(David, is, person)`.

The second variant is uncommon because it moves the property of being a person (which is really a tag) into the value of the association.

## Question 1

> Are there statements (about abstract objects) that cannot be broken down like this, i.e., reduced to _is_ or _has_?

Even _has_ could be encoded into _is_ via a property object _having_ X. David is "having an age of 42". If question 1 turns out to be true, Everything's model could simplify even more, going from three objects per statement to two: subject and property.

## Problem 1

All tags are either of the form `HAS_*` or `IS_*`. Maybe even reducing `HAS_*` to `IS_HAVING_*`. This means that every tag in a statement just declares an "is" relationship between an abstract object and a value.

For example, to encode colors and relationships with their subjects, you have to

- define an `is_color` tag (to denote that an object is a color),
- define colors `(Red, is_color, {})`, `(Blue, is_color, {})`, ...,
- and define a `has_color` tag (to denote that an object has a color).

Then you may state `(Shirt, has_color, Red)`.

But better would be to just

- define `color` as some sort of abstract property
- and use it like `(Shirt, Red)`.

This would be interpreted as _shirt_ is _red_.

## Problem 2

Relationships and properties that require no additional data are akwardly implemented (where you may choose additional data in statements which is ignored). In other words, you have to provide a value in statements even if the semantics of the tag do not require it. The tag then _should_ check for the value in its constraint and validate that it is equal to the empty object.

Yes, one could just not require a value on a statement but the constraint still needs to be called with _some_ value (the empty object) which does not make the (required) value disappear. The value should not even be conceptually present in the relation.

I should be able to state `(David, Person)`, `(Red, Color)`, and `(Shirt, Red)`. (With an implicit "is" as the comma.)

## Bottom line

Maybe we could merge tag and value of a statement into one thing, a _property_. Then the knowledge reduces to abstract objects, having a set of properties.

- Some properties may be unit properties, like `Person` or `Color` which do not require extra additional data.
- And all other properties may be "constructed" via a tag and a value, such as `(Age, 42)`. But I don't know yet how to implement this in the model. Maybe we need secondary things.
