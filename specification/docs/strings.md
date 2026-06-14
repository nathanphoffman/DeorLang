# Strings

Deor treats `string` like JavaScript strings — simple, flexible, no ownership friction. All `String`/`&str` complexity is handled by the transpiler invisibly.

---

## Concatenation

Use `+` to join strings. The transpiler uses `format!()` under the hood, avoiding Rust's ownership-consuming `+` operator.

```
string first = "Hello"
string second = "World"
string greeting = first + " " + second
```

```rust
let greeting = format!("{} {}", first, second);
```

---

## Interpolation

Embed variables directly in string literals using `{name}` syntax. Variable names only — no expressions inside braces.

```
string name = "Nate"
string msg = "Hello {name}, welcome to Deor"
```

```rust
let msg = format!("Hello {}, welcome to Deor", name);
```

---

## Length

`len(s)` returns the number of characters as `int`. Works on strings and lists.

```
string label = "hello"
int n = len(label)
```

```rust
let n: i32 = label.len() as i32;
```

---

## Equality

`==` is structural — same content, same result, regardless of how the string was created.

```
string a = "hello"
string b = "hello"
bool same = a == b    # true
```

---

## Conversion notes

- All strings in Deor are owned `String` in Rust. The transpiler uses `&str` borrows internally where provably safe — this is invisible to the user.
- `+` concatenation always uses `format!()`, never Rust's `+` which would consume the left operand.
- `{name}` interpolation maps to a `format!()` positional argument.
- `len()` returns `i32` (Deor `int`), not `usize` — the transpiler casts via `as i32`.

**Performance note (v2):** Smart `&str` vs `String` inference for string-heavy code is flagged for v2. See [v2 roadmap](v2.md).
