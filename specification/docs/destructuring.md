# Destructuring
`in` extracts one or more fields from a struct into the current scope. This is the only way to access struct fields — there is no dot syntax in source.

Parentheses are always required, even for a single field.

## Single Field
Deor:
```
(area) in room
```
Rust:
```rust
let area = room.area.clone();
```

## Multiple Fields
Deor:
```
(area, name) in room
```
Rust:
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
(area, occupied) in room  # valid — any subset, any order
```

---

## Extraction Order
Fields can be extracted in any order — the names drive the binding, not the position. Any subset of a struct's fields is valid.

```
(area, name) in room    # valid
(name, area) in room    # also valid — same result
```
---

## Shadowing
If a name being extracted already exists in scope, the new binding silently shadows it. This is standard Rust `let` rebinding and is intentional in Deor.

```
world as 2
(world) in tab    # world now refers to tab.world — the 2 is gone
```

Use this deliberately to "update" a name after processing, or avoid it by choosing distinct names.

