# Open Questions / Future Work

- **Additional mutation verbs** beyond `append` (pop, remove, insert, etc.) — same "verb → `Vec` method" pattern, just not yet enumerated.
- **Pattern matching / `match`** — listed early as a candidate header keyword but never fully designed.
- **Error handling** — validator types panic on invalid construction; whether user code gets a `Result`-like type for recoverable errors is undecided.
- **String semantics** — `string` currently assumed to map to owned `String` by default, with `&str` used where the transpiler can prove borrowing is sufficient. Not yet stress-tested against real programs.
- **User-defined generics** beyond `list<T>` / `list<T, N>` — no syntax yet for generic `struct`/`fn` declarations.
