# Open Questions / Future Work

- **Additional mutation verbs** beyond `insert`/`remove` (pop, etc.) — same "verb → `Vec` method" pattern, just not yet enumerated.
- **User-defined generics** — `[shape: T]` annotation and `using shape ConcreteType` call syntax are designed (see [functions](functions.md#shape-t--generics)). Multiple shape parameters (`[shape: T, U]`) are v2.
- **Built-in functions** — `print`, `len`, `range` and math functions (`rand`, `floor`, `sqrt`) are used in examples but not formally specced. These will form the `deor:` stdlib core.
- **Operators** — arithmetic and comparison operators are used in examples but never formally listed. Integer division, modulo (`%`), `!=` syntax all undecided.
- **Type conversion** — no syntax yet for `int` to `float`, `float` to `int`, `int` to `string`.
- **Index read** — `list[0]` for reading a list element by index is used but not formally specced.
- **Truthiness beyond validator types** — whether `if my_list` (non-empty check) or `if my_string` (non-empty check) are valid is undecided.
- **Module system** — how `.deor` files import each other (`(fn) in utils` resolving to `utils.deor`) is not fully specced.

**Deferred to v2 — see [v2 roadmap](v2.md):**
- Literal predicate validation (compile-time `None` for known-bad literals)
- `throw` with struct support
- String `&str` performance inference
- Multiple `[shape: T, U]` parameters
