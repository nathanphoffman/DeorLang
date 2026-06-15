# Open Questions / Future Work

These are active design decisions not yet resolved.

---

## Struct Field Extraction Order

Construction enforces declaration order (`room as (area, name, occupied)` must match struct field order). Should extraction also enforce order? Consistent with "both ordered and named" principle:

```
struct Room
    Squarefeet area
    string name

(area, name) in room    # enforced order?
(name, area) in room    # transpiler error?
```

Likely yes — needs decision and doc update to `destructuring.md` and `enforced_practices.md`.

---

## `list<ValidatorType>`

Is `list<Roll>` valid? It would be `Vec<Option<Roll>>` in Rust. Iterating it gives `Option<Roll>` elements, each truthy/falsy. Useful but semantics need to be explicit — especially around `insert`, construction, and whether a failed predicate on assignment silently inserts `None` into the list.

---

## Visibility

Are all top-level declarations importable by other files? The caller already opts in explicitly (`(fn) in "./utils"`), so there's some natural scoping. But the defining file has no say in what's exported. Options:
- All declarations are always importable (simple, consistent)
- A `pub` prefix or similar marks things as exported (more control, more syntax)

---

## `is known` / `avow` — Resolved

Presence checks use `if val` (truthy = Some) and `if not val` (falsy = None). The forced unwrap uses the dedicated keyword `avow`:

```
if roll              # presence check — if roll is Some
if not roll          # absence check — if roll is None
int val = (avow roll)   # forced unwrap — panics if None, extracts inner value
```

`(val is known)` was replaced by `(avow val)` — `avow` is a cleaner, standalone keyword with no ambiguity with the `is` equality operator. Using `avow` on a non-validator-type is a transpiler error. Parens are always required.

**Decision:** `is known` removed. `avow` is the forced-unwrap keyword. `if val` / `if not val` are the idiomatic presence checks.

---

## Async / Concurrency

Not addressed. Likely handled via `rust` blocks for v1. Decide whether async is in scope for v2 or permanently delegated to Rust interop.

---

## String Operations — Resolved

Specced as `deor:strings`. See [deor:strings](strings.md). Functions provided: `contains`, `trim`, `split`, `to_upper`, `to_lower`, `starts_with`, `ends_with`. Operations not covered (`replace`, `index_of`, character access) use `rust` blocks. Character-level indexing deferred to v2 due to UTF-8 complexity.

---

**Deferred to v2 — see [v2 roadmap](v2.md):**
- Literal predicate validation (compile-time `None` for known-bad literals)
- `throw` with struct support
- String `&str` performance inference
- Multiple `[shape: T, U]` parameters
- Hex/binary numeric literals (`0xFF`, `0b1010`)
- Additional mutation verbs beyond `insert`/`remove` (pop, etc.)
