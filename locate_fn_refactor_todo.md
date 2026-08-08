# Locate-fn / prefix-cleanup rework — remaining phases

Rework applied to `codegen/` and `tokens_validator/` in this codebase: replace raw
`pos + N` offset arithmetic with named `locate_*` helper functions, nested locally
inside the one function that uses them (not shared globals — Deor's `fn` namespace
is flat/file-wide once imported, and duplicate names are silently "keep first,
discard second" unless `ENFORCE_UNIQUE_IMPORT_DECLARATIONS` is set, so a shared
generic name risks a silent collision). Also drop cryptic 2-4 letter macro-scoped
variable prefixes (`vd_`, `tbp_`, `td_`, etc.) in favor of full, clear names — no
prefix needed for macro-local variables since macro bodies are auto-`block`-scoped
and can't leak or collide with siblings.

## Done

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

## Remaining

3. **Struct declarations** — `codegen/decl/struct.deor`,
   `prescan_check_struct_fields.deor` (already touched once for the hang fix, not
   yet for prefix/offset cleanup), `prescan_check_duplicate_decls.deor`
4. **Enum declarations** — `check_enum_decl.deor`,
   `prescan_check_enum_variants.deor`, `codegen/decl/enum.deor`
5. **Shape declarations** (`list of T` / `func of T to U` / `raw`) —
   `codegen/decl/shape.deor`, `codegen/decl/raw.deor`,
   `codegen/decl/macros/shape_list.deor`, `codegen/decl/macros/shape_func.deor`,
   and `check_func_shape_multi_param.deor` (shape-related, not fn-related, despite
   the name)
6. **Variable decls / bindings** — `check_var_decl.deor`, `check_void_var.deor`,
   `reassign/*` (3 files), `stmt/typed_binding.deor` + `tb_*` macros,
   `stmt/as_binding.deor` + `aas_*` macros
7. **Statement/control-flow codegen** — `stmt.deor`, `if.deor`, `for.deor`,
   `for_loop/*`, `call_stmt.deor`, `list_mutation.deor`, `stmt_structural.deor`,
   `stmt_flow.deor`, `stmt_blocks.deor`, destructure macros
8. **Expression parsing** — `expr/expr.deor`, `expr/primary.deor`, `expr/macros/*`
   (prefix_ops, paren_expr, literals, list_literal, ident_expr)
9. **Use-after-move tracking** — `use_after_move/*` (7 files), `track/*` (3 files)
10. **Misc syntax-rule checks** — `syntax_rules/*` (8 files), `idents/*` (4),
    `builtins/*` (2), `brackets_parens/*` (4), plus loose ones
    (`check_undefined_var_read.deor`, `check_avow_target.deor`,
    `skip_rust_block.deor`)
11. **Import/lexer/registry infrastructure** — `importer/*`, `registry/*`,
    `deor_helpers.deor`, `arg_helpers.deor`, `tokens_validation.deor`,
    `codegen.deor`

Each phase: extract repeated `pos+N` into locally-nested `locate_*` fns where a
construct's offsets are reused across a file's macro/fn boundary, drop abbreviated
prefixes, rebuild (`just build-transpiler && just rebuild-binary`), run the full
suite (`just test-examples`), then move on — so a break is always traceable to one
phase.
