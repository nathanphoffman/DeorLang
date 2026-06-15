# deor:strings

Standard string operations beyond the built-in `len`, `+`, and `{name}` interpolation. Import explicitly — none of these are global built-ins.

```
(contains, trim, split, to_upper, to_lower, starts_with, ends_with) in deor:strings
```

Import only what you use — the named import list is the contract.

---

## Functions

| Function | Signature | Notes |
|---|---|---|
| `contains(str, needle)` | `string, string → bool` | true if `needle` appears anywhere in `str` |
| `starts_with(str, prefix)` | `string, string → bool` | true if `str` begins with `prefix` |
| `ends_with(str, suffix)` | `string, string → bool` | true if `str` ends with `suffix` |
| `trim(str)` | `string → string` | strips leading and trailing whitespace |
| `to_upper(str)` | `string → string` | all characters uppercased |
| `to_lower(str)` | `string → string` | all characters lowercased |
| `split(str, delimiter)` | `string, string → nameList` | split on every occurrence of `delimiter`; result type requires `shape nameList = list of string` |

All arguments must be named variables already in scope — the named-args rule applies. All functions return a new string and never mutate the original. An empty `delimiter` in `split` is a transpiler error.

For operations not covered here (`replace`, `index_of`, one-sided trim, character access), use a `rust` block.

---

## Examples

```
(contains, trim, split, to_lower) in deor:strings

raw as "  Hello, World!  "
string clean = trim(raw)

query as "world"
string lower = to_lower(clean)
bool found = contains(lower, query)

csv as "apple,banana,cherry"
sep as ","
nameList parts = split(csv, sep)    # requires shape nameList = list of string at top of file
```

```rust
let raw = "  Hello, World!  ".to_string();
let clean: String = raw.trim().to_string();
let query = "world".to_string();
let lower: String = clean.to_lowercase();
let found: bool = lower.contains(query.as_str());
let csv = "apple,banana,cherry".to_string();
let sep = ",".to_string();
let parts: Vec<String> = csv.split(sep.as_str()).map(|s| s.to_string()).collect();
```

```
(starts_with, ends_with) in deor:strings

path as "/api/users"
slash as "/"
bool is_abs = starts_with(path, slash)

filename as "report.pdf"
ext as ".pdf"
bool is_pdf = ends_with(filename, ext)
```

```rust
let path = "/api/users".to_string();
let slash = "/".to_string();
let is_abs: bool = path.starts_with(slash.as_str());
let filename = "report.pdf".to_string();
let ext = ".pdf".to_string();
let is_pdf: bool = filename.ends_with(ext.as_str());
```

`split` always returns at least one element — an input with no delimiter occurrences returns a single-element list containing the original string.

---

## Conversion Notes

| Deor | Rust |
|---|---|
| `contains(str, needle)` | `str.contains(needle.as_str())` |
| `starts_with(str, prefix)` | `str.starts_with(prefix.as_str())` |
| `ends_with(str, suffix)` | `str.ends_with(suffix.as_str())` |
| `trim(str)` | `str.trim().to_string()` |
| `to_upper(str)` | `str.to_uppercase()` |
| `to_lower(str)` | `str.to_lowercase()` |
| `split(str, delimiter)` | `str.split(delimiter.as_str()).map(\|s\| s.to_string()).collect()` |
