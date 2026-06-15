# Built-in Functions

All built-in functions are available without any import. Because they are part of the language, they accept literals and expressions directly — no named variable required. See [Enforced Practices — Named Arguments](enforced_practices.md#named-arguments--user-defined-functions-only).

---

## Input / Output

| Function | Signature | Notes |
|---|---|---|
| `print(value)` | any type → void | Converts value to string and writes to stdout with newline |
| `read_line()` | → `string` | Reads one line from stdin, strips the trailing newline |

```
print("Hello, world!")
print(count)

string input = read_line()
print(input)
```

```rust
println!("{}", "Hello, world!");
println!("{}", count);

use std::io::{self, BufRead};
let mut line = String::new();
io::stdin().lock().read_line(&mut line).unwrap();
let input: String = line.trim_end_matches('\n').to_string();
println!("{}", input);
```

`read_line()` always returns a `string` — parse the result with `parse_int` or `parse_float` if a number is needed.

---

## Length

| Function | Signature | Notes |
|---|---|---|
| `len(value)` | `string` or any list shape → `int` | Number of characters or elements |

```
int cnt = len(name)
int size = len(rooms)
```

---

## Numeric Range

| Function | Signature | Notes |
|---|---|---|
| `range(count)` | `int` → range | Produces values `0` through `count-1` |
| `range(start, end)` | `int, int` → range | Produces values `start` through `end-1`; exclusive upper bound |

`range(count)` is shorthand for `range(0, count)`. See [Loops](loops.md) for full usage including the no-variable form `for range(n)`.

---

## Math

| Function | Signature | Notes |
|---|---|---|
| `pow(base, exp)` | `int, NonNeg` → `int` | Integer exponentiation; negative exponent is a transpiler error |
| `sqrt(val)` | `float` → `NonNegFloat` | Square root; `None` if `val < 0` |
| `abs(val)` | `int` → `int` | Absolute value |
| `floor(val)` | `float` → `int` | Round down |
| `ceil(val)` | `float` → `int` | Round up |
| `round(val)` | `float` → `int` | Round to nearest |
| `min(left, right)` | `int, int` → `int` | Smaller of two values |
| `max(left, right)` | `int, int` → `int` | Larger of two values |

```
int val = pow(2, 10)                      # 1024

NonNegFloat res = sqrt(4.0)               # Some(2.0)
float root = res else 0.0                 # 2.0

NonNegFloat bad = sqrt(-1.0)              # None
float safe = bad else 0.0                 # 0.0

int small = min(3, 7)                     # 3
```

`sqrt` returns `NonNegFloat` — unwrap with `else` for a safe default or `avow` when certain the input is non-negative.

`pow`'s `exp` parameter is typed `NonNeg` — enforces `val >= 0` at the type level. See [Stdlib Numeric Types](#stdlib-numeric-types).

---

## Random

| Function | Signature | Notes |
|---|---|---|
| `random(min, max)` | `int, int` → `int` | Random integer in `[min, max]` inclusive; `throw` if `min > max` |

```
int roll = random(1, 6)
```

```rust
use rand::Rng;
// transpiler emits a guard: if min > max { panic!("rand: min > max") }
let roll: i32 = rand::thread_rng().gen_range(1..=6);
```

`min > max` is a programming error — `random` throws rather than returning `None`. If the bounds come from user input, validate them before calling `random`.

---

## String Operations

| Function | Signature | Notes |
|---|---|---|
| `contains(str, needle)` | `string, string → bool` | true if `needle` appears anywhere in `str` |
| `starts_with(str, prefix)` | `string, string → bool` | true if `str` begins with `prefix` |
| `ends_with(str, suffix)` | `string, string → bool` | true if `str` ends with `suffix` |
| `trim(str)` | `string → string` | strips leading and trailing whitespace |
| `to_upper(str)` | `string → string` | all characters uppercased |
| `to_lower(str)` | `string → string` | all characters lowercased |
| `split(str, delimiter)` | `string, string → nameList` | split on every occurrence of `delimiter`; result type requires `shape nameList = list of string` |

```
string clean = trim("  Hello, World!  ")
bool found = contains(clean, "World")

shape nameList = list of string
nameList parts = split("apple,banana,cherry", ",")
```

For operations not covered here (`replace`, `index_of`, character access), use a `rust` block.

---

## Type Conversion

| Function | Signature | Notes |
|---|---|---|
| `to_float(val)` | `int` → `float` | Widens integer to float |
| `to_int(val)` | `float` → `int` | Truncates float toward zero |
| `to_string(val)` | any primitive → `string` | Formats value as string |

```
float flt = to_float(count)
int val = to_int(ratio)
string label = to_string(score)
```

```rust
let flt: f64 = count as f64;
let val: i32 = ratio as i32;
let label: String = score.to_string();
```

`to_int` truncates — `to_int(2.9)` = `2`, `to_int(-2.9)` = `-2`. Use `floor`/`ceil`/`round` before converting if rounding behavior matters.

---

## Fallible Parsing

| Function | Signature | Notes |
|---|---|---|
| `parse_int(str)` | `string` → `ParsedInt` | `None` if `str` is not a valid integer |
| `parse_float(str)` | `string` → `ParsedFloat` | `None` if `str` is not a valid decimal |

```
ParsedInt result = parse_int(user_input)
if result
    int val = (avow result)
    print(val)

int port = parse_int(port_str) else 8080    # default if unparseable
```

```rust
let result: Option<ParsedInt> = user_input.parse::<i32>().ok().map(ParsedInt);
if let Some(r) = result {
    let val: i32 = r.0;
    println!("{}", val);
}
let port: i32 = port_str.parse::<i32>().ok().map(ParsedInt).map(|v| v.0).unwrap_or(8080);
```

`ParsedInt` and `ParsedFloat` behave identically to user-defined validator types — check presence, then unwrap or provide a default.

---

## Stdlib Numeric Types

These validator types are built into the language. They represent constrained numeric domains and are used as parameter and return types for built-ins that operate on restricted ranges.

| Type | Base | Predicate | Use |
|---|---|---|---|
| `NonNeg` | `int` | `val >= 0` | Non-negative integers — indices, sizes, exponents |
| `Positive` | `int` | `val > 0` | Strictly positive integers — divisors, counts |
| `NonNegFloat` | `float` | `val >= 0.0` | Non-negative floats — `sqrt` return type |
| `PositiveFloat` | `float` | `val > 0.0` | Strictly positive floats — rates, logarithm inputs |

These behave identically to user-defined `type` declarations — `Option<T>` under the hood, support `avow`, `else`, and `if`/`if not` checks, usable as struct fields.

```
NonNeg exp = 10
if exp
    int result = pow(2, exp)

NonNegFloat res = sqrt(4.0)    # Some(2.0)
float root = res else 0.0      # 2.0
```

**Conversion notes:** each type compiles to a Rust newtype struct with `fn new(n: T) -> Option<Self>`, identical to user-defined validator types.
