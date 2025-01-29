# Open Questions

## _Should the model a file to be in multiple directories all at once?_

### 📈 Advantages

Allowing a file to be in multiple trees simultaneously has several advantages:

* Great for user collaboration
* Restricted, isolated views for multiple sessions and/or users

### ❓ Multiple Inheritance

This would imply that the file has to choose which parent's tag it wants to inherit. For example, if file _F_ would be
contained in directory _A_ and _B_, but _A_ is owned by Bob and _B_ is owned by Alice, _who owns F?_

### Solution(s)

* This could be solved using some sort of primary and secondary path entries in the directory hierarchy.
* Or we could remove inheritance of some attributes, but this goes against the mental model, "you own the box,
  but not the contents". If we viewed directories not as containers but as 2D-_views_ of the multidimensional data
  structure, then it would be ok.

### Specifics

AFAIK the only problem tag is `Owner`.

## _Should the model allow singletons?_