# Rust Interop

Deor transpiles to Rust, so escape hatches into raw Rust are a first-class feature — not an afterthought. Three mechanisms cover the full range of interop needs: inline `rust` blocks, external `.rs` file imports, and the `deor:` standard library wrappers.

---

## `rust` Blocks

A `rust` block drops into raw Rust inside a Deor function body. The Deor function signature defines the Deor-facing contract; the block handles everything else.

```
fn string http_get(string url)
    rust
        reqwest::blocking::get(url.as_str())
            .and_then(|r| r.text())
            .unwrap_or_default()
```

**Rules:**
- Block form is required — a newline and indent after `rust` are enforced by the transpiler. No one-liners.
- Deor parameters are available by their Deor names inside the block (already snake_case, maps directly to Rust).
- The last expression is the return value and must match the Deor function's return type.
- Full Rust inside: any types, closures, method chaining, `?`, SIMD intrinsics, raw pointers — no restrictions.
- `std::` is always in scope inside `rust` blocks. No import or `deps` declaration needed for standard library usage.

**Block boundary — indentation only:**

The transpiler delimits a `rust` block purely by indentation. Every line indented deeper than the `rust` keyword is captured verbatim and emitted directly into the generated Rust output. The block ends when a non-blank line returns to the same indentation level as `rust` or shallower. Blank lines inside the block are always considered part of the block, regardless of their indentation, as long as indented content resumes afterward.

Inside the block, Rust's own formatting conventions apply freely — curly braces, `match` arms, nested closures, Rust-style indentation. The transpiler does not parse or validate any of it. It strips the leading Deor indent level and passes the rest through as a raw string:

```
fn string classify(string input)
    rust
        let result = match input.as_str() {
            "a" | "e" | "i" | "o" | "u" => {
                let upper = input.to_uppercase();
                format!("vowel: {}", upper)
            },
            _ => String::from("consonant"),
        };
        result
```

Everything from `let result` through `result` is captured verbatim. The Deor transpiler sees only the indentation boundary — the curly braces, `match` syntax, and Rust-style nesting inside are invisible to it.

```
fn string read_file(string path)
    rust
        std::fs::read_to_string(path.as_str())
            .unwrap_or_default()
```

---

## The `raw` Type

`raw` is a Deor variable that holds an opaque Rust value — one the transpiler cannot name or inspect. It is the bridge between `rust` blocks when intermediate computation produces a Rust-native structure that has no Deor equivalent.

```
raw index = rust
    entries.iter()
        .map(|e| (e.key.clone(), e.value.clone()))
        .collect::<std::collections::HashMap<String, String>>()

string result = rust
    index.get(search_key.as_str()).cloned().unwrap_or_default()
```

**Rules:**
- A `raw` variable is always assigned from a `rust` block return value — never from a Deor expression.
- A `raw` variable may only be consumed inside a `rust` block.
- A `raw` variable may be passed as a parameter to a Deor function, provided that function uses it only inside `rust` blocks.
- `raw` cannot appear as a struct field.
- No Deor operator (`avow`, `else`, `if`, `is`, `+`, etc.) works on a `raw` variable — it is opaque to the transpiler.
- `raw` parameters count toward the 3-parameter limit.

**Zero cost:** The transpiler emits the `raw` variable name into the generated Rust without wrapping or boxing. Rust's type inference assigns the concrete type at compile time. No `Box<dyn Any>`, no type erasure, no runtime overhead — the generated Rust is identical to code you would write by hand.

**Passing `raw` down the call hierarchy:**

A Deor function that accepts a `raw` parameter generates a generic Rust function. Rust monomorphizes it to the concrete type at each call site — zero runtime cost:

```
fn string lookup(raw index, string key)
    rust
        index.get(key.as_str()).cloned().unwrap_or_default()
```

```rust
fn lookup<T>(index: &T, key: String) -> String {
    index.get(key.as_str()).cloned().unwrap_or_default()
}
```

**The build-once pattern:**

Build a Rust-native structure once, capture it in `raw`, pass it through the call graph wherever it is needed:

```
shape entryList = list of Entry

raw index = rust
    entries.iter()
        .map(|e| (e.key.clone(), e.value.clone()))
        .collect::<std::collections::HashMap<String, String>>()

string name = lookup(index, search_key)
string alt  = lookup(index, fallback_key)
```

This performs identically to hand-written Rust. The `entryList` is already `Vec<Entry>` in Rust. The HashMap build is O(n) once; each `lookup` call is O(1). Passing `raw` down through multiple function calls adds no overhead — Rust's monomorphization ensures the concrete type is resolved at compile time.

