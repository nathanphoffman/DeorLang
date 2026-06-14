# Functions

## Declaration

Return type is written as a prefix before the function name. Parameters follow `Type name` order.

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

## Return Rules

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

**Conversion notes:** the transpiler can always emit explicit `return` safely — implicit-tail is a source-level convenience, not a Rust requirement.

---

## No Lambdas / Closures

All callable values are named `fn`s — top-level or nested inside another `fn`. There is no anonymous-function syntax.

```
fn list<int> doubled(list<int> nums)
    list<int> result = []
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
