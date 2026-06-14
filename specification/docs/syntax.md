# Syntax

## Block Structure (No Colons)

Indentation alone opens a block after `fn`, `if`, `for`, `type`, or `struct`. No colon is written.

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
int x = 5    # inline comment
```

```rust
// this is a comment
let x: i32 = 5;    // inline comment
```

Only `#` is valid — `//`, `/*`, `*/`, and `--` are syntax errors in Deor.

**Conversion notes:** `#` → `//` in generated Rust. No multi-line comment form is needed in source since generated Rust is not intended to be hand-read.