**Why Deor deliberately omits dict, while, and more:**

Deor has no native `dict` type, no `while` loop, no ring buffer, no async runtime. This is a boundary decision, not an oversight. Every one of these constructs lives comfortably in Rust and uncomfortably in a high-level transpiled language. A native `dict` would require a two-type generic (`HashMap<K, V>`) that conflicts with Deor's one-type-per-generic constraint. A `while` loop obscures termination guarantees that `for` makes explicit, and truly unbounded loops always coincide with low-level systems concerns that belong in Rust anyway.

Rather than design half-measures, Deor draws a clean line: bounded, typed data belongs in Deor; data structures and algorithms that require Rust's full type system belong in `rust` blocks. The `raw` type makes that boundary clean and explicit — build it in Rust, name it in Deor, pass it where it is needed.

---

## Boundary Types

Only these types cross the Deor/Rust boundary — as function parameters into `rust` blocks and return values out of them. Everything else stays inside.

| Deor | Rust |
|---|---|
| `int` | `i32` |
| `float` | `f64` |
| `bool` | `bool` |
| `string` | `String` (available as `&str` via `.as_str()`) |
| `bytes` | `Vec<u8>` |
| list shape (e.g. `roomList`) | `Vec<T>` where T is the shape's element type |
| validator types | `Option<T>` |
| structs | Rust struct |

`raw` is not listed here — it does not cross a type boundary. A `raw` variable *is* the Rust value; the transpiler passes its name through directly. See [The `raw` Type](#the-raw-type).

If two crate calls both need `u64`, keep both in one `rust` block — never cross the boundary mid-computation. The `rust` block is the right place to cast:

```
fn int file_size(string path)
    rust
        std::fs::metadata(path.as_str())
            .map(|m| m.len() as i32)
            .unwrap_or(0)
```

---

## External `.rs` File Imports

For raw Rust code too large to inline, write a `.rs` file and import from it using the `rust:` prefix:

```
(calculate, transform) in rust:math_utils
```

The transpiler looks for `math_utils.rs` in the project directory and includes it as a Rust module (`mod math_utils;`) in the generated output. No `.rs` extension in the import — the `rust:` prefix implies it.

Functions imported via `rust:` can only be called from inside `rust` blocks — they have Rust signatures, not Deor ones. The Deor function wraps them:

```
(compress, decompress) in rust:codec

fn bytes compress_data(bytes data)
    rust
        codec::compress(&data)
```

The `.rs` file is full Rust — any `use` statements, any types, any Rust features. It can use standard library imports freely and use crates declared in the project's `deps` blocks. It cannot call back into Deor-defined functions (it's a lower layer).

**Project layout:**
```
project/
  main.deor
  utils.deor
  codec.rs          # importable via rust:codec
```

`rust:` imports are allowed in any `.deor` file, not just files with inline `rust` blocks. A clean Deor wrapper module can import from a `.rs` file and expose a tidy Deor interface with no inline `rust` blocks in the wrapper itself.

---

## `deps` Block

Cargo dependencies are declared in a `deps` block at the top of any `.deor` file. The transpiler reconciles all `deps` blocks across the project into a single generated `Cargo.toml` — you never touch `Cargo.toml` directly.

```
deps
    reqwest 0.11
        features blocking, json
    serde 1.0
        features derive
```

- Deor-idiomatic: indentation for features, no TOML syntax
- Same crate declared in multiple files: deduped by the transpiler
- Version conflict across files: transpiler error with a helpful message
- `std` never needs a `deps` declaration — it's always available
- Crates used only inside `.rs` files still need a `deps` block somewhere in the project
- Built-in functions that use crates (e.g. `random` uses `rand`) have their deps bundled by the transpiler automatically

---

## Import Tiers

Two import tiers, both using the same `in` syntax:

```
(something) in "./utils"          # another .deor file in your project
(something) in rust:math_utils    # raw .rs file in your project
```

Built-in functions (`print`, `len`, `range`, `sqrt`, `random`, `contains`, etc.) are always available — no import needed. See [Built-ins](builtins.md).

---

## When to Use What

| Scenario | Approach |
|---|---|
| Common operations (math, strings, I/O, parsing) | Built-in functions — no import |
| Custom crate usage | `deps` block + `rust` block |
| Large or complex raw Rust | `rust:myfile` external import |
| Obscure `std` / one-off call | `rust` block — `std::` works inline, no declaration needed |
| HashMap, BTreeMap, ring buffer, or any Rust-native structure passed between calls | `rust` block to build → `raw` variable to carry |
