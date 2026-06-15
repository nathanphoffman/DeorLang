# Collections

## Declaring a List

`list` is the only collection type. Its element type is always provided with `using shape` on the continuation line. The transpiler marks the binding `mut` automatically when the list is mutated.

```
list result = []
    using shape int

list rooms = [kitchen, office, bedroom]
    using shape Room
```

```rust
let mut result: Vec<i32> = Vec::new();
let rooms = vec![kitchen.clone(), office.clone(), bedroom.clone()];
```

When a list is the return value of a function with a known concrete element type, the shape is inferred from the return type — no `using shape` required at the assignment:

```
[shape: Room]
fn Room list occupied_rooms(Room list rooms)
    ...

list occ = occupied_rooms(rooms)    # shape inferred from fn return type
```

---

## Index Read

Elements are read by index using bracket notation. Zero-indexed, matching Rust's behavior.

```
list scores = [10, 20, 30, 40]
    using shape int
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

In inline type positions — function parameters, return types, and struct fields — the element type is written as a prefix before `list`. No `using shape` continuation line is needed because the type is already explicit in-place.

```
fn int total_area(Room list rooms)
    ...

fn Room list occupied_rooms(Room list rooms)
    ...

struct House
    string address
    Room list rooms
```

```rust
fn total_area(rooms: &Vec<Room>) -> i32 { ... }
fn occupied_rooms(rooms: &Vec<Room>) -> Vec<Room> { ... }

struct House {
    address: String,
    rooms: Vec<Room>,
}
```

The pattern is consistent with all other Deor type declarations: `Type name`. A list field `Room list rooms` reads "a list of Room named rooms."

---

## `bytes` vs `list`

Raw binary data uses `bytes` (`Vec<u8>`), not `list` with `using shape int`. A list of int is `Vec<i32>` — wrong width for byte manipulation and incompatible with APIs expecting `&[u8]`.

```
bytes data = read_raw("file.bin")    # correct — raw binary

list scores = [10, 20, 30]          # correct — integer list
    using shape int
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
