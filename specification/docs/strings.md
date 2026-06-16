# Strings

String utility functions live in `lib/string.deor`. Import what you need:

```
(s_trim, s_split, s_contains) in "lib/string"
```

All functions in `lib/string.deor` follow the `s_` prefix convention. They are regular user-defined functions, so arguments must be named variables — literals are not valid arguments. See [Enforced Practices](enforced_practices.md#named-arguments--user-defined-functions-only).

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
let greeting: String = format!("{}{}", "hello ", name);
let line: String = format!("{}{}", prefix, format!("{}{}", content, "\n"));
let full: String = format!("{}{}", first, format!("{} {}", " ", last));
```

Chains of `+` are evaluated left to right. Mixed string/int `+` in the same expression is a transpiler error — use a `rust` block if you need to format an integer into a string.

```
# Transpiler error — mixed types in one + chain
string bad = "count: " + count    # count is int — not allowed

# Correct — convert first with a rust block
fn string int_to_str(int n)
    rust
        n.to_string()

string msg = "count: " + int_to_str(count)
```

---

## Examples

```
(s_trim, s_to_lower, s_contains, s_split, s_starts_with, s_ends_with, stringList) in "lib/string"

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
| `a + b` | `format!("{}{}", a, b)` |
| `s_contains(str, needle)` | `str.contains(needle.as_str())` |
| `s_starts_with(str, prefix)` | `str.starts_with(prefix.as_str())` |
| `s_ends_with(str, suffix)` | `str.ends_with(suffix.as_str())` |
| `s_trim(str)` | `str.trim().to_string()` |
| `s_to_upper(str)` | `str.to_uppercase()` |
| `s_to_lower(str)` | `str.to_lowercase()` |
| `s_split(str, delimiter)` | `str.split(delimiter.as_str()).map(\|s\| s.to_string()).collect()` |
