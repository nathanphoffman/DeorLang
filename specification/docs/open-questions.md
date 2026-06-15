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

## `is not known`

Is `val is not known` valid syntax for a None check? Currently `if not val` is the only way. `is not known` would be more explicit and symmetric with `is known`:

```
if val is known       # Some — currently (val is known) with parens for unwrap
if val is not known   # None — not currently specced
```

Needs a clear decision on whether this form is valid and whether it's an alias for `if not val`.

---

## Async / Concurrency

Not addressed. Likely handled via `rust` blocks for v1. Decide whether async is in scope for v2 or permanently delegated to Rust interop.

---

**Deferred to v2 — see [v2 roadmap](v2.md):**
- Literal predicate validation (compile-time `None` for known-bad literals)
- `throw` with struct support
- String `&str` performance inference
- Multiple `[shape: T, U]` parameters
- Hex/binary numeric literals (`0xFF`, `0b1010`)
- Additional mutation verbs beyond `insert`/`remove` (pop, etc.)
