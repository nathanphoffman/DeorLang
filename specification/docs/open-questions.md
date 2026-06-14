# Open Questions / Future Work

- **Additional mutation verbs** beyond `insert`/`remove` (pop, etc.) — same "verb → `Vec` method" pattern, just not yet enumerated.
- **Pattern matching / `match`** — listed early as a candidate header keyword but never fully designed.
- **Error handling** — validator types produce `None` on invalid construction (not panics). Whether user code gets a `Result`-like type for recoverable errors (distinct from absence) is undecided.
- **Literal predicate validation** — `Squarefeet bad = -1` currently produces a runtime `None`. The transpiler will eventually evaluate predicates on known literals at transpile time and emit a compile error instead. Deferred.
- **String semantics** — `string` currently assumed to map to owned `String` by default, with `&str` used where the transpiler can prove borrowing is sufficient. Not yet stress-tested against real programs.
- **User-defined generics** beyond `list<T>` / `list<T, N>` — no syntax yet for generic `struct`/`fn` declarations.
