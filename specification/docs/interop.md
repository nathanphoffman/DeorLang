# Rust Interop

Deor transpiles to a single Rust file. When Deor cannot express something — a data structure, an algorithm, a crate call — you drop into a `rust` block. This is intentional and encouraged where appropriate.

---

## `rust` Blocks

A `rust` block is raw Rust inlined directly into the generated output. It is delimited by indentation — everything indented under `rust` is captured verbatim and emitted as-is.

```
fn string read_env(string key)
    rust
        std::env::var(key.as_str()).unwrap_or_default()
```

Deor parameters are available inside the block by their Deor names. The last expression is the return value and must match the Deor function's return type.

Deor does not check this — a `rust` block is spliced into the generated file verbatim, with no awareness of what type the block actually produces. If the last expression doesn't match the declared return type, Deor will transpile it without complaint and the mismatch only surfaces as a `rustc` error against the generated `.rs` file, not against your `.deor` source. Anyone writing a `rust` block is expected to know the Rust they're writing well enough to get this right.

---

## When to Use a `rust` Block

Use a rust block when Deor cannot express what you need — crate calls, closures, `HashMap`, async, type casting, anything that requires Rust's full type system. Do not reach for a rust block just to avoid Deor syntax.

The `raw` type exists for passing opaque Rust values through Deor code. It is the right pattern when you need to build something once in Rust and use it repeatedly — a HashMap built from a list, a compiled regex, a connection pool handle. Build it once in a rust block, hold it as `raw`, pass it through:

```
raw index = rust
    entries.iter()
        .map(|e| (e.key.clone(), e.value.clone()))
        .collect::<std::collections::HashMap<String, String>>()

string result = lookup(index, search_key)
```

This generates the same Rust you would write by hand — no boxing, no overhead.

---

## The Wrapping Pattern

The recommended pattern is a small rust block inside a Deor function. The Deor function owns the signature and naming; the rust block handles the implementation detail.

```
fn string json_get(string src, string key)
    rust
        let v: serde_json::Value = serde_json::from_str(&src).unwrap_or(serde_json::Value::Null);
        v.get(&key).and_then(|x| x.as_str()).unwrap_or("").to_string()
```

Keep rust blocks small. If a rust block is growing large, it is a signal that the logic belongs in a dedicated `.rs` file or should be broken into multiple wrapped functions. A rust block should do one thing.

---

## External `.rs` Files

If you have a large body of Rust code that does not belong inline, you can pull it in with `include!` inside a rust block:

```
rust
    include!("helpers.rs");
```

The path is relative to the generated output file. This should be used sparingly. Prefer Deor-wrapped rust blocks in `.deor` lib files over external `.rs` imports — they stay in version control naturally, travel with the import system, and keep the interop surface visible in one place.

Only use `include!` when the Rust code is genuinely too large or complex to live inline and has no Deor-facing wrapper worth writing.
