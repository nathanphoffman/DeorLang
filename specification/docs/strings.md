# Strings

String utility functions live in `lib/string.deor`. Import the whole file:

```
import "lib/string.deor"
```

All functions in `lib/string.deor` follow the `s_` prefix convention. They are regular user-defined functions, so arguments must be named variables — literals are not valid arguments. See [Enforced Practices](docs/enforced_practices.md#named-arguments-user-defined-functions-only).

---

## Escape Sequences

Standard escape sequences are supported inside string literals:

| Sequence | Meaning |
|---|---|
| `\n` | Newline |
| `\t` | Horizontal tab |
| `\\` | Literal backslash |
| `\"` | Literal double quote |

```
msg as "Hello\nWorld"
path as "C:\\Users\\name"
quote as "She said \"hello\""
```

```rust
let msg = "Hello\nWorld".to_string();
let path = "C:\\Users\\name".to_string();
let quote = "She said \"hello\"".to_string();
```

No other escape sequences are supported in v1. For Unicode escapes or raw byte strings, use a `rust` block.

---

## Concatenation

`+` joins strings. It works with literals, variables, or any combination:

```
string greeting = "hello " + name
string line = prefix + content + "\n"
string full = first + " " + last
```

```rust
let greeting: String = "hello ".to_string() + &name;
let line: String = prefix + &content + "\n";
let full: String = first + " " + &last;
```

Chains of `+` are evaluated left to right and compiled to a native Rust `+`/`&` chain, not `format!`. The first operand becomes an owned `String` (a literal gets `.to_string()`; a variable or call is already owned). Every operand after that is borrowed with `&`, except string literals, which are already `&str` and need no borrow.

The transpiler does not check that all operands in a `+` chain are strings — mixing in a non-string operand (e.g. an `int`) is not caught at the Deor level and will fail with a Rust type error instead. Use a `rust` block if you need to format an integer into a string:

```
fn string int_to_str(int n)
    rust
        n.to_string()

string msg = "count: " + int_to_str(count)
```

---

## Examples

```
import "lib/string.deor"

string raw = "  Hello, World!  "
string clean = s_trim(raw)

string lower = s_to_lower(clean)
string query = "world"
bool found = s_contains(lower, query)

string csv = "apple,banana,cherry"
string sep = ","
stringList parts = s_split(csv, sep)
```

```rust
let raw: String = "  Hello, World!  ".to_string();
let clean: String = raw.trim().to_string();
let lower: String = clean.to_lowercase();
let query: String = "world".to_string();
let found: bool = lower.contains(query.as_str());
let csv: String = "apple,banana,cherry".to_string();
let sep: String = ",".to_string();
let parts: Vec<String> = csv.split(sep.as_str()).map(|s| s.to_string()).collect();
```

```
string path = "/api/users"
string slash = "/"
bool is_abs = s_starts_with(path, slash)

string filename = "report.pdf"
string ext = ".pdf"
bool is_pdf = s_ends_with(filename, ext)
```

`s_split` always returns at least one element — an input with no delimiter occurrences returns a single-element list containing the original string.

---

## Conversion Notes

| Deor | Rust |
|---|---|
| `a + b` (both idents) | `a + &b` |
| `"lit" + b` | `"lit".to_string() + &b` |
| `s_contains(str, needle)` | `str.contains(needle.as_str())` |
| `s_starts_with(str, prefix)` | `str.starts_with(prefix.as_str())` |
| `s_ends_with(str, suffix)` | `str.ends_with(suffix.as_str())` |
| `s_trim(str)` | `str.trim().to_string()` |
| `s_to_upper(str)` | `str.to_uppercase()` |
| `s_to_lower(str)` | `str.to_lowercase()` |
| `s_split(str, delimiter)` | `str.split(delimiter.as_str()).map(\|s\| s.to_string()).collect()` |
| `s_join(parts)` | `parts.join("")` |
| `s_join_with(parts, sep)` | `parts.join(sep.as_str())` |
| `s_trim_start(str)` | `str.trim_start().to_string()` |
| `s_trim_end(str)` | `str.trim_end().to_string()` |
| `s_replace(str, from, dest)` | `str.replace(from.as_str(), dest.as_str())` |
| `s_index_of(str, needle)` | `str.find(needle.as_str()).map(\|i\| i as i64).unwrap_or(-1)` |
| `s_char_at(str, idx)` | `str.chars().nth(idx as usize).map(\|c\| c.to_string()).unwrap_or_default()` |
| `s_substring(str, start, end)` | `str.chars().skip(start).take(end - start).collect()` |
| `s_repeat(str, count)` | `str.repeat(count as usize)` |
