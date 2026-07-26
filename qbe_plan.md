# QBE Backend — Phased Plan

Two tracks. Track A is worth doing regardless of backend. Track B only matters if/when QBE replaces Rust as a target.

---

## Track A — Useful even if we stay on Rust

1. **[DONE] Move checker in the Deor frontend.** `check_use_after_move.deor` + `track_copy_vars.deor` in `tokens_validator/`. Flow-sensitive across if/else-if/else chains (snapshot stack keyed by block depth, correctly excludes arms that end in `return`/`break`/`continue`), and tracks partial-move destructuring (`move (f1, f2) in source`) per field via `moved_fields`. Self-hosting confirmed clean and stable across repeated build cycles (71/71 tests).
   - **Why this had to move past "diagnostics nicety":** on the Rust backend, anything this checker misses is still caught by rustc's own borrow checker — an ugly error, but never a bad binary. QBE has no such backstop (see Track B #5), so before QBE ships this checker's remaining gaps become real memory-safety bugs (use-after-free, double-free) in compiled output, not compile errors. It needed to become sound, not just conservative-and-good-enough.
   - **Remaining known gaps (undocumented as of this pass, not yet fixed):**
     - **False positive, highest priority:** `(f1, f2) in source` destructure targets don't clear prior moved-status the way `f1 = expr` does. Move a name, then re-bind it via destructure, then read it — falsely flagged as already-moved. Breaks the checker's own "never false-positive" rule; can reject a valid program.
     - **False negative:** doesn't flag the whole struct being used by value after a partial move (only a repeat destructure of an already-moved field).
     - **False negative:** single textual pass, not a real loop simulation — a move at the end of a loop body followed by a read at the start of the same body (a real use-after-move on the second iteration) isn't caught.
2. **Formalize `raw` (Rc/Arc) lifecycle.** Currently hand-written per `rust` block. Document/standardize the pattern so it's consistent, ahead of ever needing to reimplement it manually.
3. **Diagnostics pass groundwork.** Anything built for #1 (scope tracking, binding lifetime) is reusable infrastructure for better error messages generally, independent of backend.

---

## Track B — Only needed for QBE

4. **Spike: minimal QBE codegen path.** Parallel codegen module (alongside `codegen/codegen.deor`) emitting QBE IL for a tiny subset (ints, functions, print) to prove the pipeline: Deor → QBE IL → qbe → asm → cc/ld → binary.
5. **Memory strategy for clone-default path.** Move/raw/rust aren't the only aliasing outside clone-default: `for item in collection` borrows by default (`item` is a reference, collection stays usable — see `docs/loops.md`), which rustc's borrow checker currently keeps safe for free (no mutation of the collection during iteration, no escaping the loop body). QBE has no such checker, so this needs its own explicit rule, enforced by the compiler itself. Once that's covered, ownership elsewhere is single-owner (scope-exit frees, stack or arena — no borrow checker needed for the rest). Design and implement this allocator discipline.
6. **Manual refcounting for `raw`.** No more borrowing Rust's `Rc`/`Arc` — implement our own refcount inc/dec around clone/drop of `raw`-wrapped values.
7. **Replacement for `rust` blocks.** This is the big one — currently the escape hatch for anything hard (dicts, bytes, cargo crates). Needs a real answer: raw QBE block, or C FFI. Nothing else in this plan matters if this isn't solved.
8. **Runtime library.** No Rust std. Reimplement what `lib/*.deor` currently gets for free from Rust wrappers — strings, list growth/resize, formatting, I/O — as a small C or hand-written runtime linked into every binary.
9. **Build pipeline swap.** Replace `cargo build` step with `qbe` + assembler + linker invocation; update `justfile`/install scripts accordingly.
10. **Bootstrapping decision.** The transpiler itself is written in Deor and currently transpiles to Rust to run. Decide: QBE as an *additional* output target first (dual-backend), not a rip-and-replace, until parity is proven.
11. **Parity testing + cutover call.** Run `tests/` against both backends before considering QBE a real replacement rather than an experiment.
