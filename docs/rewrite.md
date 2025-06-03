`(ObjectId, TagId, Value)`

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