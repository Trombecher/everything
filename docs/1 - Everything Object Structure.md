# Chapter 1 - Everything Object Structure

This is the documentation for the structure of Everything objects which the _Everything universal data model_ is built ontop of.

## Objects

First of all, some defintions.

* An _object_ is either an abstract or composite.
* An _abstract object_ is an abstract identifier which is just a 128 bit non-negative integer.
* A _composite object_ is a set of properties.
* A _property_ is a pair of the form `(tag, value)`, both objects.

### Things & Metaphysical Objects

Living beings talk about things. These things are their domain of discourse. These things are "abstract, metaphysical objects".

Everything defines and provides this (meta-)model to quantify, model, and talk about "abstract, metaphysical" objects. The model provides the ability to create a one-to-one mapping between "abstract, metaphysical" objects and abstract and composite objects. **This mapping must be one-to-one**. The problem of comparing "abstract, metaphysical objects" for equality therefore maps to simple structural equality of Everything objects.

### Abstract Objects

Conceptually, an abstract object represents an "abstract, metaphysical object" **which is not identified by it "abstract" composition**. Most people would agree that things like "the sky", "love", and the screen you are reading this from are abstract. However, text like `Hello!` wouldn't be abstract (but composite; because one could define text are a list of Unicode code points).

The integer value of abstract objects therefore is a unique identifier for the corresponding "abstract, metaphysical object". **The integer value of an abstract object MUST NOT carry ANY semantic meaning.** Because, if the meaning would change over time as knowledge evolves, the identifier would be unfitting for the abstract object.

It is STRONGLY recommended to use some combination of time and randomness to generate abstract objects, for example ULIDs. That makes them virtually universally unique.

> [!TIP]
> Use the CLI to generate new abstract objects which uses ULIDs under the hood:
> 
> ```sh
> everything_cli gen
> ```

### Composite Objects

Composite objects represent all other "abstract, metaphysical objects", which are "composite". These other "abstract, metaphysical objects" are **defined and identified** by their internal structure.

## Notation / File Format

The [Object Text Notation specification](./spec/Object%20Text%20Notation.md) specifies a way to write objects in a classical programming language style syntax (written with text). This notation is used throughout the next chapters.
