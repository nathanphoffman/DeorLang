# Shapes

A `shape` is a named type alias for a parameterized or named type. Shapes are the only way to use lists, function types, and named byte buffers in Deor — there is no anonymous inline syntax like `list of Room` outside of a shape declaration.

## Declaration

```
shape roomList = list of Room
shape filterFunc = func of Room to bool
shape requestBody = bytes
shape colorTag = union of Red | Green | Blue
```

Shapes are declared at the top level of a file, after imports and before structs. Four kinds exist: list shapes, func shapes, bytes shapes, and union shapes.

---

## List Shapes

A list shape names a specific element type:

```
shape roomList = list of Room
shape intList = list of int
shape rollList = list of Roll
```

```rust
type RoomList = Vec<Room>;
type IntList = Vec<i32>;
type RollList = Vec<Roll>;
```

List shape variables are declared and used like any other typed variable:

```
roomList result = []
roomList rooms = [kitchen, office, bedroom]
int cnt = len(rooms)
```

```rust
let mut result: Vec<Room> = Vec::new();
let rooms: Vec<Room> = vec![kitchen.clone(), office.clone(), bedroom.clone()];
let cnt: i32 = rooms.len() as i32;
```

In function signatures and struct fields, the shape name stands in for the full type:

```
fn roomList occupied_rooms(roomList rooms)
    ...

struct House
    string address
    roomList rooms
```

```rust
fn occupied_rooms(rooms: &Vec<Room>) -> Vec<Room> { ... }

struct House {
    address: String,
    rooms: Vec<Room>,
}
```

---

## Func Shapes

A func shape names a specific function signature. This is the only way to pass functions as values in Deor — no lambdas, no closures, just named top-level functions matched to a declared signature.

```
shape filterFunc = func of Room to bool     # takes Room, returns bool
shape handlerFunc = func of Error           # takes Error, returns nothing
shape supplierFunc = func to bool           # no input, returns bool
```

```rust
type FilterFunc = fn(Room) -> bool;
type HandlerFunc = fn(Error);
type SupplierFunc = fn() -> bool;
```

### Void forms

When input or output is absent, the corresponding `of`/`to` clause is omitted:

| Form | Meaning |
|---|---|
| `func of Room to bool` | takes Room, returns bool |
| `func of Error` | takes Error, returns nothing |
| `func to bool` | no input, returns bool |

### Passing functions as values

A function matching a func shape can be passed as a regular typed argument. No special syntax at the call site — it is just a named variable of the declared type.

```
shape filterFunc = func of Room to bool

fn roomList filter(roomList items, filterFunc predicate)
    roomList result = []
    for item in items
        if predicate(item)
            result insert item
    return result

fn bool by_name(Room room)
    name in room
    return name is "Kitchen"

filter(rooms, by_name)    # by_name satisfies filterFunc — passed as a regular argument
```

```rust
fn filter(items: &Vec<Room>, predicate: fn(Room) -> bool) -> Vec<Room> {
    let mut result: Vec<Room> = Vec::new();
    for item in items {
        if predicate(item.clone()) {
            result.push(item.clone());
        }
    }
    result
}
```

The named-args rule applies: `by_name` must be a named top-level function, not an inline expression. This is how Deor handles what other languages do with lambdas — the function is named, top-level, and typed through the shape.

### Single-param constraint

Func shapes accept at most one input type and one output type. Multi-input shapes are a transpiler error — bundle context into a struct first:

```
# Transpiler error — multi-input not allowed
shape badFunc = func of (Room, string) to bool

# Correct — bundle into a struct
struct RoomQuery
    Room room
    string query

shape roomQueryFunc = func of RoomQuery to bool
```

### Func shapes in structs

Func shapes cannot be used as struct fields — structs are pure data. A struct carrying a function would be a closure in disguise, which Deor does not allow.

```
# Transpiler error
struct Filter
    roomList items
    filterFunc predicate    # not allowed — func shape as struct field

# Correct — pass the function as a parameter instead
fn roomList apply_filter(roomList items, filterFunc predicate)
    ...
```

---

## Naming Convention

Shape names are camelCase — enforced by the transpiler. By convention (not enforced), the name ends with the shape's kind:

| Kind | Suffix | Examples |
|---|---|---|
| List shapes | `List` | `roomList`, `intList`, `rollList` |
| Func shapes | `Func` | `filterFunc`, `predicateFunc`, `handlerFunc` |
| Bytes shapes | — | `requestBody`, `frameData`, `imageBuffer` |
| Union shapes | `Tag` | `colorTag`, `statusTag`, `directionTag` |

camelCase distinguishes shapes from every other identifier category:
- Primitives and keywords: lowercase (`int`, `list`, `func`, `of`)
- User-defined types: PascalCase (`Room`, `Roll`)
- Variables, functions, fields: snake_case (`room_list`, `filter_func`)

Seeing a camelCase identifier always means: this is a shape.

