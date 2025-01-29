# Flora

Flora is a self-describing dynamic relational tag-based database with built-in productivity.

It is designed to replace your file system, file management and file synchronization.

## Composition

Flora consists of:

* A web server (that also hosts the database)
* A web client

## Model

Flora's data model is quite special and very simple, yet flexible enough to let you describe anything. It is
very similar to JavaScript's objects.

The referential base thing in Flora is the _object_. It is just something where you can put tags on. **Tags are special
objects that can be associated with other objects**, making them behave like properties.

A tag can have one associated value. There are 15 types of values. You can read about them [here](./types.md).
A tag cannot host a union of values.

## Technologies

- Server: Rust + Tokio
- Database: SQLite
- Web client: Vite + TypeScript + TailwindCSS + SolidJS
- Transport: a custom protocol over Secure WebSocket