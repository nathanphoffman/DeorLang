# Imports

Imports use the same `in` grammar as destructuring. The source is either a bare identifier (stdlib/external crate) or a string path (local module).

```
(sqrt, floor) in math

(trim, split) in strings

(
    Room,
    House,
    Squarefeet,
    total_area,
    occupied_rooms,
) in "./models"

geo in "./geometry"
(distance, midpoint) in geo

(format_address as fmt_addr) in "./utils"
```

```rust
use my_crate::models::{Room, House, Squarefeet, total_area, occupied_rooms};
use my_crate::geometry::{self as geo, distance, midpoint};
use my_crate::utils::format_address as fmt_addr;
// (sqrt, floor) resolve to whatever crate/std path the
// stdlib-equivalence table maps `math` to
```

**Private declarations:** a declaration marked `private` in its source file cannot be imported. Attempting to name it in an `in` import is a transpiler error. See [Enforced Practices — Visibility](enforced_practices.md#visibility--private).

---

**Conversion notes:**
- **Bare identifier source** (`math`, `strings`) → external crate or `std` module path, via a curated stdlib-equivalence table maintained by the transpiler.
- **String path source** (`"./models"`) → local module path, resolved relative to the current file and translated into Rust's `crate::`/`super::` system. The transpiler must also emit the corresponding `mod` declarations.
- **`Name as alias in source`** maps almost exactly onto Rust's `use path::Name as alias;` — one of the cleanest 1:1 conversions in the spec.
- **Two-step imports** (`geo in "./geometry"` then `(distance, midpoint) in geo`) — if the intermediate `geo` namespace binding is never used directly, the transpiler can drop it and emit only the destructured `use` statements.
