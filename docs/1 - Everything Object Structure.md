# Chapter 1 - Everything Object Structure

This is the documentation for the structure of Everything objects which the _Everything universal data model_ is built ontop of.

## Things

Living beings talk about _things_. These things are their domain of discourse.

Everything defines and provides a meta model to quantify, model, and talk about things. It is desireable that objects in this model are in a **one-to-one correspondance** to the things they represent. If this is given, the problem of comparing things for equality therefore maps to simple structural equality of objects.

Suppose a thing _A_ is unidentifiable; with the previous sentence we identified all things that are, supposedly, unidentifiable. Therefore, every thing is identifiable. Everything partitions all things into two classes: things that have no inherent structure and are defined by themselves, and things that have inherent structure.

## Objects

First of all, some defintions.

* An _object_ is either an abstract or composite.
* An _abstract object_ is an abstract identifier which is just a 128 bit non-negative integer.
* A _composite object_ is a set of properties.
* A _property_ is a pair of the form `(tag, value)`, both objects.

### Abstract Objects

Conceptually, an _abstract object_ represents a thing which is _not composed_, i.e. it cannot be identified or defined by its inherent structure. Most people would agree that things like "the sky", "love", and the screen you are reading this from are abstract. However, text like `Hello!` wouldn't be abstract (but composite; because one could define text are a list of Unicode code points).

The integer value of abstract objects is a unique identifier for the corresponding thing. **The integer value of an abstract object MUST NOT carry ANY semantic meaning.** Because, if the meaning would change over time as knowledge evolves, the identifier would be unfitting for the abstract object.

It is STRONGLY recommended to use some combination of time and randomness to generate abstract objects, for example ULIDs. That makes them virtually universally unique.

> [!TIP]
> Use the CLI to generate new abstract objects which uses ULIDs under the hood:
> 
> ```sh
> everything_cli gen
> ```

### Composite Objects

Composite objects represent all other things, those that are are "composite". These things are **defined and solely identified** by their intrinsic structure.

## Notation / File Format

The [Object Text Notation specification](./spec/Object%20Text%20Notation.md) specifies a way to write objects in a classical programming language style syntax (written with text). This notation is used throughout the next chapters.
