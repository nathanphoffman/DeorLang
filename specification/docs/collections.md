# Collections

## `list<T>` — Dynamic (Vec)

A `list<T>` with no size is heap-allocated and growable. The transpiler marks the binding `mut` automatically based on usage.

```
list<int> result = []
result append 4
```

```rust
let mut result: Vec<i32> = Vec::new();
result.push(4);
```

---

## `list<T, N>` — Fixed-Size (Array)

A `list<T, N>` with a size is stack-allocated and fixed. `append` is a compile-time error on fixed arrays.

```
list<int, 4> scores = [10, 20, 30, 40]
scores[0] = 15
```

```rust
let mut scores: [i32; 4] = [10, 20, 30, 40];
scores[0] = 15;
```

**Conversion notes:**
- `list<T, N>` → `[T; N]`: supports **index-assignment** (element mutation), but `append` is a **compile-time error** — fixed arrays can't grow.
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

`append` is the current mutation verb for growable lists.

```
result append n
```

```rust
result.push(n);
```

**Conversion notes:** `append` is currently the only verb-keyword mutation. Any future verbs (e.g., for pop/remove) would follow the same "verb → `Vec` method" pattern.
