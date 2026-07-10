<!-- title: Deor Specification -->
<!-- [Deor Specification Index](index.md) -->
<!-- themes: blackboard -->
# Destructuring
`in` extracts one or more fields from a struct into the current scope. This is the only way to access struct fields — there is no dot syntax in source, so every field a block of code touches is named up front in one place, instead of scattered across dot-chains wherever they happen to get used.

Parentheses are always required, even for a single field. Deor calls this **bagging** for readability: you show the dumping of the bag (in this case) and the 
putting items into the bag (for construction, not shown here). Just like the real world, objects that fall out of a bag are still themselves hence 
why there is no aliasing, you get it as the field name that it existed as inside the bag.

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

**Shadowing is scoped to the block it happens in.** Destructuring inside an `if`, loop, or other nested block only shadows for the rest of that block — it does not change the outer variable. Once the block ends, the outer name is back to its original value.

Deor:
```deor
name as "Alice"
if some_condition
    (name) in employee    # shadows name, but only inside this if-block
    print(name)            # employee.name
print(name)                 # "Alice" — unaffected by the block above
```

Rust:
```rust
let name = "Alice".to_string();
if some_condition {
    let name = employee.name.clone();  // scoped to this block
    println!("{}", name);
}
println!("{}", name);  // "Alice"
```

This is normal Rust `let` scoping, not special Deor behavior — a destructure never reaches out to reassign a variable declared in an outer scope.

A **further thought** on shadowing: Deor does not shy away from ```macros```, as a result shadowing is necessary to prevent collision, Deor also 
accepts its Rust lineage and since Rust supports this it seemed like an obvious feature to keep. This also aligns with the default import philosophy 
which is to silently swallow same-imports as they are already imported. If you don't want to silently swallow a duplicate definition, the best approach 
is to use a ```const```, as they will not allow shadowing.

---

## Move Destructuring
`move (f1, f2) in source` extracts fields without cloning — each binding takes ownership of the field instead of copying it. `source` cannot be used afterward for any field that was moved out. See [Move](docs/move.md#destructuring) for details.

Deor:
```deor
move (label, points) in score
# can't do `(something_else) in score` << will error
```

Rust:
```rust
let label = score.label;
let points = score.points;
```