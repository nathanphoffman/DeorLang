# Language Specification (Draft)

A small, indentation-based language that transpiles to Rust. Core influences: TypeScript's literal-derived typing (`as const`), Python's indentation and `for x in y`, and Go/C's prefix type declarations (`Type name`).

## Core Principles

- **No dots.** Field access is via destructuring (`area in room`), not `.field`.
- **No colons for blocks.** Indentation alone opens a block after a header keyword (`fn`, `if`, `for`, `type`, `struct`).
- **One statement per line.** Multi-line expressions only wrap inside `()`, `[]`, or `{}`.
- **`as`** = "derive this binding's type/shape from a literal" (compile-time only).
- **`in`** = "extract something from a source" — struct fields, collection elements, or module contents, all one grammar.
- **Structs are immutable.** Primitives and lists are mutable.
- **`==` is always structural**, regardless of how a struct is represented internally.
- **No lambdas.** Only named `fn`s (top-level or nested).

---

## 1. Functions

```
fn int add(int a, int b)
    a + b
```

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

**Conversion notes:** the prefix return type becomes Rust's trailing `-> Type`. The transpiler must ensure the final expression has no trailing `;` for it to act as the return value.

### Multiple return values

```
fn (int, int) divmod(int a, int b)
    a / b, a % b
```

```rust
fn divmod(a: i32, b: i32) -> (i32, i32) {
    (a / b, a % b)
}
```

**Conversion notes:** a bare comma-separated tuple in source becomes a parenthesized tuple literal in Rust.

---

## 2. Return Rules

- If a function body contains **no bindings** (`as`, `=`, or `Type name = expr`), the **tail expression is implicitly returned** — `return` is optional there.
- If the body contains **any binding**, `return` is **mandatory** at every exit, including the tail.
- **Non-tail exits always require `return`**, regardless of bindings.

```
fn int square(int x)
    x * x
```

```rust
fn square(x: i32) -> i32 {
    x * x
}
```

```
fn int abs(int x)
    if x < 0
        return -x
    return x
```

```rust
fn abs(x: i32) -> i32 {
    if x < 0 {
        return -x;
    }
    return x;
}
```

**Conversion notes:** the transpiler can always emit explicit `return` safely — implicit-tail is a source-level convenience, not a Rust requirement. Going the other direction (no binding present → tail expr without `return`) is a pure stylistic choice the transpiler can make either way.

---

## 3. Block Structure (No Colons)

Indentation alone opens a block after `fn`, `if`, `for`, `type`, or `struct`.

```
fn int abs(int x)
    if x < 0
        -x
    else
        x
```

```rust
fn abs(x: i32) -> i32 {
    if x < 0 {
        -x
    } else {
        x
    }
}
```

**Conversion notes:** indentation depth maps directly to brace nesting — a straightforward structural transform with no semantic subtleties.

---

## 4. One Statement Per Line

No continuations except inside delimiters. Long expressions wrap inside `()`/`[]`/`{}`:

```
Connection conn = Connect(
    host,
    port,
    timeout,
)
```

```rust
let conn: Connection = Connect(
    host,
    port,
    timeout,
);
```

**Conversion notes:** trailing commas are encouraged and map directly onto Rust's own trailing-comma convention.

---

## 5. Variable Bindings

### `as` — literal-derived (compile-time only)

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

### Explicit typing — runtime values

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

**Conversion notes:** `as` is reserved for literals; any value that depends on a function call or other runtime computation must use explicit `Type name = expr`. A `List<T> name = []` binding that's later `append`ed must be emitted as `let mut` even though source never writes a mutability marker — the transpiler infers `mut` from usage.

### Reassignment

```
total = total + 1
```

```rust
total += 1;
```

---

## 6. Destructuring & Iteration (`in`)

### Field extraction

```
area in room
(area, name) in room
```

```rust
let area = room.area;
let Room { area, name, .. } = room;
```

**Conversion notes:** parentheses are used for multi-name extraction even though single-name extraction doesn't strictly need them — kept for visual consistency. The generated `.area` access is fine in Rust output even though the *source* language has no dot syntax — "no dots" is a source-grammar rule, not a constraint on generated code.

### Collection iteration

```
for room in rooms
    ...
```

