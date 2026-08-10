# Documentation vs. transpiler audit — findings

Audit date: 2026-08-06. Scope: all files under `specification/` cross-checked against
`transpiler-deor/` source and actual compiler behavior (verified by transpiling/compiling
throwaway snippets through `deor` + `rustc`, not just reading source). No repo files were
changed as part of the audit itself.

Severity key:
- **code bug** — the transpiler does something wrong/inconsistent, independent of the docs.
- **doc wrong** — the doc states something false or contradicted by actual behavior.
- **undocumented** — real, intentional behavior with no doc coverage.

---

## Critical code bugs

Fixed:
1. **Empty validator predicate body can crash the transpiler itself.** `type Foo(int val)`
   with no predicate body doesn't just fail to validate — depending on position in the file
   it can panic the transpiler process with an internal `index out of bounds` error (exit
   101) instead of reporting a clean error. No check exists for a missing predicate body.
   (`codegen/decl/validator_type.deor`; docs/validator_types.md claims this is rejected —
   it is not, in either failure mode.) Fixed via `check_validator_declaration.deor`'s
   `has_body` guard, emitting `rule_validator_missing_body` instead of reading out of
   bounds. Regression tests: `validator_missing_body_test.deor`,
   `validator_missing_body_eof_test.deor`.

2. **`==` is not actually rejected**, though `docs/operators.md` (lines 117-127) lists it
   alongside `!=`/`&&`/`||` as a banned symbolic operator that's a transpiler error. `!=`,
   `&&`, `||` really do fail (their characters are invalid outside strings — see
   `tokens_validator/macros/check_invalid_char.deor`). `==` doesn't: `=` is a valid token
   (EQUALS) and nothing scans for two adjacent EQUALS. `if val == 5` transpiles with exit 0,
   emitting broken Rust that only fails later at `rustc` with a confusing `E0308` type
   error instead of a clear Deor-level message. Root cause: `is_binary_op` in
   `codegen/decl/stmt/expr/expr.deor` doesn't include EQUALS, so `gen_expr` just stops
   folding and leaves the tokens dangling. Fixed via a new `check_double_equals.deor`
   validator check. Regression test: `double_equals_banned_test.deor`.

3. **Bare `if <validator_var>` (no `is valid`) is not rejected**, though
   `docs/variables.md` (lines 81-87) explicitly says it's a transpiler error. It compiles
   clean to `if sqft { ... }` where `sqft: Option<Squarefeet>`, then fails at `rustc` with
   `E0308`. Root cause: `track_non_bool_vars.deor` deliberately excludes validator types
   from `non_bool_var_names`, and `check_bare_truthiness.deor` only checks that list — the
   validator-specific truthiness check was apparently never written. Fixed by having
   `check_bare_truthiness.deor` also check the existing `validator_vars` list (already
   populated by `track_validator_vars.deor` for other checks). Regression test:
   `validator_bare_truthiness_test.deor`.

5. **`range()` and `len()` have zero argument-count validation.** Unlike `print`/`crash`
   (which have dedicated `check_print_args`/`check_crash_args` macros), no such check
   exists for `range` or `len`.
   - `range(one, two, three)` — 3 named args — transpiles, compiles, and **runs**,
     silently dropping `three` and iterating `one..two`. Silent wrong behavior, not even a
     compile error.
   - `range()` / `len()` with 0 args transpile "successfully" (exit 0) but emit invalid
     Rust (`/* unknown_primary */` in the argument position), failing only at `rustc`.
   - `docs/builtins.md` (lines 47-72) documents only the intended forms and doesn't
     disclose that malformed calls aren't caught by Deor.
   Fixed via new `check_range_args.deor`/`check_len_args.deor`, reusing the existing
   `check_builtin_arg_count` helper print/crash already used. Regression test:
   `range_len_args_test.deor`.

