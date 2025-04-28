# Types

| Name         | Description                                                   | Rust Backing Value  | JS Raw (=Backing) Value                 |
|--------------|---------------------------------------------------------------|---------------------|-----------------------------------------|
| `object`     | An object ID, guaranteed to be valid.                         | `NonZeroU64`        | `bigint`                                |
| `decimal`    | A 64-bit floating point number.                               | `f64`               | `number`                                |
| `integer`    | A 64-bit integer.                                             | `i64`               | `bigint`                                |
| `text`       | A string of characters.                                       | `str`               | `string`                                |
| `duration`   | A duration of time measured in nanoseconds. May be negative.  | `Duration` (`i128`) | `bigint`                                |
| `datetime`   | A moment in time measured in nanoseconds after Jan. 1st 1970. | `DateTime` (`i128`) | `bigint`                                |
| `character`  | A single unicode code point.                                  | `char`              | `number`                                |
| `url`        | A string but matching the URL scheme.                         | `Url` (`str`)       | `string`                                |
| `binary`     | A binary piece of data, a Blob.                               | `[u8]`              | `Blob` or `ArrayBuffer` or `Uint8Array` |
| `color`      | A CSS color.                                                  | ?                   | `string` (?)                            |
| `email`      | A string that matches the email scheme.                       | `Email` (`str`)     | `string`                                |
| `language`   | An ISO 639-3 language.                                        | `Language` (`u16`)  | `number`                                |
| `schema`     | A tag schema.                                                 | `Schema`            | `string` (?)                            |
| `constraint` | A constraint.                                                 | `Constraint`        | `ArrayBuffer` (?)                       |