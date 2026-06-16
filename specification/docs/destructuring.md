# Destructuring

`in` extracts one or more fields from a struct into the current scope. This is the only way to access struct fields — there is no dot syntax in source.

Parentheses are always required, even for a single field.

## Single Field

```
(area) in room
```

```rust
let area = room.area.clone();
```

## Multiple Fields

```
(area, name) in room
```

```rust
let area = room.area.clone();
let name = room.name.clone();
```

Each extracted field becomes its own `let field = src.field.clone();` binding — not a Rust pattern destructure.

---

## Partial Extraction

You can extract a subset of a struct's fields. Any combination is valid — the struct may have more fields than you extract.

```
struct Room
    Squarefeet area
    string name
    bool occupied

(name) in room          # valid — ignores area and occupied
(area, occupied) in room  # valid — any subset, but must follow declaration order
```

---

## Extraction Order

Fields must be extracted in declaration order — the same rule as struct construction. The transpiler enforces this; wrong order is a transpiler error. Single-field extraction has no ordering constraint.

```
(area, name) in room    # correct — matches declaration order
(name, area) in room    # transpiler error — area is declared before name
```

See [Enforced Practices — Field Extraction Order](enforced_practices.md#field-extraction-order).

---

## Shadowing

If a name being extracted already exists in scope, the new binding silently shadows it. This is standard Rust `let` rebinding and is intentional in Deor.

```
world as 2
(world) in t    # world now refers to t.world — the 2 is gone
```

Use this deliberately to "update" a name after processing, or avoid it by choosing distinct names.

---

**Conversion notes:** the generated `.field` access is valid in Rust output even though the source language has no dot syntax — "no dots" is a source-grammar rule, not a constraint on generated code.
