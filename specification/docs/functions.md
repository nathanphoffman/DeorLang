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
shape itemList = list of Item

# Correct — use if/else to express all paths
fn process(itemList items, bool skip_invalid)
    for item in items
        valid in item
        if skip_invalid and not valid
            # skip — continue to next iteration
        else
            handle(item)
```

```
shape itemList = list of Item

# Transpiler error — early return not allowed in void functions
fn process(itemList items, bool skip_invalid)
    for item in items
        valid in item
        if skip_invalid and not valid
            return    # not allowed
        handle(item)
```

This is intentional: early returns in void functions are usually a sign the logic should be restructured. Use `if/else` or `continue` inside loops instead.

### Multiple return values

There are no anonymous tuple return types. A function returning multiple values must declare a named struct for the return type. The struct is then constructed and destructured using the existing `as`/`in` syntax — no new keywords.

```
struct DivResult
    int quotient
    int remainder

fn DivResult divmod(int left, int right)
    int quotient = left / right
    int remainder = left % right
    return (quotient, remainder)
```

```rust
struct DivResult { quotient: i32, remainder: i32 }

fn divmod(left: i32, right: i32) -> DivResult {
    let quotient = left / right;
    let remainder = left % right;
    DivResult { quotient, remainder }
}
```

Capturing the result uses standard struct destructuring:

```
(quotient, remainder) in divmod(num, div)
print(quotient)
print(remainder)
```

```rust
let result = divmod(num, div);
let (quotient, remainder) = (result.quotient, result.remainder);
```

The struct name documents what the paired values represent — `DivResult` communicates more than an anonymous `(int, int)`. If you need a general-purpose pair, declare a `struct Pair` with `value1` and `value2` fields.

---

## Return Rules

A function body that is a **single expression** — one line, nothing else — implicitly returns that expression. `return` is optional there.

Any function body with **more than one statement** requires explicit `return` at every exit point.

```
fn int square(int val)
    val * val    # single expression — return implicit
```

```rust
fn square(val: i32) -> i32 {
    val * val
}
```

```
fn int abs(int val)
    if val < 0       # multiple statements — return required everywhere
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

**Conversion notes:** the transpiler can always emit explicit `return` safely — implicit single-expression return is a source-level convenience, not a Rust requirement.

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

`throw` is an unrecoverable hard stop — transpiles to `panic!()` in Rust. Takes a named string variable — inline string literals are a transpiler error, the same as any other function argument. For recoverable error handling, accept an error handler as a `func` shape parameter instead.

```
fn int divide(int left, int right)
    if right is 0
        msg as "division by zero"
        throw msg
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

To pass behavior as a value, declare a `func` shape and accept it as a typed parameter. The caller passes a named top-level function by name — this is Deor's equivalent of a lambda.

```
shape intList = list of int
shape doubleFunc = func of int to int

fn intList apply_all(intList nums, doubleFunc transform)
    intList result = []
    for num in nums
        int out = transform(num)
        result insert out
    return result

fn int double(int num)
    return num * 2

apply_all(nums, double)    # double satisfies doubleFunc — no special syntax
```

```rust
fn apply_all(nums: &Vec<i32>, transform: fn(i32) -> i32) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    for num in nums {
        result.push(transform(*num));
    }
    result
}
```

**Conversion notes:** func shapes compile to Rust `fn` pointers — not `Fn`/`FnMut`/`FnOnce` traits. This means no closure capture, no lifetime complications, and no `Box<dyn Fn>` overhead. All functions are top-level items.

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

## Parameters

Functions accept at most **3 parameters**. If more context is needed, bundle values into a struct first. This is enforced by the transpiler.

```
fn roomList filter(roomList items, string query, filterFunc predicate)
    # 3 params: list, data, behavior — the natural ceiling
```

Parameters follow `Type name` order. All types — including shape names — are written as a prefix. `func` shape parameters are regular parameters: no special keyword, no annotation, just a typed name.

```
shape filterFunc = func of Room to bool

fn roomList filter(roomList items, filterFunc predicate)
    roomList result = []
    for item in items
        if predicate(item)
            result insert item
    return result
```

## Entry Point

The function named `main` is always the program entry point. No annotation is needed or allowed — naming the function `main` is sufficient.

```
fn main()
    # program starts here
```

```rust
fn main() {
    // program starts here
}
```

Only one `fn main()` may exist per project. Naming any other function `main` is a transpiler error if a `main` already exists.
