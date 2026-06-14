# Immutability & Record Update

## Mutability Rules

| Kind | Mutability | Notes |
|---|---|---|
| Primitives (`int`, `float`, `bool`, ...) | Mutable value types | `x = x + 1` always legal |
| `struct` / `struct+` / `struct*` | **Immutable** | No field-assignment syntax exists. The only way to get a "changed" struct is `with` |
| `List<T>` / `List<T, N>` | Mutable container | `append` (growable) or index-assignment (fixed); elements may themselves be immutable structs |

---

## Equality

`==` is **always structural**, regardless of how a struct is represented internally. For `struct*` types (`Rc<T>`), Rust's default `PartialEq` already delegates to `T`'s impl, so structural equality holds with no extra work.

---

## Record Update (`with`)

`with` produces a new struct with one or more fields overridden. The original is unchanged.

```
newRoom as room with area=2
biggerOffice as office with (area=20, name="Bigger Office")
```

```rust
let new_room = Room { area: Squarefeet::new(2), ..room };
let bigger_office = Room {
    area: Squarefeet::new(20),
    name: "Bigger Office".to_string(),
    ..office
};
```

**Conversion notes:** near 1:1 with Rust's built-in functional record update (`..` spread) syntax — one of the easiest conversions in the entire spec. Overridden fields that are validator types route through their constructor (`Squarefeet::new(20)`) like any other assignment to that type.
