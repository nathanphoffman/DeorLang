# Syntax

## Block Structure (No Colons)

Indentation alone opens a block after `fn`, `if`, `for`, `type`, `struct`, `shape`, `enum`, or `rust`. No colon is written.

```
fn int abs(int x)
    if x < 0
        -x
    else
        x
```

```rust
fn abs(x: i32) -> i32 {
    if x < 0 {
        -x
    } else {
        x
    }
}
```

**Conversion notes:** indentation depth maps directly to brace nesting — a straightforward structural transform with no semantic subtleties.

---

## One Statement Per Line

No line continuations except inside delimiters. Long expressions wrap inside `()` or `[]`:

```
Connection conn = Connect(
    host,
    port,
    timeout,
)
```

```rust
let conn: Connection = Connect(
    host,
    port,
    timeout,
);
```

**Conversion notes:** trailing commas are encouraged and map directly onto Rust's own trailing-comma convention.

---

## Comments

`#` starts a line comment. Everything from `#` to end of line is ignored. No block comments.

```
# this is a comment
int val = 5    # inline comment
```

```rust
// this is a comment
let val: i32 = 5;    // inline comment
```

Only `#` is valid — `//`, `/*`, `*/`, and `--` are syntax errors in Deor.

**Conversion notes:** `#` → `//` in generated Rust. No multi-line comment form is needed in source since generated Rust is not intended to be hand-read.

---

## Reserved Words

These identifiers have fixed meaning in Deor and cannot be used as variable, function, parameter, struct, or type names.

### Block Headers
Open an indented block when followed by a newline.

| Word | Use |
|---|---|
| `fn` | Function declaration |
| `if`,`else`,`else if` | Conditional block |
| `for` | Loop |
| `type` | Validator type declaration |
| `struct` | Struct declaration (`struct`) |
| `shape` | Shape declaration (`shape name = list of T` / `func of T to O`) |
| `enum` | Enum declaration — named variant type |
| `rust` | Inline Rust block |
| `block` | Adds a block scope, all variables inside are non-polluting, similar to rust {}, very useful paired with macros you don't want bleeding into scope |

### Statement Keywords

| Word | Use |
|---|---|
| `return` | Return a value from a function |
| `crash` | Unrecoverable hard stop (`panic!`) |
| `avow` | Forced unwrap of a validator type — panics if not valid |
| `break` | Exit the innermost loop |
| `continue` | Skip to the next loop iteration |
| `const` | Immutable typed binding (function scope only) |
| `move` | Transfer ownership instead of cloning — argument, loop, or assignment |

### Operators and Expression Keywords

| Word | Use |
|---|---|
| `and` | Logical AND (`&&`) |
| `or` | Logical OR (`\|\|`) |
| `not` | Logical NOT (`!`) |
| `is` | Structural equality (`is`) and inequality (`is not`) |
| `in` | Destructuring / import / loop iteration source |
| `as` | Shape-derived binding |
| `with` | Record update (inside `as` binding) |
| `at` | Index access and write (`list at idx`, `list at idx = val`, `list at end = val`) |
| `end` | Reserved sentinel — "end of list" in `list at end = val` |
| `of` | Element type connector in shape declarations (`list of Room`) |
| `to` | Return type connector in func shapes (`func of Room to bool`) |

### Values

| Word | Use |
|---|---|
| `true` | Boolean true |
| `false` | Boolean false |
| `empty` | Empty initial value for list shapes — `Vec::new()` (declaration only) |

### Built-in Type Keywords

| Word | Use |
|---|---|
| `list` | Parameterized list — always used inside a `shape` declaration |
| `func` | Parameterized function type — always used inside a `shape` declaration |

**Note:** `remove` is a reserved mutation verb for lists and cannot be used as an identifier. `range` is a for-loop-only construct — it is not a callable function and cannot be used outside a `for` header (e.g. assigned to a variable or passed as an argument). `end` is a reserved sentinel — only valid as `list at end = val`.
