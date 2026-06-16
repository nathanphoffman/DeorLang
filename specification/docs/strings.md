# Strings

String operations are built into the language — no import required. All string functions accept literals directly since they are built-ins. See [Built-ins](builtins.md#string-operations) for the full function table.

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
string clean = trim("  Hello, World!  ")

string lower = to_lower(clean)
bool found = contains(lower, "world")

shape nameList = list of string
nameList parts = split("apple,banana,cherry", ",")
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
bool is_abs = starts_with("/api/users", "/")

bool is_pdf = ends_with("report.pdf", ".pdf")
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
| `a + b` | `format!("{}{}", a, b)` |
| `contains(str, needle)` | `str.contains(needle.as_str())` |
| `starts_with(str, prefix)` | `str.starts_with(prefix.as_str())` |
| `ends_with(str, suffix)` | `str.ends_with(suffix.as_str())` |
| `trim(str)` | `str.trim().to_string()` |
| `to_upper(str)` | `str.to_uppercase()` |
| `to_lower(str)` | `str.to_lowercase()` |
| `split(str, delimiter)` | `str.split(delimiter.as_str()).map(\|s\| s.to_string()).collect()` |
