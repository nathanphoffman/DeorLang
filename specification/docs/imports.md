# Imports

Imports use the `import` keyword followed by the names in parentheses and `in` with a string path.

```
import (Room, House, Squarefeet, total_area) in "./models"
```

Empty parens import everything from the file:

```
import () in "./models"
```

**Private declarations:** a declaration marked `private` in its source file cannot be imported. Attempting to name it in an import is a transpiler error. See [Enforced Practices — Visibility](enforced_practices.md#visibility--private).

---

## Named Imports

List the specific declarations to import between the parentheses. The transpiler filters the imported file to only those names — all other declarations are excluded from the resulting token stream.

```
import (gen_fn_decl, gen_struct_decl) in "./codegen_decl"
```

This keeps the compiled output lean and makes dependencies explicit.

---

## No Wildcard Imports

No wildcard imports are supported, it is anti-Deorian

---

## Import Resolution

Imports are resolved recursively at transpile time: if an imported file has its own imports, those are inlined first. The final token stream seen by the code generator is fully flattened — there is no module namespace at runtime.

---

**Conversion notes:**
- Imports are processed entirely at transpile time. The resulting Rust file is a single flat source — no `use` or `mod` statements are generated from Deor imports.
- The `import` keyword is consumed by the importer and does not appear in generated Rust.
