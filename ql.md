# Query Language

```
find $obj where ($obj, #200, 35) offset 10 limit 23
```

## Object Constraint

An object constraint is an expression that, when called with an object, evaluates to either `true` or `false`.

```
$obj where ($obj.#200 = 10) & ()
```