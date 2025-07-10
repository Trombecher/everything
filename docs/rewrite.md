# Rewrite Plan

## Indexes

* `(ObjectId, TagId, Value) -> ()` "Does this association exist?"
* `(ObjectId, TagId) -> (Value)` "Iterate over the values of the object-tag pair."
* `(ObjectId, TagId) -> ()` "Does this object-tag exist?"
* `(ObjectId, Value) -> (TagId)` "Iterate over all the tags that are associated with the given object and value."
* `(ObjectId, Value) -> ()` "Does this object have an association with this value?"
* `(TagId, Value) -> (ObjectId)`
* `(TagId, Value) -> ()` "Is this tag ever associated with this value?"
* `(ObjectId) -> (TagId, Value)`
* `(ObjectId) -> (TagId)`
* `(ObjectId) -> (Value)`
* `(ObjectId) -> ()`
* `(TagId) -> (ObjectId, Value)`
* `(TagId) -> (ObjectId)`
* `(TagId) -> (Value)`
* `(TagId) -> ()`
* `(Value) -> (ObjectId, TagId)` 
* `(Value) -> (ObjectId)` "Iterate over all objects that are associated with this value."
* `(Value) -> (TagId)` "Iterate over tags whose associations use this value."
* `(Value) -> ()` "Check if a value is being used."
* `() -> (ObjectId, TagId, Value)` "Iterate over all associations."
* `() -> (ObjectId, TagId)` "Iterate over all object-tag pairs."
* `() -> (ObjectId, Value)` "Iterate over all object-value pairs."
* `() -> (TagId, Value)` "Iterate over all tag-value pairs."
* `() -> (ObjectId)` "Iterate over all objects."
* `() -> (TagId)` "Iterate over all tags."
* `() -> (Value)` "Iterate over all used values."
* `() -> ()` useless

## Levelled Indexes

* `ObjectId -> (TagId -> Value, Value -> TagId)`
* `TagId -> (ObjectId, Value -> ObjectId)`, to iterate over `(ObjectId, Value)` for a `TagId`,
  look up each value in the previous index.
* `Value -> (ObjectId, TagId)`

## Sizes

* `ObjectId` / `TagId`: 64bit integer, non-zero
* `PageId`: 42bit integer, allows for packing into an atomic U128
* `Value`

## Concept

The core concept is that the whole db state will be stored in three root page pointers for the three root B-tree indexes. If the database changes, all nodes that need to change must be reallocated and the root pointers must be updated. Nodes can be reused. Once allocated, every node is immutable.

This concept ensures that readers and writers will never collide. Readers will observe the database as static while writers produce a new database each time. Each node can only be referenced by one parent node. Meaning, once a node is replaced, it can be garbage collected if all current transactions postdate the replacement.

For write-heavy workloads, this approach comes with some performance hits, since for each write up to 20KiB of data must be replaced. To cut down on this, writes are queued and applied in batch when a time limit is hit or the queue is full.