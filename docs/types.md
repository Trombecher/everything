# Types And Values

Cells annotated with (?) are subject to change.

| ID | Name         | Description                                                                                                       | SQLite Column Type | JS Raw (=Backing) Value                 |
|----|--------------|-------------------------------------------------------------------------------------------------------------------|--------------------|-----------------------------------------|
| 1  | `ObjectId`   | An object ID, guaranteed to be valid.                                                                             | `INTEGER`          | `number`                                |
| 2  | `Decimal`    | A 64-bit floating point number.                                                                                   | `REAL`             | `number`                                |
| 3  | `Integer`    | A 64-bit integer.                                                                                                 | `INTEGER`          | `number`                                |
| 4  | `String`     | A string of characters.                                                                                           | `TEXT`             | `string`                                |
| 5  | `Duration`   | A duration of time, stored as a signed 64-bit integer describing the elapsed milliseconds.                        | `INTEGER`          | `number`                                |
| 6  | `DateTime`   | An absolute time stamp, stored as a signed 64-bit integer describing the elapsed milliseconds after Jan. 1st 1970 | `INTEGER`          | `number`                                |
| 7  | `Boolean`    | `true` or `false`.                                                                                                | `INTEGER`          | `boolean`                               |
| 8  | `Character`  | A single unicode code point.                                                                                      | `INTEGER`          | `number`                                |
| 9  | `URL`        | A string but matching the URL scheme.                                                                             | `TEXT`             | `string`                                |
| 10 | `Binary`     | A binary piece of data, a Blob.                                                                                   | `BLOB`             | `Blob` or `ArrayBuffer` or `Uint8Array` |
| 11 | `Color`      | A CSS color.                                                                                                      | `TEXT`             | `string` (?)                            |
| 12 | `Email`      | A string that matches the email scheme.                                                                           | `TEXT`             | `string`                                |
| 13 | `Language`   | A language locale stored an integer.                                                                              | `TEXT`             | `number`                                |
| 14 | `Schema`     | A tag schema.                                                                                                     | `TEXT` (?)         | `string` (?)                            |
| 15 | `Constraint` | A constraint.                                                                                                     | `TEXT` (?)         | `ArrayBuffer` (?)                       |