```rust
for room in &rooms {
    ...
}
```

**Conversion notes:** the transpiler chooses `&rooms` (borrow) vs `rooms` (move/copy) based on whether `Room` is `Copy` and whether `rooms` is used again afterward.

### Numeric iteration

```
for i in range(count)
    ...
```

```rust
for i in 0..count {
    ...
}
```

**Conversion notes:** `range(n)` is a **builtin function**, not new syntax — it transpiles to Rust's `0..n` range expression. This keeps the source grammar free of additional punctuation (no `..` operator needed).

---

## 7. Imports (`in`)

```
(sqrt, floor, rand) in math

(trim, split) in strings

(
    Room,
    House,
    Squarefeet,
    total_area,
    occupied_rooms,
) in "./models"

geo in "./geometry"
(distance, midpoint) in geo

(format_address as fmt_addr) in "./utils"
```

```rust
use my_crate::models::{Room, House, Squarefeet, total_area, occupied_rooms};
use my_crate::geometry::{self as geo, distance, midpoint};
use my_crate::utils::format_address as fmt_addr;
// (sqrt, floor, rand) resolve to whatever crate/std path the
// stdlib-equivalence table maps `math` to
```

**Conversion notes:**
- **Bare identifier source** (`math`, `strings`) → external crate or `std` module path, via a curated stdlib-equivalence table maintained by the transpiler.
- **String path source** (`"./models"`) → local module path, resolved relative to the current file and translated into Rust's `crate::`/`super::` system. The transpiler must also emit the corresponding `mod` declarations.
- **`Name as alias in source`** maps almost exactly onto Rust's `use path::Name as alias;` — one of the cleanest 1:1 conversions in the spec.
- **Two-step imports** (`geo in "./geometry"` then `(distance, midpoint) in geo`) — if the intermediate `geo` namespace binding is never used directly, the transpiler can drop it and emit only the destructured `use` statements.

---

## 8. Validator Types (`type`)

A `type` definition wraps a base type with a predicate. The body is an implicit `bool` expression over the parameter.

```
type Squarefeet(int n)
    n >= 0 and sqrt(n) == floor(sqrt(n))
```

```rust
#[derive(Clone, Copy, PartialEq, Debug)]
struct Squarefeet(i32);

impl Squarefeet {
    fn new(n: i32) -> Self {
        assert!(n >= 0 && (n as f64).sqrt().fract() == 0.0, "invalid Squarefeet: {}", n);
        Squarefeet(n)
    }
}
```

```
Squarefeet area = 9
```

```rust
let area = Squarefeet::new(9);
```

**Conversion notes:**
- This is a textbook Rust **newtype + smart constructor** — arguably more idiomatic in Rust than in any other target.
- **Compile-time-constant arguments** (`9`) should be validated at transpile time where possible — an invalid *literal* becomes a transpile error, not a runtime panic.
- **Dynamic values** always route through `Squarefeet::new(...)`, which panics on failure.
- `and` / `or` / `not` map to `&&` / `||` / `!`.
- For arithmetic between a validator type and its base type (e.g., `sum + area` where `sum: int`, `area: Squarefeet`), the transpiler unwraps via the generated tuple field (`sum += area.0`) — again, this `.0` is generated code, not source syntax, so it doesn't violate the no-dots rule.

---

## 9. Structs (`struct` / `struct+` / `struct*`)

```
struct Room
    Squarefeet area
    string name
    bool occupied
```

```rust
#[derive(Clone, PartialEq, Debug)]
struct Room {
    area: Squarefeet,
    name: String,
    occupied: bool,
}
```

### Representation: `struct`, `struct+`, `struct*`

| Form | Meaning | Rust representation |
|---|---|---|
| `struct Name` | Transpiler decides | `Name` (value) or `Rc<Name>` (reference), based on size + whether any field is an unsized `List<T>` |
| `struct+ Name` | Force value, always | `Name`, `.clone()` is a full (possibly deep) copy |
| `struct* Name` | Force reference, always | `Rc<Name>`, `.clone()` is a refcount bump |

```
struct House
    string address
    List<Room> rooms       # unsized List -> auto becomes struct*

struct+ House               # explicit override: always a value, full clone on copy
    string address
    List<Room> rooms
```

