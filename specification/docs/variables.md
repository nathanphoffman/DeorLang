# Variables

## `as` — Literal-Derived Bindings (Compile-Time Only)

`as` derives a binding's type from a literal value. It cannot be used with runtime expressions.

```
sum as 0
room_list as [kitchen, office, bedroom]
```

```rust
let sum = 0;
let room_list = vec![kitchen.clone(), office.clone(), bedroom.clone()];
```

**Conversion notes:** if `Room` isn't already declared via `struct`, the transpiler must **synthesize a matching struct definition** from the literal's shape — this is the core "derive a type from data" feature inherited from TS's `as const`. String literals become `.to_string()` (owned) or `&str` (borrowed) depending on how the binding is used downstream.

### Struct Literals — Shorthand Only

Struct literals use shorthand syntax exclusively. Every field must already be a variable in scope, and the variable name must match the struct field name exactly. There is no `field: value` pair syntax.

```
area as 9
name as "Office"
occupied as true
room as {area, name, occupied}
```

```rust
let area = Squarefeet::new(9);
let name = "Office".to_string();
let occupied = true;
let room = Room { area, name, occupied };
```

If you need a field name that differs from the variable you have, rename it first:

```
name in other_room
label = name
entry as {label}
```

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