6. **A macro called from outside the function it's locally scoped in silently vanishes** —
   zero code emitted, zero diagnostic. `expand_deor_macros` removes out-of-scope local
   macro definitions from its lookup map on `DEDENT`, and the `macro_run` expansion site
   has no `else` branch for "not found," so the call site just disappears from the output.
   `docs/macros.md` says local macros aren't "visible outside" their block but doesn't
   mention that calling one from outside fails silently instead of erroring. Fixed by
   adding that `else` branch in `expand_deor_macros` (`macro_expander.deor`). Regression
   test: `macro_out_of_scope_test.deor`.

4. **`(avow value) + 2` silently drops the `+ 2`.** This is `docs/validator_types.md`'s own
   worked example (line ~195) for parenthesized avow used in an expression. Actual
   behavior: transpiles and compiles, but silently discards everything after the closing
   paren and prints only the unwrapped value. Root cause: `codegen/decl/stmt/macros/typed_binding/tb_paren.deor`
   (lines ~44-69) returns immediately after parsing the parenthesized avow expression,
   never continuing to parse a trailing operator. Avow *without* the outer parens, or with
   parens around the right-hand operand instead, both work correctly. Fixed by having
   `tb_paren.deor` continue folding trailing binary operators after the closing paren
   (mirroring `gen_expr`'s own fold loop) instead of returning immediately. Regression
   test: `avow_paren_trailing_op_test.deor`.

7. **The docs' flagship function example doesn't actually work.**
   `docs/functions.md`'s implicit-return style (`fn int add(int a, int b)` with a bare
   `left + right` as the last line, no `return`) has no codegen path — `gen_stmt`
   (`codegen/decl/stmt/stmt.deor`) has no handler for a bare trailing expression statement,
   so it falls through to a catch-all that emits `/* unhandled(IDENT) */` per token.
   Transpiles with exit 0; fails `rustc` with a type mismatch. Every single-expression-body
   example in the doc is affected. Explicit `return` works fine. No validator catches a
   function body missing a `return` at its exit point either — same broken-codegen result.
   Fixed in two parts: (1) detecting a single-bare-expression function body in
   `fn_build_body_ctx.deor` (before `gen_block`/`gen_stmt` ever see it) and emitting it as
   an explicit `return` in `fn_emit.deor` — regression test `implicit_return_test.deor`;
   (2) a new `check_missing_return_expr.deor` validator that flags a bare expression
   statement anywhere inside a function body (any nesting depth) that has more than one
   statement, since that's not the documented implicit-return shape and used to hit the
   same silent `/* unhandled */` codegen fallback — regression test
   `missing_return_expr_test.deor`.


In Progress:
8. **Destructuring a multi-return-value call invokes the function twice, not once.**
   `docs/functions.md`'s multi-return example (`(quotient, remainder) in divmod(a, b)`)
   generates code that calls `divmod` once per destructured field. Real correctness risk
   for any function with side effects (I/O, mutation, `crash()`, etc.) — silent, no
   warning anywhere.

9. **`Type x = move (f1, f2)` struct construction isn't tracked by use-after-move.**
   `docs/move.md`'s own "Struct Construction via move" example
   (`Score built = move (label, points)`) doesn't get flagged by Deor's use-after-move
   checker if `label`/`points` are read afterward — only bare `move IDENT` and
   `move (...) in source` forms are recognized
   (`check_use_after_move_var_move.deor`, `check_use_after_move_field.deor`). Passes Deor,
   fails later at `rustc` with `E0382`.

10. **Direct struct field assignment (`thing.field = val`) doesn't get a clean error — it
    silently corrupts codegen for that line *and* the following statement.**
    `docs/immutability.md` (lines 12-16) correctly says no field-assignment syntax exists,
    but doesn't mention the failure mode: because the field name can coincide with an
    already-declared local variable, the reassignment-tracking logic misfires on the token
    after the dot as if it were a fresh statement. Verified: produced a nonsense type
    (`let mut occupied: Kitchen = false;`) and a mangled `println!` on the following line.
    Fails `rustc` with unrelated errors instead of one clear Deor diagnostic.

