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

```
fn string read_file(string path)
    rust
        std::fs::read_to_string(path.as_str())
            .unwrap_or_default()
```

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
| `list` (element type `T`) | `Vec<T>` |
| validator types | `Option<T>` |
| structs | Rust struct |

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
- `deor:` stdlib modules have their deps pre-bundled — never need to declare them
- `std` never needs a `deps` declaration — it's always available
- Crates used only inside `.rs` files still need a `deps` block somewhere in the project

---

## Import Tiers

Three import tiers, all using the same `in` syntax:

```
(something) in utils              # another .deor file in your project
(something) in rust:math_utils    # raw .rs file in your project
(something) in deor:http          # official Deor stdlib wrapper
(something) in reqwest            # cargo crate (declared in deps)
```

---

## `deor:` Standard Library

Official Deor wrappers around common Rust crates and `std` modules. Written as `.deor` files using `rust` blocks internally. Pre-bundled deps, importable with the `deor:` prefix. They handle `Result<T, E>` internally so Deor-typed return values come out cleanly.

```
(get, post) in deor:http                                          # wraps reqwest
(read, write, exists) in deor:fs                                  # wraps std::fs
(args, var) in deor:env                                           # wraps std::env
(now, elapsed) in deor:time                                       # wraps std::time
(random) in deor:rand                                             # wraps rand crate
(parse_int, to_string) in deor:convert
(contains, trim, split, to_upper, to_lower, starts_with, ends_with) in deor:strings  # wraps std::str
```

See [deor:strings](strings.md) for full documentation on the string functions.

The `deor:` namespace is reserved. Third-party packages use bare crate names via `deps`.

---

## When to Use What

| Scenario | Approach |
|---|---|
| Common `std` operations (fs, env, time) | `deor:` stdlib wrappers |
| Popular crates (reqwest, serde) | `deor:` stdlib wrappers |
| Custom crate usage | `deps` block + `rust` block |
| Large or complex raw Rust | `rust:myfile` external import |
| Obscure `std` / one-off call | `rust` block — `std::` works inline, no declaration needed |
