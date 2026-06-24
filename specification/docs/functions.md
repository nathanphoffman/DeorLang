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

`void` is the explicit return type for functions that return nothing. It is mandatory — omitting the return type is a transpiler error.

```
fn void greet(string name)
    print(name)
```

```rust
fn greet(name: String) {
    println!("{}", name);
}
```

The entry point follows this same rule:

```
fn void main()
    # program starts here
```

Void functions may use `return` with no value for early exit:

```
shape itemList = list of Item

fn void process(itemList items, bool skip_invalid)
    for item in items
        valid in item
        if skip_invalid and not valid
            return
        handle(item)
```

```rust
fn process(items: Vec<Item>, skip_invalid: bool) {
    for item in &items {
        let valid = item.valid;
        if skip_invalid && !valid {
            return;
        }
        handle(item.clone());
    }
}
```

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

A function returning a validator type returns a variable that may or may not be valid. To return a not-valid result, declare the variable without a value and return it unassigned, or assign a value that fails the predicate. `empty` and `none` are both transpiler errors in return position.

```
shape rollList = list of Roll

fn Roll find_best(rollList rolls)
    Roll best

    for roll in rolls
        if roll valid
            best = roll

    return best    # not valid if rolls is empty or all not valid
```

When the result depends entirely on the predicate, just assign and return:

```
fn Positive get_positive(int num)
    Positive result = num    # not valid if num fails the predicate
    return result
```

Primitive return types (`fn int`, `fn bool`, etc.) are never valid/not valid — they always have a value.

---

## No Lambdas / Closures / Nested Functions

All callable values are named top-level `fn`s. There is no anonymous-function syntax, and functions may not be defined inside other function bodies. No built-in `filter`, `map`, or `reduce` — write explicit loops instead.

To pass behavior as a value, declare a `func` shape and accept it as a typed parameter. The caller passes a named top-level function by name — this is Deor's equivalent of a lambda. See [Shapes — Func Shapes](shapes.md#func-shapes) for declaration syntax, single-param constraint, and conversion notes.

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
    roomList result = empty
    for item in items
        if predicate(item)
            result at end = item
    return result
```

## Entry Point

The function named `main` is always the program entry point. It must be declared `fn void main()`.

```
fn void main()
    # program starts here
```

```rust
fn main() {
    // program starts here
}
```

Only one `fn void main()` may exist per project. Naming any other function `main` is a transpiler error if a `main` already exists.

