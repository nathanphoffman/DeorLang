<!-- title: Deor Specification -->
<!-- [Deor Specification Index](index.md) -->
<!-- themes: dusk -->
# Destructuring
`in` extracts one or more fields from a struct into the current scope. This is the only way to access struct fields — there is no dot syntax in source, so every field a block of code touches is named up front in one place, instead of scattered across dot-chains wherever they happen to get used.

Parentheses are always required, even for a single field.

## Single Field
Deor:
```deor
(area) in room
```
Rust:
```rust
let area = room.area.clone();
```

## Multiple Fields
Deor:
```deor
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

```deor
struct Room
    Squarefeet area
    string name
    bool occupied

(name) in room          # valid — ignores area and occupied
(area, occupied) in room  # valid — any subset, any order
```

---

## Shadowing
If a name being extracted already exists in scope, the new binding silently shadows it. This is standard Rust `let` rebinding and is intentional in Deor.

```deor
name as "Alice"
(name) in employee    # name now refers to employee.name — "Alice" is gone
```

Use this deliberately to "update" a name after processing, or avoid it by choosing distinct names.

---

## Move Destructuring
`move (f1, f2) in source` extracts fields without cloning — each binding takes ownership of the field instead of copying it. `source` cannot be used afterward for any field that was moved out. See [Move](docs/move.md#destructuring) for details.

```deor
move (label, points) in score
```
```rust
let label = score.label;
let points = score.points;
```