---

## Cross-cutting: undocumented rule that breaks the docs' own example code

11. **Any call site with 2 or more arguments requires every argument to be a plain named
    variable — never a literal or expression.** (`tokens_validator/macros/builtins/check_call_args.deor`)
    This rule isn't documented in *any* of the 24 spec files, and it breaks the docs' own
    worked examples in two different places:
    - `docs/functions.md`'s recursion example implies this is a recursion-specific
      restriction; it's actually a general call-site rule that only kicks in at 2+ args
      (1-arg calls with literals/expressions are fine).
    - `docs/interop.md`'s "Global-Style References" example
      (`config = h_set(config, "host", "localhost")`) violates this rule as written and
      fails to compile with "each arg must be a named variable when passing 2 or more
      args."
    Worth documenting once, prominently (e.g. in `syntax.md` or `functions.md`), since it's
    a rule every new user will hit immediately.

---

## Doc examples that don't match actual generated Rust (cosmetic, but worth fixing)

12. `docs/collections.md`'s "Index Read" example shows `let first: i64 = scores[0];` with
    no `.clone()`/cast. Actual output: `scores[0 as usize].clone()`. The "Index Write"
    example similarly omits `.clone()` on the right-hand side that codegen always adds.

13. `docs/shapes.md`'s Conversion Notes table (line ~230) claims a list argument is passed
    by reference (`&rooms`). It isn't — every plain-identifier call argument gets
    `.clone()`'d (`codegen/decl/stmt/expr/call_args.deor`); no reference-passing codegen
    exists anywhere for this.

14. `docs/builtins.md`'s print-separator example (lines 32-43) declares
    `string separator = ", "` in the Deor source but shows the Rust translation using
    `sep.clone()` — a leftover from an earlier variable name. Actual generated code
    correctly uses `separator.clone()`.

15. `docs/enums.md` (line 183) states a typed enum can't be used as a variable's type
    (`Color background = Red`), implying it's rejected. No validator check exists for this
    — it transpiles clean and fails only at `rustc` ("cannot find type `Color`").

16. `docs/validator_types.md`'s `Squarefeet area = -1` example is claimed to "transpile and
    compile fine" — it doesn't; it fails at `rustc` (E0061). This is really the pre-existing
    unary-minus-has-no-codegen gap (see prior session notes) tripping up this specific
    example, not a new bug.

---

## Undocumented but real behavior

17. **`raw TypeName` declarations** — an entire feature (opaque Rust-backed types, distinct
    from `shape`) — has no documentation. `docs/shapes.md`'s intro mentions "named byte
    buffers" as one of three things shapes cover, but the doc body only ever describes
    `list of T` and `func of T to U`; `raw` is never given syntax or an example.
    (`registry/shape.deor`, `codegen/decl/raw.deor`.)

18. **4 stdlib string functions are missing from `docs/libs.md`**: `s_chars`,
    `s_pad_left`, `s_pad_right`, `s_split_whitespace` (all real, working functions in
    `lib/string.deor`). Confirmed independently by two separate audit passes.

19. **`ENFORCE_UNIQUE_*` pragma docs are incomplete.** `docs/enforced_practices.md`
    (line 131) lists `struct`/`enum`/`shape`/`type`/`fn` as the covered declaration forms.
    In reality `macro`, `unsafe_macro`, and `raw` are checked with identical semantics
    (`importer/dedup.deor`, `importer/macros/dedup/dd_handle_raw.deor`) — verified live.

20. **Identical global `rust { }` blocks are always silently deduplicated, regardless of
    the `ENFORCE_UNIQUE_*` pragmas.** Two files with a byte-identical top-level `rust`
    block, imported together with both strict pragmas set, compile cleanly with no
    duplicate-symbol error — the strict-mode pragmas don't apply to raw blocks at all.
    Not mentioned in `docs/enforced_practices.md` or `docs/imports.md`.

