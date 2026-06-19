

# In progress

 - Void-input/output func shapes — func to bool and func of Error read wrong token positions; registry stores garbage

- Destructuring and three argument limitations

# More Stuff

  ---
  Implemented But Undocumented (9 items)

  - for (start, end) range syntax — used in examples, no mention in loops.md
  - BLOCK_START/BLOCK_END tokens — internal synthetic tokens from macro expansion
  - var_type_reg — variable type registry for using support
  - Legacy macro_define / place — old macro syntax still wired up but entirely absent from docs
  - Deduplication behavior — first occurrence wins when merging imports; later definitions silently dropped

  ---
  Documented Incorrectly (10 items)

  - Named-arg rule threshold — spec says all args must be named variables; transpiler only checks when there are 2+ args (single-arg literals pass silently)
  - [] claimed to be a transpiler error — transpiler accepts [] for empty list init; struct_test.deor example uses it
  - avow statement context — checks var_type is "int" etc. to decide whether to add .0, but tests the binding type not the validator's base type
  - Comment in tokens_validation.deor — says "camelCase" for type validators but calls is_pascal (implementation is correct, comment is wrong)
  - r_join[...] in loops.md — references a function that doesn't exist anywhere
  - crash argument count — spec says 1 string arg; transpiler emits panic!("{}", args) which is a 2-arg Rust macro with no count validation
  - collections_test.deor example — calls contains and to_upper which don't exist; should be s_contains/s_to_upper

  ---
  General Concerns

  1. Collection loops missing & — for room in rooms emits for room in rooms { which moves the collection; Rust requires &rooms to borrow it — this would cause
  compile errors for most loops
  2. bad/empty transition is half-done — both keywords coexist with no policy; docs are mid-migration
  3. hello.deor line 10 — (hello, world) = test uses = instead of in; syntax error per spec
  4. hello.deor macro inside function — macro defined inside fn void main(), which violates the flat-structure rule
  5. s_pascal only capitalizes first char — not true PascalCase conversion; would silently produce broken Rust type names if shape names ever contain
  underscores
  6. Import path extension — spec examples use "./models" (no extension) but the transpiler reads paths literally; no .deor suffix injection means spec-style
  paths would fail

  ---
  The biggest functional gaps worth prioritizing: float literals, collection loop & borrowing, the import keyword doc error, the named import filtering not
  actually filtering, and the avow .unwrap().0 mismatch in expression context. Those will cause actual Rust compile errors in real code.
