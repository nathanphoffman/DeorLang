# Variables

## `as` — Literal-Derived Bindings (Compile-Time Only)

`as` derives a binding's type from a literal value. It cannot be used with runtime expressions.

```
sum as 0
room as {area: 9, name: "Office", occupied: true}
room_list as [kitchen, office, bedroom]
```

```rust
let sum = 0;
let room = Room { area: Squarefeet::new(9), name: "Office".to_string(), occupied: true };
let room_list = vec![kitchen.clone(), office.clone(), bedroom.clone()];
```

**Conversion notes:** if `Room` isn't already declared via `struct`, the transpiler must **synthesize a matching struct definition** from the literal's shape — this is the core "derive a type from data" feature inherited from TS's `as const`. String literals become `.to_string()` (owned) or `&str` (borrowed) depending on how the binding is used downstream.

---

## Explicit Typing — Runtime Values

Any value that depends on a function call or other runtime computation must use `Type name = expr`.

```
int t = rand(1, 10)
string pick = random_room_name(rooms)
List<int> result = []
```

```rust
let t: i32 = rand(1, 10);
let pick: String = random_room_name(&rooms);
let mut result: Vec<i32> = Vec::new();
```

**Conversion notes:** a `List<T> name = []` binding that's later `append`ed must be emitted as `let mut` even though source never writes a mutability marker — the transpiler infers `mut` from usage.

---

## Reassignment

```
total = total + 1
```

```rust
total += 1;
```
