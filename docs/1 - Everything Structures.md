# Chapter 1 - Everything Structures

This is the documentation for the structure that the _Everything universal data model_ is working with.

## Objects, Structures, And Properties

First of all, some defintions.

An _object_ is either an abstract object or a structure. An _abstract object_ is an abstract identifier which is just a 128 bit unsigned integer. A _structure_ is a set of properties. A _property_ is a pair of the form `(tag, value)`, both objects.

### Notes

Conceptually, abstract objects are just global "labels" for some sort of abstract object (out of the digital domain). They allow for talking about things regardless of their properties. **The integer identifier of abstract objects MUST NOT carry ANY semantic meaning.** Because, if the meaning would change over time as knowledge evolves, the identifier would be unfitting for the abstract object.

Because abstract objects are **unambiguosly** identified by the integer, **they ARE the integer identifier**. It is STRONGLY recommended to use some combination of time and randomness to generate abstract objects, for example ULIDs. That makes them virtually universally unique.

> [!TIP]
> Use the CLI to generate new abstract objects which uses ULIDs under the hood:
> 
> ```sh
> everything_cli gen
> ```

Structures derive their semantic meaning through the properties they are composed of.

## Notation / File Format

Abstract objects are written with an `@`. Examples include `@43`, `@4538503485`, and `@2`. Structures are written this way:

```
{(<<OBJECT>>, <<OBJECT>>), (<<OBJECT>>, <<OBJECT>>) ...}
```

Here `<<OBJECT>>` represents a placeholder for any object. Properties in structures may also have a separating/trailing comma.

## Example Structures

```struct
{(@1, {(@534, @3)}), (@4, {})}
```

---

That is it. That is the whole structures format.
