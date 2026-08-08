# Locate-fn / prefix-cleanup rework — complete

Rework applied to `codegen/` and `tokens_validator/` in this codebase: replace raw
`pos + N` offset arithmetic with named `locate_*` helper functions, nested locally
inside the one function that uses them (not shared globals — Deor's `fn` namespace
is flat/file-wide once imported, and duplicate names are silently "keep first,
discard second" unless `ENFORCE_UNIQUE_IMPORT_DECLARATIONS` is set, so a shared
generic name risks a silent collision). Also drop cryptic 2-4 letter macro-scoped
variable prefixes (`vd_`, `tbp_`, `td_`, etc.) in favor of full, clear names — no
prefix needed for macro-local variables since macro bodies are auto-`block`-scoped
and can't leak or collide with siblings.

All 11 phases done. Verified at every phase via `just build-transpiler &&
just rebuild-binary && just test-examples` (94/94 passing), plus a final
double self-compilation round-trip to confirm the self-hosted transpiler is
fully stable.

**Post-phase-11 gap sweep:** the original file survey undercounted — it
grouped by directory but never separately enumerated `prescan/` or a few
standalone macro files, so they silently fell outside every phase. Found and
fixed via two systematic scans (a precise `pos ± N` regex and a repeated-prefix
frequency scan) after claiming completion:
- `tokens_validator/macros/raw/*` (4 files: `check_raw_in_binding`,
  `check_raw_operator_use`, `check_raw_assignment`,
  `check_raw_in_special_builtin`) — an entire subdirectory missed outright.
- `check_type_base_primitive.deor`, `check_validator_empty.deor`,
  `check_ident_token_rules.deor` — declaration/dispatcher files never
  captured by any phase grouping.
- `gen_expr_r.deor`, `expr_is_special.deor` — small helper macros.
- `gen_enum_extract_check.deor` — 51 lines using one cryptic acronym
  (`geec_`) 53 times, nearly every line.
- The entire `prescan_collect_*` family (11 files): all 7
  `prescan_collect_declared_vars_*.deor` (`dv_` prefix),
  `prescan_collect_const_names.deor` (`cn_`),
  `prescan_collect_fn_names.deor` (`fnn_`),
  `prescan_collect_func_shapes.deor` (`cfs_`),
  `prescan_collect_validator_types.deor` (`pvt_`).
- `codegen/decl/stmt/block.deor` (`gen_block`, a central codegen function)
  had numbered `mc1`–`mc5` variables — never touched despite being about as
  foundational as it gets.
- `tokens_validator/error_handling.deor` (`val_err`/`handle_errors`, used by
  every single check in the codebase) — cleaned for full consistency.
- Minor internal inconsistency fixed in the use-after-move family: a few
  `pos - 1` lookbacks that should have been wrapped in a `locate_prev_token`
  like their siblings were, weren't.

Re-verified with the same build/test/self-compile cycle after each batch of
fixes. The two scans that caught this are worth re-running if this pattern
of work continues elsewhere in the codebase — directory-based file surveys
miss subdirectories and stray files; a regex/frequency scan over actual file
content doesn't.

## Phases

1. **Type/validator declarations** — `check_validator_declaration.deor`,
   `codegen/decl/validator_type.deor`. Includes the fix for finding #1 (empty
   validator predicate body crashing the transpiler).
2. **Function declarations** — `check_fn_declaration.deor`,
   `codegen/decl/function.deor`, `codegen/decl/macros/fn_parse_signature.deor`.
   Also added general nested-fn support (a `fn` may now nest inside another `fn`,
   restricted to a single `return expr` body) to make step 1 possible in the first
   place, and fixed a real pre-existing bug found along the way:
   `prescan_check_struct_fields.deor` could hang the transpiler forever on any
   nested block inside a struct body (not depth-aware DEDENT tracking).
