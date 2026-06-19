
AI DONT TOUCH THIS DOCUMENT THIS IS FOR NATE ONLY

# In progress

# More Stuff

Clarification: validator types can only be compared on bad and avowed

Important: Look to see if args are required to be named variables, should be required for more than 1 arg.

Important: Use empty for lists, bad for values, clearup that distinction empty and bad should not be interchangeable.  bad is for validator types empty for lists.  So a list of a validator type could be empty or have multiple bad values inside it, but the values inside could never be empty and the list could never be bad.  This also means lists should be banned from being validator types.  So you cant have a type natesdeal(listString: stringList) that is not allowed.  Also we should prevent [] from being used to initialize an empty array, only empty can do that.  The only place [] should be used is in composing arrays with actual data/variables.

looking into:

  Documented Incorrectly (10 items)
  - [] claimed to be a transpiler error — transpiler accepts [] for empty list init; struct_test.deor example uses it
  - Comment in tokens_validation.deor — says "camelCase" for type validators but calls is_pascal (implementation is correct, comment is wrong)
  - r_join[...] in loops.md — references a function that doesn't exist anywhere
  - crash argument count — spec says 1 string arg; transpiler emits panic!("{}", args) which is a 2-arg Rust macro with no count validation
  - collections_test.deor example — calls contains and to_upper which don't exist; should be s_contains/s_to_upper

  ---
  General Concerns
  3. hello.deor line 10 — (hello, world) = test uses = instead of in; syntax error per spec
  4. hello.deor macro inside function — macro defined inside fn void main(), which violates the flat-structure rule
  5. s_pascal only capitalizes first char — not true PascalCase conversion; would silently produce broken Rust type names if shape names ever contain
  underscores
  6. Import path extension — spec examples use "./models" (no extension) but the transpiler reads paths literally; no .deor suffix injection means spec-style
  paths would fail

