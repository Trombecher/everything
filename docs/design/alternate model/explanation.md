I want to be able to state `(Shirt, Red)`, i.e. "(this) shirt is red". `Shirt` is abstract. But then I must also be able to use/define an object (which is not abstract) that is only _identified_ by that it is red. I.e., I must be able to write `{Red}`, the composite object containing the property `Red`. Let's add `MadeOf` as a property constructor to the shirt:

```
(Shirt, Red)
(Shirt, MadeOf Wool) or (Shirt, {MadeOf Wool})?
```

Here, `MadeOf` should be a _property constructor_, not a property itself. "Using" it with an object yields a property. Analogous, I must be able to write `{Red, MadeOf Wool}` (or `{Red, {MadeOf Wool}}`?). These question marks, the bother me, so lets talk about them. What is `{MadeOf Wool}`? Analogous to `{Red}`, `{MadeOf Wool}` must mean "the object that is solely identified by that it is made of wool". This means `MadeOf Wool` MUST be some kind of primitive expression, a new object type. I think I am going to call it _constructor_ object.

Together with constructor objects, composite objects become effectively just objects that hold values (=sets), so I propose a new object model with these as primitives:

- Abstract objects (as is),
- sets, and
- constructors (form `a b`, both objects).

## Example

Natural language statement to db statement derivation:

```
"David is twenty years old"
= "David has an age of 20"
= "David is having an age of 20"
= "David has age 20"
= (David, has age 20)

or (David, having age 20) with the comma read as 'is'
```

which is the statement. Both `has` and `age` are property constructors.

One could restrict `has` to have an associated object being another `property` and the associated value of `age` being a non-negative integer. This must be possible in the db.
