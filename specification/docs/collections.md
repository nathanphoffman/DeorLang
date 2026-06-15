# Collections

## Index Read

Elements are read by index using bracket notation. Zero-indexed, matching Rust's behavior.

```
list<int> scores = [10, 20, 30, 40]
int first = scores[0]    # 10
int last = scores[3]     # 40
```

```rust
let scores: Vec<i32> = vec![10, 20, 30, 40];
let first: i32 = scores[0];
let last: i32 = scores[3];
```

Index must be a concrete integer expression — a variable or literal. Dynamic or computed indices are fine:

```
int idx = 2
int mid = scores[idx]    # 30
```

Out-of-bounds access is a runtime panic (same as Rust). No bounds-checking syntax in v1.

Fixed-size lists (`list<T, N>`) support index read identically.

---

## `list<T>` — Dynamic (Vec)

A `list<T>` with no size is heap-allocated and growable. The transpiler marks the binding `mut` automatically based on usage.

```
list<int> result = []
result insert 4
```

```rust
let mut result: Vec<i32> = Vec::new();
result.push(4);
```

---

## `list<T, N>` — Fixed-Size (Array)

A `list<T, N>` with a size is stack-allocated and fixed. `insert` and `remove` are compile-time errors on fixed arrays — use index-assignment instead.

```
list<int, 4> scores = [10, 20, 30, 40]
scores[0] = 15
```

```rust
let mut scores: [i32; 4] = [10, 20, 30, 40];
scores[0] = 15;
```

**Conversion notes:**
- `list<T, N>` → `[T; N]`: supports **index-assignment** (element mutation), but `insert`/`remove` are **compile-time errors** — fixed arrays can't grow or shrink.
- A `list<T, N>` field contributes a known size (`N * sizeof(T)`) toward a struct's size cap and `struct+`/auto-value eligibility. An unsized `list<T>` field does not.

---

## `bytes` vs `list<int>`

Raw binary data should use `bytes` (`Vec<u8>`), not `list<int>`. `list<int>` is `Vec<i32>` — the wrong width for byte manipulation and incompatible with crate APIs that expect `&[u8]`. Use `bytes` when crossing the Rust interop boundary with binary data; use `list<int>` for collections of integers in Deor logic.

```
bytes data = read_raw("file.bin")   # correct — raw binary
list<int> scores = [10, 20, 30]     # correct — Deor integer list
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

Brackets are always required, even for a single index. `remove` is a compile-time error on fixed-size `list<T, N>`.

**Conversion notes:**
- `insert` without `at` → `Vec::push`
- `insert at [n]` → `Vec::insert(n, item)`
- `remove [n]` → `Vec::remove(n)`
- Multi-remove transpiles to multiple `Vec::remove` calls in descending index order