---

## File Ordering

Shapes must appear after imports and before structs. Enforced by the transpiler — see [Enforced Practices](enforced_practices.md#file-declaration-order).

```
# 1. Imports
(sqrt, floor) in math

# 2. Shapes
shape roomList = list of Room
shape filterFunc = func of Room to bool

# 3. Structs
struct House
    string address
    roomList rooms

# 4. Functions
fn roomList filter(roomList items, filterFunc predicate)
    ...
```

---

## Importing Shapes

Shapes are importable like any other top-level declaration. Co-locating a shape with the functions that use it is idiomatic:

```
# rooms.deor
shape roomList = list of Room
shape filterFunc = func of Room to bool

fn roomList filter(roomList items, filterFunc predicate)
    ...
```

```
# main.deor
(roomList, filterFunc, filter) in rooms
```

---

## Bytes Shapes

A bytes shape gives a semantic name to a raw byte buffer. `bytes` is not parameterized — it is always `Vec<u8>` — so the declaration has no `of` clause.

```
shape requestBody = bytes
shape frameData = bytes
shape imageBuffer = bytes
```

```rust
type RequestBody = Vec<u8>;
type FrameData = Vec<u8>;
type ImageBuffer = Vec<u8>;
```

Bytes shape variables are used exactly like list shapes — as parameter types, return types, struct fields, and variable declarations:

```
shape requestBody = bytes

fn void send(requestBody data)
    ...

struct Request
    string url
    requestBody body
```

`len()` works on bytes shapes. `insert` and `remove` work at the element level (individual `u8` values). For any actual byte-level computation — bit manipulation, encoding, parsing — use a `rust` block; bytes shapes carry the data in and out.

---

## Union Shapes

A union shape defines a closed set of named variants — a tagged union with no associated data per variant. It is the only way to express a discriminated type in Deor.

```
shape colorTag = union of Red | Green | Blue
shape directionTag = union of North | South | East | West
shape statusTag = union of Active | Inactive | Pending
```

```rust
#[derive(Clone, PartialEq, Debug)]
enum ColorTag { Red, Green, Blue }

#[derive(Clone, PartialEq, Debug)]
enum DirectionTag { North, South, East, West }

#[derive(Clone, PartialEq, Debug)]
enum StatusTag { Active, Inactive, Pending }
```

**Variant names are PascalCase.** This is the one context where PascalCase does not mean struct or validator type — union variants are PascalCase because they are type constructors, matching Rust enum convention. The parser distinguishes them by context: after `union of` in a declaration, or as a value assigned to a union-typed variable.

**Assignment:**

```
colorTag color = Red
statusTag current = Pending
```

```rust
let color: ColorTag = ColorTag::Red;
let current: StatusTag = StatusTag::Pending;
```

**Checking variants — `if`/`else if` with `is`:**

Deor has no pattern matching. Check which variant a union variable holds using `is` in an `if`/`else if` chain:

```
if color is Red
    print(msg_red)
else if color is Green
    print(msg_green)
else if color is Blue
    print(msg_blue)
```

```rust
if color == ColorTag::Red {
    println!("{}", msg_red);
} else if color == ColorTag::Green {
    println!("{}", msg_green);
} else if color == ColorTag::Blue {
    println!("{}", msg_blue);
}
```

Exhaustiveness is not enforced in v1 — write an `else` branch as a catch-all if needed. V2 may add exhaustiveness warnings.

**Union shapes in structs and function signatures:**

Union shapes work everywhere list and bytes shapes do — as struct fields, function parameters, and return types:

```
struct Task
    string name
    statusTag status

fn void handle(Task task, statusTag next)
    ...
```

**Tag-only in v1.** Variants carry no associated data. To attach context to a variant, pair the union with a struct:

```
struct Event
    statusTag kind
    string payload
```

Payload variants — where each arm carries its own distinct type — are a v2 consideration. See [V2 — Union Variants with Associated Data](v2.md#union-variants-with-associated-data).

---

## Conversion Notes

| Deor | Rust |
|---|---|
| `shape roomList = list of Room` | `type RoomList = Vec<Room>;` |
| `shape filterFunc = func of Room to bool` | `type FilterFunc = fn(Room) -> bool;` |
| `shape handlerFunc = func of Error` | `type HandlerFunc = fn(Error);` |
| `shape requestBody = bytes` | `type RequestBody = Vec<u8>;` |
| `shape colorTag = union of Red \| Green \| Blue` | `#[derive(Clone, PartialEq, Debug)] enum ColorTag { Red, Green, Blue }` |
| `colorTag color = Red` | `let color: ColorTag = ColorTag::Red;` |
| `roomList result = []` | `let mut result: Vec<Room> = Vec::new();` |
| `filter(rooms, by_name)` | `filter(&rooms, by_name)` |

Func shapes use Rust `fn` pointers, not closures — they cannot capture environment, consistent with Deor's no-lambda rule.