**Conversion notes:**
- The **struct definition itself is identical** regardless of `+`/`*`/auto — only how *usages* are represented changes (`House` vs `Rc<House>`).
- An **unsized `List<T>` field** makes a struct's clone cost O(n) and unbounded, so it defaults to `*` (reference) unless overridden with `+`.
- `==` is always `#[derive(PartialEq)]` on the underlying struct. `Rc<T>`'s default `PartialEq` already delegates to `T`'s impl in Rust, so structural equality holds for `struct*` types **with no extra work** — a happy existing alignment.
- A struct containing only primitives and/or `List<T, N>` (fixed-size) fields has a fully known size and is `Copy`-eligible if every field is `Copy`.

---

## 10. Lists (`List<T>` / `List<T, N>`)

### Dynamic (`Vec<T>`)

```
List<int> result = []
result append 4
```

```rust
let mut result: Vec<i32> = Vec::new();
result.push(4);
```

### Fixed-size (`[T; N]`)

```
List<int, 4> scores = [10, 20, 30, 40]
scores[0] = 15
```

```rust
let mut scores: [i32; 4] = [10, 20, 30, 40];
scores[0] = 15;
```

**Conversion notes:**
- `List<T>` (no size) → `Vec<T>`: heap-allocated, growable, `append` → `.push()`. The transpiler marks the binding `mut` automatically based on usage.
- `List<T, N>` → `[T; N]`: stack-allocated, fixed, `Copy` if `T: Copy`. Supports **index-assignment** (element mutation), but `append`/`push` is a **compile-time error** — fixed arrays can't grow.
- A `List<T, N>` field contributes a known size (`N * sizeof(T)`) toward a struct's size cap and `struct+`/auto-value eligibility. An unsized `List<T>` field does not — see Section 9.

---

## 11. Mutation Verbs

```
result append n
```

```rust
result.push(n);
```

**Conversion notes:** `append` is currently the only verb-keyword mutation. Any future verbs (e.g., for pop/remove) would follow the same "verb → `Vec` method" pattern.

---

## 12. Immutability & Equality

| Kind | Mutability | Notes |
|---|---|---|
| Primitives (`int`, `float`, `bool`, ...) | Mutable value types | `x = x + 1` always legal |
| `struct` / `struct+` / `struct*` | **Immutable** | No field-assignment syntax exists at all. The only way to get a "changed" struct is `with` (Section 13) |
| `List<T>` / `List<T, N>` | Mutable container | `append` (growable) or index-assignment (fixed); elements may themselves be immutable structs |

`==` is **always structural**, regardless of representation — see the `Rc<T>: PartialEq` note in Section 9.

---

## 13. Record Update (`with`)

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

---

## 14. No Lambdas / Closures

All callable values are named `fn`s — top-level or nested inside another `fn`. There is no anonymous-function syntax.

```
fn List<int> doubled(List<int> nums)
    List<int> result = []
    for n in nums
        result append n * 2
    return result
```

```rust
fn doubled(nums: &Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    for n in nums {
        result.push(n * 2);
    }
    return result;
}
```

**Conversion notes:** avoids `Fn`/`FnMut`/`FnOnce`, closure capture, and capture-related lifetime issues entirely in generated Rust. Nested `fn`s map to Rust's nested `fn` items, which also can't capture outer variables — the same restriction applies in source, so there's no surprise gap.

---

## 15. Full Worked Example

```
(sqrt, floor, rand) in math

type Squarefeet(int n)
    n >= 0 and sqrt(n) == floor(sqrt(n))

struct Room
    Squarefeet area
    string name
    bool occupied

struct House
    string address
    List<Room> rooms

fn int total_area(List<Room> rooms)
    sum as 0
    for room in rooms
        area in room
        sum = sum + area
    return sum

fn List<Room> occupied_rooms(List<Room> rooms)
    List<Room> result = []
    for room in rooms
        occupied in room
        if occupied
            result append room
    return result

fn string random_room_name(List<Room> rooms)
    int idx = rand(0, len(rooms) - 1)
    name in rooms[idx]
    return name

fn main()
    kitchen as {area: 9, name: "Kitchen", occupied: true}
    office as {area: 16, name: "Office", occupied: false}
    bedroom as {area: 25, name: "Bedroom", occupied: true}

    room_list as [kitchen, office, bedroom]
    house as {address: "12 Main St", rooms: room_list}

    rooms in house
    print(total_area(rooms))

    List<Room> occ = occupied_rooms(rooms)
    for room in occ
        name in room
        print(name)

    biggerKitchen as kitchen with area=25

    string pick = random_room_name(rooms)
    print(pick)
```

