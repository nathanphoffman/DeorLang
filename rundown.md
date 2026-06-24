# Deor Language — Rundown

Deor transpiles to Rust. Indentation-based blocks, no `{}` `;` `<>`. Human-readable first.

---

## File Order
Imports must be first — only enforced rule. Recommended order after: Enums → `const` → Types → Shapes → Structs → Functions

## Naming
- PascalCase: structs, validator types, enums
- camelCase: shapes
- snake_case: variables, functions, fields
- All identifiers ≥ 3 chars, enforced

## Types
- Primitives: `int` (i32), `float` (f64), `bool`, `string`
- `raw`: opaque Rust value — assigned from `rust` block only, consumed in `rust` block only, never a struct field
- Integer literals allow `_` separators

## Variables
- `Type name = expr` — explicit typed binding
- `name as literal` — type-inferred from literal or `[items]` only; no type prefix, no rebinding from variable
- `const Type name = val` — immutable, function-scope only

## Structs
- Immutable — no field mutation ever
- Pure data — no func shape fields allowed
- Construct: `Type name = (field1, field2)` — all fields must be named vars in scope, matched by name
- Destruct (only way to access fields — no dot syntax): `(field1, field2) in val` — any subset, any order
- Update (produces new struct): `new as old with (field1, field2)`

## Validator Types (`type`)
- Wraps a primitive with a boolean predicate; result is `Option<T>` under the hood
- Predicate body is mandatory
- Assignment runs predicate at runtime: passes → valid (`Some`), fails → not valid (`None`)
- Declaring without a value starts not valid: `Roll best`
- Assigning a value that fails the predicate also produces not valid
- No `bad`, no `empty` for validator types — invalid state is implicit, not assigned
- Check with `if val valid` / `if val not valid`
- Extract with `(avow val)` — panics if not valid; parens always required; must capture to variable

## Shapes
- Named type aliases — the only way to use list or func types
- List shape: `shape roomList = list of Room` → `Vec<Room>`
- Func shape: `shape filterFunc = func of Room to bool` — at most one input and one output
- Func shapes cannot be struct fields

## Enums
- Named set of plain variants, no associated data
- Assign: `ColorTag bg = Blue`
- Check with `is` / `is not` in `if`/`else if` chains — no pattern matching

## Functions
- Signature: `fn ReturnType name(Type param, ...)` — return type is prefix
- `void` is mandatory for functions returning nothing
- Single-expression body: implicit return. Multi-statement: explicit `return` everywhere
- Max 3 parameters — bundle extras into a struct
- No lambdas, closures, or nested functions
- Recursion allowed
- Validator return type: return `bad` when no valid value; never `return empty`
- Multi-return: declare a named struct, return `(field1, field2)` in return position only
- Entry point: `fn void main()`


## Operators
- Arithmetic: `+` `-` `*` `/` `%`; integer division truncates; no `**`
- Comparison: `is` `is not` `<` `>` `<=` `>=`
- Logical: `and` `or` `not`
- `==` `!=` `&&` `||` are transpiler errors — use word forms
- No bitwise operators — use `rust` block
- `is empty` / `is not empty` for lists

## Collections
- Declare a shape first, then: `roomList rooms = empty` to init
- `[a, b, c]` constructs a list literal — all same type, all named vars
- `list at idx` — read; `list at idx = val` — write; `list at end = val` — append
- `list remove at idx` — remove, shifts remaining elements left
- `len(list)` — element count; `len(string)` — char count
- No membership operator — write explicit loops
- Update struct inside list: read → `with` → write back; no in-place field mutation

## Loops
- `for item in collection` — collection iteration
- `for idx in range(count)` — 0 to count-1; `range(start, end)` exclusive upper bound
- `for in range(n)` — no index needed
- `for idx in (low, high)` — bare tuple range
- `for if condition` — while loop; `for if true` — infinite loop
- `break` / `continue` — innermost loop only; no labeled breaks

## Conditionals
- `if` / `else if` / `else` — no parens around condition
- No `match` — use `if`/`else if` with `is` for enum dispatch

## Built-ins (no import)
- `print(val)` — stdout + newline, any primitive
- `len(val)` — length of list or string
- `range(n)` / `range(start, end)` — for-loop iteration source only, not a value
- `crash(message)` — panic!
- `(avow val)` — unwrap validator type to raw primitive

