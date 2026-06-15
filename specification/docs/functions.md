# Functions

## Declaration

Return type is written as a prefix before the function name. Parameters follow `Type name` order.

```
fn int add(int left, int right)
    left + right
```

```rust
fn add(left: i32, right: i32) -> i32 {
    left + right
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

Void functions **cannot contain `return` statements**. Early exit is a transpiler error — all conditional paths must be expressed with `if/else` block structure:

```
# Correct — use if/else to express all paths
fn process(Item list items, bool skip_invalid)
    for item in items
        valid in item
        if skip_invalid and not valid
            # skip — continue to next iteration
        else
            handle(item)
```

```
# Transpiler error — early return not allowed in void functions
fn process(Item list items, bool skip_invalid)
    for item in items
        valid in item
        if skip_invalid and not valid
            return    # not allowed
        handle(item)
```

This is intentional: early returns in void functions are usually a sign the logic should be restructured. Use `if/else` or `continue` inside loops instead.

### Multiple return values

Tuple return types must name each element — the names document what each position means and must match what the body returns.

```
fn (int quotient, int remainder) divmod(int left, int right)
    int quotient = left / right
    int remainder = left % right
    return (quotient, remainder)
```

```rust
fn divmod(left: i32, right: i32) -> (i32, i32) {
    (left / right, left % right)
}
```

The body must define variables with exactly the declared return names before returning them. This is consistent with the general rule that everything inside `()` must be named variables already in scope.

**Conversion notes:** the named return types are a Deor-only concept — they don't appear in generated Rust, which uses positional tuple elements.

### Capturing multiple return values

Use `in` to destructure a tuple return. Your chosen names are bound positionally — first declared element goes to the first name you provide, second to the second, and so on.

```
(quo, rem) in divmod(num, div)    # quo = quotient, rem = remainder
print(quo)
print(rem)

(out, rst) in divmod(val, amt)    # different names for a second call — no conflict
```

```rust
let (quo, rem) = divmod(num, div);
println!("{}", quo);
println!("{}", rem);
let (out, rst) = divmod(val, amt);
```

The declared return names (`quotient`, `remainder`) tell you what each position means. Your capture names are your choice. As with all `in` extractions, this must appear before any logic that uses the bound variables in the same block.

---

## Return Rules

- If a function body contains **no bindings** (`as`, `=`, or `Type name = expr`), the **tail expression is implicitly returned** — `return` is optional there.
- If the body contains **any binding**, `return` is **mandatory** at every exit, including the tail.
- **Non-tail exits always require `return`**, regardless of bindings.

```
fn int square(int val)
    val * val
```

```rust
fn square(val: i32) -> i32 {
    val * val
}
```

```
fn int abs(int val)
    if val < 0
        return -val
    return val
```

```rust
fn abs(val: i32) -> i32 {
    if val < 0 {
        return -val;
    }
    return val;
}
```

**Conversion notes:** the transpiler can always emit explicit `return` safely — implicit-tail is a source-level convenience, not a Rust requirement.

---

## Validator Type Returns

A function whose return type is a validator type may return `None` through its return variable — the caller knows to check. `return none` is a transpiler error; always return a named typed variable.

```
fn Roll find_best(Roll list rolls)
    Roll best = none

    for roll in rolls
        if roll
            best = roll

    return best    # may be None if rolls is empty or all None
```

Primitive return types (`fn int`, `fn bool`, etc.) can never be `None`.

---

## `throw`

`throw` is an unrecoverable hard stop — transpiles to `panic!()` in Rust. Takes a string message. Use `[using error_handler]` for recoverable/handled errors instead.

```
fn int divide(int left, int right)
    if right is 0
        throw "division by zero"
    return left / right
```

```rust
fn divide(left: i32, right: i32) -> i32 {
    if right == 0 {
        panic!("division by zero");
    }
    left / right
}
```

`throw` accepts a string only. Struct-based throw (e.g. `throw Error`) is flagged for v2.

---

## No Lambdas / Closures / Nested Functions

All callable values are named top-level `fn`s. There is no anonymous-function syntax, and functions may not be defined inside other function bodies. No built-in `filter`, `map`, or `reduce` — write explicit loops instead.

```
fn int list doubled(int list nums)
    list result = []
        using shape int
    for num in nums
        result insert num * 2
    return result
```

```rust
fn doubled(nums: &Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    for num in nums {
        result.push(num * 2);
    }
    return result;
}
```

**Conversion notes:** avoids `Fn`/`FnMut`/`FnOnce`, closure capture, and capture-related lifetime issues entirely in generated Rust. All functions are top-level items, which maps cleanly to Rust's own top-level `fn` declarations.

---

## Recursion

Functions may call themselves. Recursion follows the same rules as any other function call — arguments must be named variables in scope, and the return type must match.

```
fn int factorial(int val)
    if val <= 1
        return 1
    int prev = val - 1
    int sub = factorial(prev)
    return val * sub
