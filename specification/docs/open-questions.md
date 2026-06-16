# Open Questions / Future Work

These are active design decisions not yet resolved.

---

## Struct Field Extraction Order — Resolved

Extraction enforces declaration order, symmetric with construction. `(name, area) in room` is a transpiler error when `area` is declared first. See [Destructuring](destructuring.md#extraction-order) and [Enforced Practices](enforced_practices.md#field-extraction-order).

---

## List of Validator Type

Is `shape rollList = list of Roll` valid (a list whose element type is a validator type)? It would be `Vec<Option<Roll>>` in Rust. Iterating it gives `Option<Roll>` elements, each truthy/falsy. Useful but semantics need to be explicit — especially around `at end =`, `remove at`, and whether a failed predicate on assignment silently inserts `None` into the list.

---

## Visibility — Resolved

Public by default. `private` prefix restricts a declaration to the current file. See [Enforced Practices](enforced_practices.md#visibility--private) and [Imports](imports.md).

---

## `avow` — Resolved

Presence checks use `if val` (truthy = Some) and `if not val` (falsy = None). Forced unwrap uses `(avow val)` — parens always required. Using `avow` on a non-validator-type is a transpiler error.

---

## Async / Concurrency

Not addressed. Handled via `rust` blocks for v1. Decide whether async is in scope for v2 or permanently delegated to Rust interop.

---

## Collection Index Access — Resolved

`rooms[idx]` replaced with `rooms at idx`. Append is `rooms at end = item`. Slice is `rooms in range(start, end)` where `end` as a keyword means "length of this list." Remove is `rooms remove at idx`. No mid-list insertion in v1 — use `remove at` + `at end` rebuild pattern or a `rust` block.

---

## Built-ins and Import System — Resolved

All standard functions (`print`, `len`, `range`, math, strings, random, parsing, type conversion) are built into the language with no import required. The `deor:` import prefix is removed. Local module imports use string paths (`"./file"`); raw Rust files use `rust:` prefix. Import aliasing removed — rename in source instead.

---

**Deferred to v2 — see [v2 roadmap](v2.md):**
- Literal predicate validation (compile-time `None` for known-bad literals)
- `throw` with struct support
- String `&str` performance inference
- Hex/binary numeric literals (`0xFF`, `0b1010`)
- List concatenation / spread syntax
- Function annotations (test, deprecated, pure)