### Rust translation

```rust
#[derive(Clone, Copy, PartialEq, Debug)]
struct Squarefeet(i32);

impl Squarefeet {
    fn new(n: i32) -> Self {
        assert!(n >= 0 && (n as f64).sqrt().fract() == 0.0, "invalid Squarefeet: {}", n);
        Squarefeet(n)
    }
}

#[derive(Clone, PartialEq, Debug)]
struct Room {
    area: Squarefeet,
    name: String,
    occupied: bool,
}

#[derive(Clone, PartialEq, Debug)]
struct House {
    address: String,
    rooms: Vec<Room>,
}

fn total_area(rooms: &Vec<Room>) -> i32 {
    let mut sum = 0;
    for room in rooms {
        let area = room.area;
        sum += area.0;
    }
    return sum;
}

fn occupied_rooms(rooms: &Vec<Room>) -> Vec<Room> {
    let mut result: Vec<Room> = Vec::new();
    for room in rooms {
        let occupied = room.occupied;
        if occupied {
            result.push(room.clone());
        }
    }
    return result;
}

fn random_room_name(rooms: &Vec<Room>) -> String {
    let idx: i32 = rand(0, (rooms.len() as i32) - 1);
    let name = rooms[idx as usize].name.clone();
    return name;
}

fn main() {
    let kitchen = Room { area: Squarefeet::new(9), name: "Kitchen".to_string(), occupied: true };
    let office = Room { area: Squarefeet::new(16), name: "Office".to_string(), occupied: false };
    let bedroom = Room { area: Squarefeet::new(25), name: "Bedroom".to_string(), occupied: true };

    let room_list = vec![kitchen.clone(), office.clone(), bedroom.clone()];
    let house = House { address: "12 Main St".to_string(), rooms: room_list };

    let rooms = house.rooms.clone();
    println!("{}", total_area(&rooms));

    let occ: Vec<Room> = occupied_rooms(&rooms);
    for room in &occ {
        let name = room.name.clone();
        println!("{}", name);
    }

    let bigger_kitchen = Room { area: Squarefeet::new(25), ..kitchen };

    let pick: String = random_room_name(&rooms);
    println!("{}", pick);
}
```

### Notable conversion decisions in this example

- `House` contains an unsized `List<Room>`, so per Section 9 it would normally default to `struct*` (`Rc<House>`). In this particular `main`, `house` isn't shared across multiple owners, so the transpiler may reasonably keep it a plain value here — the heuristic is a default, not an absolute.
- `Room` contains a `String` field, so it can never be `Copy` — only `Clone`. Every place a `Room` is duplicated (`vec![kitchen.clone(), ...]`, `result.push(room.clone())`) needs an explicit `.clone()` in Rust, even though source never writes anything special.
- `rooms[idx]` requires an `as usize` cast, since Rust indexes with `usize` but `idx` is `i32` — the transpiler inserts this cast on every list-index operation.
- `print(...)` → `println!("{}", ...)`. Values that aren't already `Display` (like `Room`) would need `{:?}` and `#[derive(Debug)]` instead — already included above for safety.

---

## Open Questions / Future Work

- **Additional mutation verbs** beyond `append` (pop, remove, insert, etc.) — same "verb → `Vec` method" pattern, just not yet enumerated.
- **Pattern matching / `match`** — listed early as a candidate header keyword but never fully designed.
- **Error handling** — validator types panic on invalid construction; whether user code gets a `Result`-like type for recoverable errors is undecided.
- **String semantics** — `string` currently assumed to map to owned `String` by default, with `&str` used where the transpiler can prove borrowing is sufficient. Not yet stress-tested against real programs.
- **User-defined generics** beyond `List<T>` / `List<T, N>` — no syntax yet for generic `struct`/`fn` declarations.
