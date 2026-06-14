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

### Void Functions

Omitting the return type means the function returns nothing. No `void` keyword.

```
fn greet(string name)
    print(name)
```

```rust
fn greet(name: String) {
    println!("{}", name);
}
```

The entry point `fn main()` follows this same rule — no return type, already correct.

### Multiple return values

Tuple return types must name each element — the names document what each position means and must match what the body returns.

```
fn (int quotient, int remainder) divmod(int a, int b)
    int quotient = a / b
    int remainder = a % b
    return (quotient, remainder)
```

```rust
fn divmod(a: i32, b: i32) -> (i32, i32) {
    (a / b, a % b)
}
```

The body must define variables with exactly the declared return names before returning them. This is consistent with the general rule that everything inside `()` must be named variables already in scope.

**Conversion notes:** the named return types are a Deor-only concept — they don't appear in generated Rust, which uses positional tuple elements.

### Capturing multiple return values

Use `in` to destructure a tuple return. Your chosen names are bound positionally — first declared element goes to the first name you provide, second to the second, and so on.

```
(q, r) in divmod(a, b)    # q = quotient, r = remainder
print(q)
print(r)

(x, y) in divmod(c, d)    # different names for a second call — no conflict
```

```rust
let (q, r) = divmod(a, b);
println!("{}", q);
println!("{}", r);
let (x, y) = divmod(c, d);
```

The declared return names (`quotient`, `remainder`) tell you what each position means. Your capture names are your choice. As with all `in` extractions, this must appear before any logic that uses the bound variables in the same block.

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

## Validator Type Returns

A function whose return type is a validator type may return `None` through its return variable — the caller knows to check. `return none` is a transpiler error; always return a named typed variable.

```
fn Roll find_best(list<Roll> rolls)
    Roll best = none

    for r in rolls
        if r
            best = r

    return best    # may be None if rolls is empty or all None
```

Primitive return types (`fn int`, `fn bool`, etc.) can never be `None`.

---

## `throw`

`throw` is an unrecoverable hard stop — transpiles to `panic!()` in Rust. Takes a string message. Use `[using error_handler]` for recoverable/handled errors instead.

```
fn int divide(int a, int b)
    if b is 0
        throw "division by zero"
    return a / b
```

```rust
fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("division by zero");
    }
    a / b
}
```

`throw` accepts a string only. Struct-based throw (e.g. `throw Error`) is flagged for v2.

---

## No Lambdas / Closures / Nested Functions

All callable values are named top-level `fn`s. There is no anonymous-function syntax, and functions may not be defined inside other function bodies. No built-in `filter`, `map`, or `reduce` — write explicit loops instead.

```
fn list<int> doubled(list<int> nums)
    list<int> result = []
    for n in nums
        result insert n * 2
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

**Conversion notes:** avoids `Fn`/`FnMut`/`FnOnce`, closure capture, and capture-related lifetime issues entirely in generated Rust. All functions are top-level items, which maps cleanly to Rust's own top-level `fn` declarations.

---

## Annotations (`[]`)

Annotations appear on the line(s) immediately above `fn`. Multiple annotations stack.

```
[test]
[deprecated]
fn Roll old_roll(int n)
    Roll r = n
    return r
```

| Annotation | Effect | Rust equivalent |
|---|---|---|
| `[test]` | Marks as a test function | `#[test]` |
| `[deprecated]` | Warns callers at compile time | `#[deprecated]` |
| `[pure]` | Signals no side effects, enables transpiler optimizations | `#[inline]` / optimizer hint |
| `[main]` | Explicit entry point — alternative to naming the function `main` | `#[main]` |
| `[using alias: T->O]` | Declares a behavior injection slot (see below) | hidden `fn` pointer parameter |
| `[using alias: T]` | Same, handler returns nothing | hidden `fn()` parameter |
| `[shape: T]` | Declares a generic type parameter (see below) | `<T>` in Rust |

---

## `[using]` — Behavior Injection

`[using]` declares that a function accepts an externally-provided named function. The annotation, the body, and the call site all use the same concept — only the keyword `using` appears at the call site; the body uses the alias name.

The annotation is **required at the call site** — calling a `[using]`-annotated function without `using fn_name` is a transpiler error.

### Syntax

```
[using alias: InputType->OutputType]    # handler with return value
[using alias: InputType]                # void handler
```

### Example — predicate injection

```
[using predicate: Room->bool]
fn list<Room> filter(list<Room> items)
    list<Room> result = []

    for item in items
        if predicate(item)              # alias called in body
            result insert item

    return result
```

```
filter(rooms) using match_name          # call site provides the function
```

The injected function must match the declared signature — `match_name` must take a `Room` and return `bool`.

### Single-param constraint

All injectable functions take exactly one parameter. If more context is needed, package it into a struct first:

```
struct RoomSearch
    Room room
    string query

[using predicate: RoomSearch->bool]
fn list<Room> filter(list<Room> items, string query)
    list<Room> result = []

    for item in items
        search as (item, query)
        if predicate(search)
            result insert item

    return result

filter(rooms, "Kitchen") using match_by_query
```

### Example — void error handler

`[using alias: T]` with no `->O` means the handler returns nothing. Inline struct construction works inside the alias call — the type is known from the annotation.

```
struct Error
    string message
    string body

[using error_handler: Error]
fn Roll parse_roll(string input)
    Roll result = none

    message as "Parse failed"
    body as input
    error_handler((message, body))      # constructs Error inline, calls handler

    return result
```

```
fn log_error(Error e)
    (message, body) in e
    print(message)
    print(body)

fn panic_error(Error e)
    (message, body) in e
    # hard stop

Roll r = parse_roll("abc") using log_error
Roll r2 = parse_roll("abc") using panic_error
```

**Conversion notes:**
- `[using predicate: Room->bool]` adds a hidden Rust parameter `predicate: fn(Room) -> bool`
- `predicate(item)` in the body calls directly through the fn pointer
- At call site, `using match_name` passes `match_name` as the fn pointer
- Uses Rust `fn` pointers (not closures) — cannot capture environment, consistent with Deor's no-lambda rule

---

## `[shape: T]` — Generics

Generics are intentionally anti-pattern in Deor — the `[shape: T]` annotation makes them visually deliberate. `using shape ConcreteType` is always required at the call site; there is no implicit type inference from arguments.

```
[shape: T]
[using predicate: T->bool]
fn list<T> filter(list<T> items)
    list<T> result = []

    for item in items
        if predicate(item)
            result insert item

    return result
```

### Call site — multi-line `using` syntax

When a function has multiple `using` clauses (shape and/or behavior), they stack on indented lines below the call with no blank lines between them — the same rule as compact ternaries.

```
filter(rooms)
    using shape Room
    using match_name
```

All `using` clauses are required. Missing any is a transpiler error. The `using shape` line always comes first when present.

```rust
fn filter<T: Clone>(items: Vec<T>, predicate: fn(T) -> bool) -> Vec<T> {
    let mut result: Vec<T> = Vec::new();
    for item in items {
        if predicate(item.clone()) {
            result.push(item);
        }
    }
    result
}

// called as:
filter::<Room>(rooms, match_name)
```

**Notes:**
- One `[shape: T]` parameter per function — multiple shape parameters are v2
- `T` can be used anywhere in the function signature: parameters, return type, local variables
- `[shape: T]` stacks with `[using]` — declare both, both required at call site