3. **Struct declarations** — `codegen/decl/struct.deor`,
   `prescan_check_struct_fields.deor`, `prescan_check_duplicate_decls.deor`
   (also covers struct/enum/shape/type's shared "builtin name as decl name"
   check and fn name, so it's done here rather than split across phases).
4. **Enum declarations** — `check_enum_decl.deor`,
   `prescan_check_enum_variants.deor`, `codegen/decl/enum.deor`
5. **Shape declarations** (`list of T` / `func of T to U` / `raw`) —
   `codegen/decl/shape.deor`, `codegen/decl/raw.deor`,
   `codegen/decl/macros/shape_list.deor`, `codegen/decl/macros/shape_func.deor`,
   and `check_func_shape_multi_param.deor` (shape-related, not fn-related, despite
   the name)
6. **Variable decls / bindings** — `check_var_decl.deor`, `check_void_var.deor`,
   `reassign/*` (all 5: `check_bare_reassign.deor` plus its two callers
   `check_const_reassign.deor`/`check_validator_reassign.deor`, since renaming its
   parameterization vars meant updating both call sites too), `stmt/typed_binding.deor`
   + all 6 `tb_*` macros, `stmt/as_binding.deor` + all 4 `aas_*` macros.
7. **Statement/control-flow codegen** — `stmt.deor`, `if.deor`, `for.deor`,
   `for_loop/*` (`for_while`, `for_move`, `for_iter_expr`, `for_build_fields`;
   `for_collect_fields.deor` was already clean, no changes needed),
   `call_stmt.deor`, `list_mutation.deor`, `stmt_structural.deor`,
   `stmt_flow.deor`, `stmt_blocks.deor`, `destructure.deor`,
   `initialize_gen_destructure.deor`, and the whole `input_destructure/` family
   (`gen_input_check`, `gic_match_kw_and_name`, `gic_match_parens`,
   `gic_emit_header`, `gic_emit_bindings` — same cross-file ambient-variable
   rename pattern as the reassign macros in phase 6).
8. **Expression parsing** — `expr/expr.deor`, `expr/primary.deor`, `expr/macros/*`
   (prefix_ops, paren_expr, literals, list_literal, ident_expr).
9. **Use-after-move tracking** — all of `use_after_move/*` (`check_move_target`,
   `check_use_after_move_field`, `check_use_after_move_var_for`,
   `check_use_after_move_var_ident`, `check_use_after_move_var_move`,
   `check_use_after_move_chain`, `track_copy_vars`; `check_use_after_move.deor`
   and `check_use_after_move_var.deor` were already clean dispatchers) and
   `track/*` (`track_block_scope` — touched again to rename `UamFrame`'s and
   `VoidFnFrame`'s own field names, since struct construction/destructure binds
   by matching local variable name to field name — `track_non_bool_vars`,
   `track_validator_vars`; `track_paren_depth.deor` was already clean).
10. **Misc syntax-rule checks** — all 12 `syntax_rules/*` files (2 already
    clean: `check_void_return`, `check_rust_generic`), all 4 `idents/*` (1
    already clean: `validate_ident` — its offset is a caller-parameterized
    variable, not a fixed constant), all 4 `builtins/*` (a cross-file
    ambient-variable rename, same pattern as phase 6/7), all 7
    `brackets_parens/*` (1 already clean: `check_keyword_in_parens`), and the
    3 loose files (`check_undefined_var_read.deor`, `check_avow_target.deor`,
    `skip_rust_block.deor`). Hit and fixed a real naming collision along the
    way: `valid` is the reserved `KW_VALID` keyword, can't be used as a Deor
    variable name (renamed to `is_valid` in the 3 files that tried).
11. **Import/lexer/registry infrastructure** — all of `registry/*` (`shape`,
    `enum`, `struct`, `validator_type`, `mut_scan`, `registry`;
    `type_resolve.deor` was already clean), the importer/dedup pipeline
    (`scan.deor`, `load.deor`, the `sep_*`/`strip_enforce_pragmas` pragma
    family — another cross-file ambient rename — and the `dd_*` dedup family,
    another one), the lexer (`tokenizer.deor`, `collect_rust_block.deor`,
    `emit_indent_or_dedent.deor` — caught and fixed a real ambient-variable
    break here: renaming `tokenizer.deor`'s loop vars broke
    `collect_rust_block.deor`'s reference to them, since it reads them as
    ambient from that same loop), `deor_helpers.deor` (already clean),
    `codegen/codegen.deor`, and `tokens_validator/arg_helpers.deor`.

## Notable things found along the way

- **Nested-fn support is a real, permanent language feature now** — a `fn` may
  nest inside another `fn`, restricted to a body of exactly one `return expr`
  statement (validated by `is_single_return_body` in
  `check_fn_declaration.deor`). This is what let every `locate_*` helper
  become a private, zero-risk nested function instead of a shared global.
- **Struct construction/destructure binds by matching local variable name to
  the struct's field name**, not by position — renaming a struct's own field
  names (`UamFrame`, `VoidFnFrame`) requires updating every call site that
  constructs or destructures it to use the new names exactly.
- **Cross-file "ambient variable" macro families** (one macro sets locals
  that sibling macros read/write without any parameter list) appear
  throughout this codebase — reassign checks, input-destructure, the pragma
  scanner, the dedup family. Renaming any of their shared names means
  updating every file in the family together, verified by grepping for the
  old names after each rename.
- **`valid` is a reserved keyword** (`KW_VALID`) — can't be used as a plain
  variable name in Deor source, including the transpiler's own.
