# Collections

## Declaring a List

Lists are declared using a list shape. Declare the shape at the top of the file, then use the shape name as the type everywhere — in variables, function signatures, and struct fields.

```
shape intList = list of int
shape roomList = list of Room
```

```
intList result = []
roomList rooms = [kitchen, office, bedroom]
```

```rust
let mut result: Vec<i32> = Vec::new();
let rooms: Vec<Room> = vec![kitchen.clone(), office.clone(), bedroom.clone()];
```

The shape name is the type everywhere — no suffix, no continuation line, no prefix notation. See [Shapes](shapes.md) for full shape declaration syntax.

---

## Index Read

Elements are read by index using bracket notation. Zero-indexed, matching Rust's behavior.

```
shape intList = list of int

intList scores = [10, 20, 30, 40]
int first = scores[0]    # 10
int last = scores[3]     # 40
```

```rust
let scores: Vec<i32> = vec![10, 20, 30, 40];
let first: i32 = scores[0];
let last: i32 = scores[3];
```

Index must be an integer variable or literal. Dynamic computed indices are fine:

```
int idx = 2
int mid = scores[idx]    # 30
```

Out-of-bounds access is a runtime panic. The transpiler inserts `as usize` casts on all index operations automatically.

---

## In Function Signatures and Struct Fields

The shape name stands in for the full list type wherever a type is expected:

```
shape roomList = list of Room

fn int total_area(roomList rooms)
    ...

fn roomList occupied_rooms(roomList rooms)
    ...

struct House
    string address
    roomList rooms
```

```rust
fn total_area(rooms: &Vec<Room>) -> i32 { ... }
fn occupied_rooms(rooms: &Vec<Room>) -> Vec<Room> { ... }

struct House {
    address: String,
    rooms: Vec<Room>,
}
```

---

## `bytes` vs `list`

Raw binary data uses `bytes` (`Vec<u8>`), not an `intList`. A list of int is `Vec<i32>` — wrong width for byte manipulation and incompatible with APIs expecting `&[u8]`.

```
bytes data = read_raw("file.bin")    # correct — raw binary

intList scores = [10, 20, 30]        # correct — integer list (requires shape intList = list of int)
```

---

## Mutation Verbs

### `insert` — Add Elements

`insert` without a position adds to the end of the list. `insert` with `at [n]` inserts at a specific index, pushing existing elements back.

```
result insert item                      # add to end
result insert item at [2]               # insert at index 2
result insert (item1, item2) at [2]     # insert both starting at index 2
```

```rust
result.push(item);
result.insert(2, item);
result.insert(2, item1);
result.insert(3, item2);
```

For multi-insert `at [n]`, items are inserted in order from that index — `item1` at `n`, `item2` at `n+1`, etc.

### `remove` — Remove by Position

`remove` takes a list of indices in brackets. The transpiler removes from highest to lowest index to avoid index-shifting errors — the order you write them doesn't matter.

```
result remove [2]           # remove at index 2
result remove [2, 5, 1]     # remove at indices 1, 2, and 5
```

```rust
result.remove(2);
// multi: sorted high-to-low, then each .remove()
result.remove(5);
result.remove(2);
result.remove(1);
```

Brackets are always required, even for a single index.

**Conversion notes:**
- `insert` without `at` → `Vec::push`
- `insert at [n]` → `Vec::insert(n, item)`
- `remove [n]` → `Vec::remove(n)`
- Multi-remove transpiles to multiple `Vec::remove` calls in descending index order

---

## Fixed-Size Lists

Fixed-size lists (Rust arrays `[T; N]`) are a v2 feature. For v1, use a `rust` block when fixed-size stack allocation is required.
