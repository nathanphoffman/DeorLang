 ---
  name as empty — yes, explicitly handled at line 206. Emits let mut name = Vec::new();. This is typeless — Rust type inference has to figure out the element
  type from later usage. If it can't, it's a Rust compile error. Per spec this should be a transpiler error (you must write roomList rooms = empty, not rooms 
  as empty).
  
  name as [] — NOT explicitly handled. It falls through all the if kind is ... checks in the KW_AS block and hits the generic expression path at line ~224.
  gen_expr → gen_primary sees LBRACKET and emits Vec::new(). So rooms as [] produces let [mut] rooms = Vec::new(); — same untyped result, same Rust inference
  problem.

  TypeName name = empty — correctly handled in the typed binding section. It checks reg3_has(type_reg, var_type) for validator types (emits Option<TypeName> =
  None) and reg_has(shape_reg, var_type) for list shapes (emits Vec<T> = Vec::new()), both with the proper type annotation.

  So the situation is:
  - The correct form (roomList rooms = empty) works properly and emits typed Rust
  - rooms as empty silently emits untyped Vec::new() instead of erroring
  - rooms as [] also silently emits untyped Vec::new() instead of erroring
  
  Both bad forms produce code that may or may not compile in Rust depending on whether type inference can fill in the element type from context. If you append
  to the list right after, Rust infers it fine. If the list is returned or passed to a function without anything being pushed into it first, Rust gives you a
  type annotations needed error.

✻ Baked for 41s

❯ can regular primitives be empty?                                                                                                                            

● No. Let me show you exactly what happens — the typed binding path in gen_stmt already handles this:

  Read 1 file (ctrl+o to expand)

● So for int val = empty:
  - reg3_has(type_reg, "int") → false (int is not a user-defined validator type)
  - reg_has(shape_reg, "int") → false (int is not a shape)
  - Falls to line 380: emits /* error: empty only valid for validator types and list shapes */

  The rejection is there and the error message is correct. The downside is the transpiler doesn't halt — it emits that comment as a statement and keeps going,
  so the Rust output would have a dangling comment where the binding should be and any subsequent use of val would be a Rust compile error.

  One other wrinkle: the none keyword on a primitive is handled separately and isn't guarded:

  # none → Option<T>
  if kind is "KW_NONE"
      return make_result(s_join([pad, "let mut ", var_name, ": Option<", rust_type, "> = None;\n"]), ...)

  So int val = none would emit let mut val: Option<i32> = None; — no error, just silently produces an Option around a primitive, which contradicts the spec's
  rule that primitives can never be null. That's a gap the empty path correctly handles but none doesn't.


❯ do we have throw in the transpiler today?                                                                                                                                      

  Searched for 1 pattern (ctrl+o to expand)

● No. That one hit is just a code comment in registry.deor. throw doesn't exist anywhere in the transpiler — not in word_to_kind, not in gen_stmt, nowhere. Writing throw msg in
  a .deor file today would tokenize throw as a plain IDENT and then confuse the statement dispatcher, likely emitting /* unhandled(IDENT) */ or mangling whatever comes after it.





# Doc Fixes
- Specify names as empty is fine in the docs and so is for arrays listString names = empty
- Rethink 

# Transpiler fixes
- We need to enforce the import syntax rather than (...) in "..." it is import (...) in "..."
- We need to enforce range(...) for for in range(...)  no for in (min,max) or for in (num) but for in collections is supported


