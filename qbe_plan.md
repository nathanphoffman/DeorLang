# QBE Backend — Phased Plan

Two tracks. Track A is worth doing regardless of backend. Track B only matters if/when QBE replaces Rust as a target.

---

## Track A — Useful even if we stay on Rust

1. **[DONE] Move checker in the Deor frontend.** `check_use_after_move.deor` + `track_copy_vars.deor` in `tokens_validator/`. Deliberately conservative — same-block only (clears on INDENT/DEDENT/`KW_ELSE`), so it never false-positives across a branch, but also won't catch a move-then-read that spans an if/else. Known gap: `move (f1, f2) in source` partial-move destructuring isn't tracked. Self-hosting confirmed clean (68/68 tests, transpiler recompiles itself with the check active).
2. **Formalize `raw` (Rc/Arc) lifecycle.** Currently hand-written per `rust` block. Document/standardize the pattern so it's consistent, ahead of ever needing to reimplement it manually.
3. **Diagnostics pass groundwork.** Anything built for #1 (scope tracking, binding lifetime) is reusable infrastructure for better error messages generally, independent of backend.

---

## Track B — Only needed for QBE

4. **Spike: minimal QBE codegen path.** Parallel codegen module (alongside `codegen/codegen.deor`) emitting QBE IL for a tiny subset (ints, functions, print) to prove the pipeline: Deor → QBE IL → qbe → asm → cc/ld → binary.
5. **Memory strategy for clone-default path.** No aliasing happens outside `move`/`raw`/`rust`, so this is scope-exit frees (stack or arena) — no borrow checker needed. Design and implement this allocator discipline.
6. **Manual refcounting for `raw`.** No more borrowing Rust's `Rc`/`Arc` — implement our own refcount inc/dec around clone/drop of `raw`-wrapped values.
7. **Replacement for `rust` blocks.** This is the big one — currently the escape hatch for anything hard (dicts, bytes, cargo crates). Needs a real answer: raw QBE block, or C FFI. Nothing else in this plan matters if this isn't solved.
8. **Runtime library.** No Rust std. Reimplement what `lib/*.deor` currently gets for free from Rust wrappers — strings, list growth/resize, formatting, I/O — as a small C or hand-written runtime linked into every binary.
9. **Build pipeline swap.** Replace `cargo build` step with `qbe` + assembler + linker invocation; update `justfile`/install scripts accordingly.
10. **Bootstrapping decision.** The transpiler itself is written in Deor and currently transpiles to Rust to run. Decide: QBE as an *additional* output target first (dual-backend), not a rip-and-replace, until parity is proven.
11. **Parity testing + cutover call.** Run `tests/` against both backends before considering QBE a real replacement rather than an experiment.
