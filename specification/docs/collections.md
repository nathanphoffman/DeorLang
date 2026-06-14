# Collections

## `List<T>` — Dynamic (Vec)

A `List<T>` with no size is heap-allocated and growable. The transpiler marks the binding `mut` automatically based on usage.

```
List<int> result = []
result append 4
```

```rust
let mut result: Vec<i32> = Vec::new();
result.push(4);
```

---

## `List<T, N>` — Fixed-Size (Array)

A `List<T, N>` with a size is stack-allocated and fixed. `append` is a compile-time error on fixed arrays.

```
List<int, 4> scores = [10, 20, 30, 40]
scores[0] = 15
```

```rust
let mut scores: [i32; 4] = [10, 20, 30, 40];
scores[0] = 15;
```

**Conversion notes:**
- `List<T, N>` → `[T; N]`: supports **index-assignment** (element mutation), but `append` is a **compile-time error** — fixed arrays can't grow.
- A `List<T, N>` field contributes a known size (`N * sizeof(T)`) toward a struct's size cap and `struct+`/auto-value eligibility. An unsized `List<T>` field does not.

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