21. **Bare `name as (f1, f2, ...)` struct-construction inference has zero field
    validation**, unlike the explicit `Type name = (f1, f2, ...)` form which is checked
    against the struct registry. If no struct matches the given field names, it silently
    emits `Unknown { ... }`, only failing at `rustc`. If *multiple* structs share the same
    field set, it silently picks whichever was declared first in the file, with no
    warning. `docs/structs.md` only caveats the explicit form's type-checking gap, not this
    one, which has no diagnostic at all.

22. **Struct field names cannot shadow builtin function names** (`print`, `crash`, `len`,
    `range`, `args`, `input`) — a real, enforced rule
    (`tokens_validator/macros/prescan/prescan_check_struct_fields.deor`) not mentioned in
    `docs/structs.md`.

23. **Flow-sensitive move tracking across if/else-if/else chains** is a substantial feature
    (~150 lines, `check_use_after_move_chain.deor`, matches rustc's own reachability logic)
    that `docs/move.md` never mentions at all — the doc doesn't discuss `if`/`else`
    interaction with moves in any form. Related undocumented details: the self-reassignment
    idiom `x = f(move x)` is specially exempted from poisoning; a double-move
    (`move x` twice) gets its own distinct error message from ordinary use-after-move.

24. Lexer silently accepts *unrecognized* string escape sequences (e.g. `\r`, `\x`) rather
    than erroring — it re-emits the literal backslash + character untouched.
    `docs/strings.md` says "no other escape sequences are supported in v1," true of the
    effect, but doesn't disclose that unsupported ones are silently passed through rather
    than rejected.

---

## Checked and confirmed correct (no action needed)

For reference — these specific, testable doc claims were verified against real compiler
runs and matched exactly: operator precedence (Deor emits flat infix Rust and defers to
`rustc`'s own precedence, as documented); `not x is y` → error / `x is not y` → ok;
`a as move b` rejection; `const` naming + reassignment rules; banned Rust type names;
builtin-name shadowing rejection; integer division truncation; `for if true` → bare
`loop {}`; hex literal rejection (deferred to v2); underscore numeric literals;
`print`/`crash` argument-count + type enforcement; `args()`/`input()` destructuring
defaults; max-3-parameters; min-3-char identifier + snake_case rules; duplicate-declaration
default "keep first, silent" behavior including the `ENFORCE_UNIQUE_*` pragma mechanics for
the 5 documented forms; `ENFORCE_MACRO_FILE_DEPTH` semantics; parameterized generic imports
(`where T = X`, including compound-identifier substitution like `l_T_first` →
`l_int_first`) — verified working correctly with the required `where T = ...` syntax; the
full function signature tables for `math.deor`, `random.deor`, `convert.deor`, `list.deor`,
`list_order.deor`, `list_numeric.deor`, `map.deor`, `file.deor`, `time.deor`, `tasks.deor`,
`taskpool.deor`; `s_split`/`s_char_at`/`s_index_of`/`s_repeat` codegen; Copy-type move
no-ops; `for move (item in collection)` consumption semantics; all 5 `raw`-variable
restrictions in `docs/interop.md`; `unsafe_macro`/`macro` scoping and nesting-rejection
rules.

---

## False alarm (corrected during synthesis)

One sub-audit initially reported the list stdlib (`lib/list.deor`, `lib/list_order.deor`,
`lib/list_numeric.deor`) as "dead on arrival" — failing Deor's own snake_case validator
because their `T`-placeholder function names (e.g. `l_T_first`) contain an uppercase `T`.
This was a false alarm caused by importing those files *without* the required
`import "lib/list.deor" where T = int` substitution syntax. Imported correctly, the
substitution renames `l_T_first` → `l_int_first` etc. before validation ever sees it, and
the module works as intended — confirmed both by this session's own passing test suite
(`tests/unit_tests/list_generic_test.deor` and siblings) and independently by a second
audit pass using the correct import form.