# Immutability & Record Update

## Mutability Rules

| Kind | Mutability | Notes |
|---|---|---|
| Primitives (`int`, `float`, `bool`, ...) | Mutable value types | `val = val + 1` always legal |
| `struct` / `struct+` / `struct*` | **Immutable** | No field-assignment syntax exists. The only way to get a "changed" struct is `with` |
| `list` | Mutable container | `insert`/`remove` for growable lists; elements may themselves be immutable structs |

---

## Equality

`==` is **always structural**, regardless of how a struct is represented internally. For `struct*` types (`Rc<T>`), Rust's default `PartialEq` already delegates to `T`'s impl, so structural equality holds with no extra work.

---

## Record Update (`with`)

`with` produces a new struct with one or more fields overridden. The original is unchanged. Each field name must already exist as a variable in scope — the same rule as struct literals.

- Single field: `newRoom as room with (area)` — parens always required
- Multiple fields: `newRoom as room with (area, name)`

```
Squarefeet area = 2
newRoom as room with (area)

area = 20
name as "Bigger Office"
biggerOffice as office with (area, name)
```

```rust
let mut area: Option<Squarefeet> = Squarefeet::new(2);
let new_room = Room { area, ..room };

area = Squarefeet::new(20);
let name = "Bigger Office".to_string();
let bigger_office = Room { area, name, ..office };
```

**Conversion notes:** near 1:1 with Rust's built-in functional record update (`..` spread) syntax. Overridden fields that are validator types route through their constructor like any other assignment to that type. The `with` pattern mirrors `in` destructuring: `in` pulls fields out of a struct, `with` pushes variables into one.