## Imports
- `import "path/file.deor"` — must be at top of file before all declarations
- All declarations are global; no private visibility
- Each file loaded once; duplicate imports silently ignored
- Depth-first resolution: dependencies always precede their importers

**Parameterized imports** — lib files that use `where T = Type` are specialized at import time via textual substitution:
- `import "lib/list.deor" where T = int` — import once per concrete type needed
- Naming after substitution: bare `T` → type name; `TSuffix` → `TypeSuffix`; `tSuffix` → `typeSuffix`; `t_T_fn` → `t_type_fn`

## Standard Library
All imported with `import "lib/name.deor"`. Functions must be called with named variables (2+ args).

- **lib/string.deor** — `s_trim`, `s_to_upper`, `s_to_lower`, `s_contains`, `s_starts_with`, `s_ends_with`, `s_split`
- **lib/math.deor** — `m_abs`, `m_sign`, `m_min`, `m_max`, `m_clamp`, `m_pow`, `m_sqrt`, `m_floor`, `m_ceil`, `m_round`, `m_log`, `m_log2`, `m_log10`; float variants: `m_absf`, `m_minf`, `m_maxf`, `m_clampf`, `m_powf`
- **lib/random.deor** — `m_rand_int(min, max)`, `m_rand_float()`, `m_rand_bool()`
- **lib/convert.deor** — `c_int_to_float`, `c_float_to_int`, `c_int_to_string`, `c_float_to_string`, `c_bool_to_string`, `c_string_to_int`, `c_string_to_float`, `c_string_to_bool`
- **lib/list.deor** (parameterized) — after `where T = int`: shape `lIntList`; fns `l_int_first`, `l_int_last`, `l_int_reverse`, `l_int_slice`, `l_int_concat`, `l_int_sort`, `l_int_sum`, `l_int_min`, `l_int_max`, `l_int_join`
- **lib/tstack.deor** (parameterized) — after `where T = string`: shape `tStringStack`; fns `t_string_make`, `t_string_size`, `t_string_get`; push/pop use `at end =` / `remove at`
- **lib/tasks.deor** (parameterized) — pool-bounded parallel map; after `where T = Score`: shape `scoreList`, `scoreTransformFunc`; fn `t_score_run_all(pool, jobs, worker)` — blocks until all results collected, returns in completion order not input order

## Strings
- Concat with `+`; mixed `string + int` in same chain is a transpiler error
- Escape sequences: `\n` `\t` `\\` `\"`

## `move` Keyword
Deor clones everything by default. `move` transfers ownership instead — original variable inaccessible after.
- Function arg: `do_something(move big_list)`
- Struct construction: `Score built = move (label, points)` — fields consumed
- Loop: `for move (item in collection)` — collection consumed after
- Assignment: `string new_var = move prev_var`

## `rust` Blocks
- Raw Rust inlined by indentation under `rust` keyword
- Deor params available by name; last expression is the return value
- Use for: crates, HashMap, async, bitwise, type casts, anything Deor can't express
- Keep blocks small; wrap in a Deor function for reuse

## `block` Keyword
- Opens a scoped block — variables declared inside do not leak into the outer scope
- Useful paired with macros to prevent variable pollution

## Macros
- `macro name` / `macro_run name` — copy-paste injection at transpile time
- Variables pollute caller scope; use `block` keyword inside macro to isolate
- Declared and imported like any other top-level declaration

## Custom Wrappers
Wrap Rust functions in a Deor function using a `rust` block. One thing per block, keep them small.
- `s_fn` — std Rust wrapper (e.g. `s_join`, `s_trim`)
- `cx_fn` — Cargo crate wrapper (e.g. `cx_rand_int`); add crate to `Cargo.toml` manually
- `ex_fn` — personal/third-party Deor lib (e.g. `ex_do_cool_thing`)

## Key Constraints
- `empty` only valid at declaration for list shapes; not valid for validator types
- `avow` parens always required and result must be captured to a variable
- All `()` args must be named variables when calling with 2+ args; 1-arg calls allow literals
- Built-ins accept literals regardless of arg count
- `raw` variables: rust block in, rust block out only
- `as` never takes a type prefix and never rebinds from a plain variable