```

```rust
fn factorial(val: i32) -> i32 {
    if val <= 1 {
        return 1;
    }
    let prev: i32 = val - 1;
    let sub: i32 = factorial(prev);
    return val * sub;
}
```

No tail-call optimization is guaranteed — deep recursion can stack-overflow just as in Rust. For large inputs, prefer iterative loops.

---

## Annotations (`[]`)

Annotations appear on the line(s) immediately above `fn`. Multiple annotations stack.

```
[test]
[deprecated]
fn Roll old_roll(int val)
    Roll roll = val
    return roll
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
fn Room list filter(Room list items)
    list result = []
        using shape Room

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
fn Room list filter(Room list items, string query)
    list result = []
        using shape Room

    for item in items
        search as (item, query)
        if predicate(search)
            result insert item

    return result

filter(rooms, "Kitchen") using match_by_query
```

### Example — void error handler

`[using alias: T]` with no `->O` means the handler returns nothing. The struct must be captured in a named variable before being passed — inline construction at the call site is not allowed, consistent with the named-args rule.

```
struct Error
    string message
    string body

[using error_handler: Error]
fn Roll parse_roll(string input)
    Roll result = none

    message as "Parse failed"
    body as input
    Error err = (message, body)
    error_handler(err)

    return result
```

```
fn log_error(Error err)
    (message, body) in err
    print(message)
    print(body)

fn panic_error(Error err)
    (message, body) in err
    # hard stop

Roll roll = parse_roll("abc") using log_error
Roll roll2 = parse_roll("abc") using panic_error
```

**Conversion notes:**
- `[using predicate: Room->bool]` adds a hidden Rust parameter `predicate: fn(Room) -> bool`
- `predicate(item)` in the body calls directly through the fn pointer
- At call site, `using match_name` passes `match_name` as the fn pointer
- Uses Rust `fn` pointers (not closures) — cannot capture environment, consistent with Deor's no-lambda rule

---

## The Decorator + `using` System

Deor uses one unified mechanism wherever a function needs something provided by its caller: declare the requirement with a `[...]` decorator, fulfill it with `using` at the call site.

Two flavors of the same pattern:

| Decorator | Provides | `using` form |
|---|---|---|
| `[using alias: T->O]` | A named function (first-class function passing) | `using fn_name` |
| `[shape: T]` | A concrete type (generic type parameter) | `using shape ConcreteType` |

Both follow the same contract — the decorator makes the requirement visible; `using` at the call site fulfills it. Both are required; omitting either is a transpiler error.

**This is how Deor passes functions as values.** There are no lambdas, no anonymous functions, no closures. Instead, define a named top-level function and pass it with `using fn_name`. The decorator on the receiving function declares the expected signature; the body uses the alias name to call it.

```
[using transform: int->int]
fn int list apply_all(int list nums)
    list result = []
        using shape int
    for num in nums
        int out = transform(num)
        result insert out
    return result

fn int double(int num)
    return num * 2

apply_all(nums)
    using double
```

`using double` is the equivalent of passing a lambda. The function `double` is named, top-level, and fills the `transform` slot declared by the decorator — consistent with Deor's rule that everything must be nameable.

---

## `[shape: T]` — Generics

No angle brackets anywhere. Generics in Deor are declared with the `[shape: T]` decorator and provided by the caller with `using shape ConcreteType`. `using shape` is always required at the call site; there is no implicit type inference from arguments.

Inside a `[shape: T]` function, `list` means a list of T throughout the signature and body. Write bare `list` — T is implicit.

```
[shape: T]
[using predicate: T->bool]
fn list filter(list items)
    list result = []

    for item in items
        if predicate(item)
            result insert item

    return result
```

### Call site

When a function has multiple `using` clauses, they stack on indented lines below the call. `using shape` always comes first.

```
filter(rooms)
    using shape Room
    using match_name
```

All `using` clauses are required. Missing any is a transpiler error.

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

### Concrete (non-generic) list functions

When the element type is fixed at definition time, use prefix notation in the signature. No `[shape: T]` decorator, no `using shape` at call sites.

```
fn Room list occupied_rooms(Room list rooms)
    list result = []
        using shape Room
    for room in rooms
        occupied in room
        if occupied
            result insert room
    return result
```

```
list occ = occupied_rooms(rooms)    # shape inferred from return type
```

```rust
fn occupied_rooms(rooms: &Vec<Room>) -> Vec<Room> {
    let mut result: Vec<Room> = Vec::new();
    for room in rooms {
        if room.occupied {
            result.push(room.clone());
        }
    }
    result
}
```

**Notes:**
- One `[shape: T]` parameter per function — multiple shape parameters are v2
- `T` flows through the entire signature: parameters, return type, and local `list` declarations
- `[shape: T]` stacks with `[using]` — declare both, both required at call site
- The `[...]` + `using` pattern for types (`[shape: T]` / `using shape`) and for functions (`[using alias]` / `using fn_name`) is the same mechanism applied to different kinds of requirements
