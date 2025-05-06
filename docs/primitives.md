# Values

Everything has many kinds of values. These are the primitives of Everything's model. A value can be:

| Value Name         | Type Description                                                                                                                                          | Rust Type                                             | TypeScript Type                |
|--------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------|-------------------------------------------------------|--------------------------------|
| _Integer_          | Signed 64-bit integer.                                                                                                                                    | `i64`                                                 | `bigint`                       |
| _Float_            | IEEE 754 64-bit floating point number.                                                                                                                    | `f64`                                                 | `number`                       |
| _Character_        | An unsigned 32-bit integer, describing a unicode code point value.                                                                                        | `char`                                                | `number`                       |
| _Duration_         | A signed 120-bit integer, describing a time duration in nanoseconds.                                                                                      | `everything::values::Duration` (`i128`)               | `bigint`                       |
| _DateTime_         | A signed 120-bit integer, describing the number of nanoseconds after 1st of January 1970 (UNIX time).                                                     | `everything::values::DateTime` (`i128`)               | `bitint`                       |
| _ObjectReference_  | An unsigned 64-bit integer, an object id if positive, else it indicates an absence of an object (null value).                                             | `Option<everything::ObjectId>` (`Option<NonZeroU64>`) | `bigint`                       |
| _Language_         | An unsigned 16-bit integer, the language id. Only a set of values is allowed. [List of language ids](./languages.txt).                                    | `everything::values::Language` (`u16`)                | `number`                       |
| _URL_              | Text matching [the URL regex](#the-url-regex).                                                                                                            | `everything::values::Url` (`str`)                     | `string` (?)                   |
| _Color_            | A color consisting of one u8 being the color space id (range limited, see [colors](#colors)) and three 32-bit slots whose meaning is dependent on the id. | `everything::values::Color` (`(u8, u32, u32, u32)`)   | `client/Color`                 |
| _Schema_           | TODO                                                                                                                                                      |                                                       |                                |
| _Constraint_       | TODO                                                                                                                                                      |                                                       |                                |
| _Email_            | Text matching [the email regex](#the-email-regex).                                                                                                        | `everything::values::Email` (`str`)                   | `string` (?)                   |
| _Text_             | Text.                                                                                                                                                     | `str`                                                 | `string`                       |
| _Binary_           | Binary data.                                                                                                                                              | `[u8]`                                                | `ArrayBuffer`                  |
| _Encrypted Email_  | [Encrypted](#encrypted-values) text matching [the email regex](#the-email-regex).                                                                         | `[u8]` -> `everything::values::Email` (`str`)         | `ArrayBuffer` -> `string` (?)  |
| _Encrypted Text_   | [Encrypted](#encrypted-values) text.                                                                                                                      | `[u8]` -> `str`                                       | `ArrayBuffer` -> `string`      |
| _Encrypted Binary_ | [Encrypted](#encrypted-values) binary data.                                                                                                               | `[u8]` -> `[u8]`                                      | `ArrayBuffer` -> `ArrayBuffer` |

## Encrypted Values

Encrypted values have their contents encrypted with AES-256-GCM using the target users' public key.
The target user then can decrypt the value using their private key.

## The URL Regex

```ts
/^\b((?:https?|ftp):\/\/(?:\S+(?::\S*)?@)?(?:[A-Za-z0-9.-]+|\[[A-Fa-f0-9:]+\])(?::\d+)?(?:\/[^\s]*)?)\b$/g
```

## The Email Regex

```ts
/^([A-Z0-9_+-]+\.?)*[A-Z0-9_+-]@([A-Z0-9][A-Z0-9-]*\.)+[A-Z]{2,}$/
```

## Colors

## Planned Values

* Phone numbers