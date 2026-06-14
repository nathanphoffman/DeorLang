# Destructuring

`in` extracts one or more fields from a struct into the current scope. This is the only way to access struct fields — there is no dot syntax in source.

## Single Field

```
area in room
```

```rust
let area = room.area;
```

## Multiple Fields

```
(area, name) in room
```

```rust
let Room { area, name, .. } = room;
```

**Conversion notes:** parentheses are used for multi-name extraction even though single-name extraction doesn't strictly need them — kept for visual consistency. The generated `.area` access is fine in Rust output even though the source language has no dot syntax — "no dots" is a source-grammar rule, not a constraint on generated code.
