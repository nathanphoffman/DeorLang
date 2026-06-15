# Imports

Imports use the same `in` grammar as destructuring. The source is a string path for local modules or the `rust:` prefix for raw `.rs` files.

```
(Room, House, Squarefeet, total_area) in "./models"

(calculate, transform) in rust:math_utils
```

```rust
use my_crate::models::{Room, House, Squarefeet, total_area};
```

**Private declarations:** a declaration marked `private` in its source file cannot be imported. Attempting to name it in an `in` import is a transpiler error. See [Enforced Practices — Visibility](enforced_practices.md#visibility--private).

---

## Multi-line Import

For many names, wrap in parentheses with trailing commas:

```
(
    Room,
    House,
    Squarefeet,
    total_area,
    occupied_rooms,
) in "./models"
```

---

## Two-Step Import

A module can be imported as a namespace first, then destructured:

```
geo in "./geometry"
(distance, midpoint) in geo
```

If the intermediate `geo` binding is never used directly, the transpiler emits only the destructured `use` statements.

---

## `rust:` File Imports

Raw `.rs` files import via the `rust:` prefix. Functions imported this way can only be called from inside `rust` blocks — they have Rust signatures, not Deor ones.

```
(compress, decompress) in rust:codec

fn bytes compress_data(bytes data)
    rust
        codec::compress(&data)
```

See [Rust Interop](interop.md#external-rs-file-imports) for full details.

---

**Conversion notes:**
- **String path source** (`"./models"`) → local module path, resolved relative to the current file and translated into Rust's `crate::`/`super::` system.
- **`rust:` source** → `mod math_utils;` in generated output; functions callable only from `rust` blocks.
