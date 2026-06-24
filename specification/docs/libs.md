# Standard Library

The `lib/` directory contains importable Deor files that ship with the transpiler. Each file is a normal `.deor` file — import it like any other file and its functions become available. No package manager, no crates required (except where noted).

```
import "lib/string.deor"
import "lib/math.deor"
```

Three of the library files are **parameterized** — they use `where T = Type` to produce a concrete, type-safe version of the module. See [Parameterized Imports](#parameterized-imports) below.

---

## `lib/string.deor`

String utilities beyond the built-in `+` concatenation.

| Function | Signature | Description |
|---|---|---|
| `s_trim` | `string → string` | Strip leading and trailing whitespace |
| `s_trim_start` | `string → string` | Strip leading whitespace only |
| `s_trim_end` | `string → string` | Strip trailing whitespace only |
| `s_to_upper` | `string → string` | Uppercase |
| `s_to_lower` | `string → string` | Lowercase |
| `s_contains` | `string, string → bool` | True if the string contains the needle |
| `s_starts_with` | `string, string → bool` | True if the string starts with the prefix |
| `s_ends_with` | `string, string → bool` | True if the string ends with the suffix |
| `s_index_of` | `string, string → int` | Position of needle, or `-1` if not found |
| `s_replace` | `string, string, string → string` | Replace all occurrences of `from` with `to` |
| `s_substring` | `string, int, int → string` | Characters from `start` (inclusive) to `end` (exclusive) |
| `s_char_at` | `string, int → string` | Single character at index as a string |
| `s_repeat` | `string, int → string` | Repeat the string `n` times |
| `s_split` | `string, string → stringList` | Split on delimiter, returns a `stringList` |
| `s_join` | `stringList → string` | Join a list of strings with no separator |
| `s_join_with` | `stringList, string → string` | Join a list of strings with a separator |

```
import "lib/string.deor"

string sentence = "  hello world  "
string trimmed = s_trim(sentence)
bool found = s_contains(trimmed, "world")
stringList words = s_split(trimmed, " ")
string upper = s_to_upper(trimmed)
string replaced = s_replace(trimmed, "world", "Deor")
int pos = s_index_of(trimmed, "hello")
```

---

## `lib/math.deor`

Integer and float math operations.

| Function | Signature | Description |
|---|---|---|
| `m_abs` | `int → int` | Absolute value |
| `m_sign` | `int → int` | −1, 0, or 1 |
| `m_min` | `int, int → int` | Smaller of two |
| `m_max` | `int, int → int` | Larger of two |
| `m_clamp` | `int, int, int → int` | Clamp value between lo and hi |
| `m_pow` | `int, int → int` | Integer exponentiation |
| `m_absf` | `float → float` | Absolute value |
| `m_minf` | `float, float → float` | Smaller of two |
| `m_maxf` | `float, float → float` | Larger of two |
| `m_clampf` | `float, float, float → float` | Clamp value between lo and hi |
| `m_powf` | `float, float → float` | Float exponentiation |
| `m_sqrt` | `float → float` | Square root |
| `m_floor` | `float → int` | Round down |
| `m_ceil` | `float → int` | Round up |
| `m_round` | `float → int` | Round to nearest |
| `m_log` | `float → float` | Natural log |
| `m_log2` | `float → float` | Log base 2 |
| `m_log10` | `float → float` | Log base 10 |

```
import "lib/math.deor"

int clamped = m_clamp(val, low, high)
float root = m_sqrt(area)
```

---

## `lib/random.deor`

Random number generation with no external crates. Seeded automatically from the system clock.

| Function | Signature | Description |
|---|---|---|
| `m_rand_int` | `int, int → int` | Random integer in the inclusive range `[min, max]` |
| `m_rand_float` | `→ float` | Random float in `[0.0, 1.0)` |
| `m_rand_bool` | `→ bool` | Random boolean |

```
import "lib/random.deor"

int roll = m_rand_int(1, 6)
float chance = m_rand_float()
```

---

## `lib/convert.deor`

Type conversions between Deor primitives.

| Function | Signature | Description |
|---|---|---|
| `c_float_to_int` | `float → int` | Truncate float to int |
| `c_int_to_float` | `int → float` | Widen int to float |
| `c_int_to_string` | `int → string` | Decimal string representation |
| `c_float_to_string` | `float → string` | String representation |
| `c_bool_to_string` | `bool → string` | `"true"` or `"false"` |
| `c_string_to_int` | `string → int` | Parse integer, returns `0` on failure |
| `c_string_to_float` | `string → float` | Parse float, returns `0.0` on failure |
| `c_string_to_bool` | `string → bool` | `"true"` → `true`, anything else → `false` |

```
import "lib/convert.deor"

float precise = c_int_to_float(count)
string label = c_int_to_string(score)
```

---

## Parameterized Imports

Three library files are **parameterized** — they define generic data structures and operations that are specialized for a concrete type at import time using `where T = Type`. The transpiler performs a textual substitution of `T` throughout the file before merging it into the token stream.

All Deor primitive types work as the concrete type: `int`, `float`, `string`, and `bool`.

```
import "lib/list.deor" where T = int
import "lib/list.deor" where T = float
import "lib/list.deor" where T = Report
```

### Naming rules after substitution

| Pattern in source | Example with `T = Report` | Example with `T = float` |
|---|---|---|
| `T` (bare type) | `Report` | `float` / `f64` in rust blocks |
| `TSender` (PascalCase prefix) | `ReportSender` | `FloatSender` |
| `tSenderFunc` (lowercase t prefix) | `reportSenderFunc` | `floatSenderFunc` |
| `t_T_spawn` (snake `_T_` segment) | `t_report_spawn` | `t_float_spawn` |

---

## `lib/list.deor`

Parameterized list operations for any element type. Import once per type you need.

```
import "lib/list.deor" where T = int
```

After substitution with `T = int`, the shape is named `lIntList` and functions are prefixed `l_int_`:

| Function (before substitution) | Signature | Description |
|---|---|---|
| `l_T_first` | `lTList → T` | First element (panics if empty) |
| `l_T_last` | `lTList → T` | Last element (panics if empty) |
| `l_T_is_empty` | `lTList → bool` | True if the list has no elements |
| `l_T_reverse` | `lTList → lTList` | Reversed copy |
| `l_T_slice` | `lTList, int, int → lTList` | Sub-list from `start` (inclusive) to `end` (exclusive) |
| `l_T_concat` | `lTList, lTList → lTList` | Concatenate two lists |
| `l_T_sort` | `lTList → lTList` | Sorted copy (requires element type to implement `Ord`) |
| `l_T_sum` | `lTList → T` | Sum of all elements |
| `l_T_min` | `lTList → T` | Minimum element |
| `l_T_max` | `lTList → T` | Maximum element |
| `l_T_join` | `lTList, string → string` | Join elements with a separator string |
| `l_T_contains` | `lTList, T → bool` | True if the list contains the item |
| `l_T_index_of` | `lTList, T → int` | Index of item, or `-1` if not found |
| `l_T_unique` | `lTList → lTList` | Copy with duplicates removed, preserving order |
| `l_T_take` | `lTList, int → lTList` | First `n` elements |
| `l_T_drop` | `lTList, int → lTList` | All elements after the first `n` |
| `l_T_push` | `lTList, T → lTList` | New list with item appended to the end |
| `l_T_pop` | `lTList → lTList` | New list with the last element removed |

```
import "lib/list.deor" where T = int

lIntList scores = [10, 20, 30]
int total = l_int_sum(scores)
int best = l_int_max(scores)
lIntList top = l_int_slice(scores, 0, 2)
bool has_ten = l_int_contains(scores, 10)
lIntList first_two = l_int_take(scores, 2)
lIntList grown = l_int_push(scores, 40)
lIntList shrunk = l_int_pop(scores)
```

`l_T_sort`, `l_T_sum`, `l_T_min`, `l_T_max`, `l_T_contains`, `l_T_index_of`, and `l_T_unique` require the element type to implement `Ord` / `Hash` / `Clone` in the generated Rust. They work naturally for `int`, `float`, and `string`. For custom structs, use a `rust` block instead.

---

## `lib/map.deor`

String-to-string hash map backed by `Arc<Mutex<HashMap>>`. The `StringMap` is a `raw` type — pass it around freely, mutations are in-place through the shared reference.

| Function | Signature | Description |
|---|---|---|
| `mp_make` | `→ StringMap` | Create an empty map |
| `mp_set` | `StringMap, string, string → StringMap` | Insert or update a key |
| `mp_get` | `StringMap, string → string` | Value for key, or `""` if absent |
| `mp_has` | `StringMap, string → bool` | True if the key exists |
| `mp_remove` | `StringMap, string → StringMap` | Remove a key |
| `mp_size` | `StringMap → int` | Number of entries |
| `mp_keys` | `StringMap → stringList` | All keys |
| `mp_values` | `StringMap → stringList` | All values |

```
import "lib/map.deor"

StringMap config = mp_make()
config = mp_set(config, "host", "localhost")
config = mp_set(config, "port", "8080")
bool has_host = mp_has(config, "host")
string host = mp_get(config, "host")
int count = mp_size(config)
```

---

## `lib/file.deor`

File system operations. All paths are strings. Functions that can fail return `bool` indicating success.

| Function | Signature | Description |
|---|---|---|
| `f_read` | `string → string` | Read entire file as a string, or `""` on failure |
| `f_write` | `string, string → bool` | Write content to file (creates or overwrites), returns success |
| `f_append` | `string, string → bool` | Append content to file (creates if absent), returns success |
| `f_exists` | `string → bool` | True if the path exists |
| `f_lines` | `string → stringList` | Read file as a list of lines |
| `f_delete` | `string → bool` | Delete a file, returns success |

```
import "lib/file.deor"

bool ok = f_write("log.txt", "starting up\n")
bool appended = f_append("log.txt", "step two\n")
string contents = f_read("log.txt")
stringList lines = f_lines("log.txt")
bool gone = f_delete("log.txt")
```

---

## `lib/time.deor`

Timestamps and elapsed time. `ti_now` returns Unix seconds as `int` (valid until 2038); use `ti_now_ms` for millisecond precision as `float`.

| Function | Signature | Description |
|---|---|---|
| `ti_now` | `→ int` | Current Unix timestamp in whole seconds |
| `ti_now_ms` | `→ float` | Current Unix timestamp in milliseconds |
| `ti_elapsed` | `int → int` | Seconds elapsed since the given `ti_now` snapshot |
| `ti_elapsed_ms` | `float → float` | Milliseconds elapsed since the given `ti_now_ms` snapshot |

```
import "lib/time.deor"

float start = ti_now_ms()
# ... do work ...
float ms = ti_elapsed_ms(start)
print(c_float_to_string(ms))
```

---

## `lib/tasks.deor`

Pool-bounded parallel map over a typed list. Imports `lib/taskpool.deor` automatically.

```
import "lib/tasks.deor" where T = Score
```

After substitution with `T = Score`:

| Name | Kind | Description |
|---|---|---|
| `scoreList` | shape | `list of Score` |
| `scoreTransformFunc` | shape | `func of Score to Score` |

| Function (before substitution) | Signature | Description |
|---|---|---|
| `t_T_run_all` | `TaskPool, tList, tTransformFunc → tList` | Map a list of T through a worker in parallel, return all results |

Pass a list of T and a worker function `T → T`; get back a list of results. All items are dispatched to the pool concurrently; the call blocks until every result is collected. Results are returned in completion order, not input order.

The pool caps concurrency automatically — dispatching 10 000 items still only runs `available_parallelism()` threads at once.

### Example

```
import "lib/tasks.deor" where T = Score

struct Score
    string label
    int points

fn Score make_score(string label, int points)
    Score built = move (label, points)
    return built

fn Score apply_bonus(Score score)
    (label, points) in score
    points = points * 2
    Score result = move (label, points)
    return result

fn void main()
    TaskPool pool = t_pool_make()

    string lbl_aaa = "accuracy"
    int pts_aaa = 10
    Score aaa = make_score(lbl_aaa, pts_aaa)

    scoreList jobs = empty
    jobs at end = aaa

    scoreTransformFunc worker = apply_bonus
    scoreList results = t_score_run_all(pool, jobs, worker)

    int count = len(results)
    for idx in range(count)
        Score res = results at idx
        (label, points) in res
        print(label)
        print(points)
```

### Primitive types

All Deor primitive types work as `T`:

```
import "lib/tasks.deor" where T = float
import "lib/tasks.deor" where T = int
import "lib/tasks.deor" where T = string
```

Primitives map to their Rust equivalents in the generated code (`float` → `f64`, `int` → `i32`, `string` → `String`).

### Importing multiple types

Each `where T = ...` import is independent:

```
import "lib/tasks.deor" where T = Request
import "lib/tasks.deor" where T = Report
```

---

## Writing Custom Wrappers

When you need functionality not in the standard library, wrap a Rust function in a small Deor function using a `rust` block. The Deor function owns the signature and naming; the `rust` block handles the implementation. Keep blocks small — one thing per block.

See [Rust Interop](interop.md) for full `rust` block rules.

### Naming Convention

Follow the same prefix convention as the standard library to keep the global namespace readable:

Standard library prefixes (reserved):

| Prefix | Module |
|---|---|
| `s_` | `lib/string.deor` |
| `m_` | `lib/math.deor`, `lib/random.deor` |
| `l_` | `lib/list.deor` |
| `c_` | `lib/convert.deor` |
| `mp_` | `lib/map.deor` |
| `f_` | `lib/file.deor` |
| `ti_` | `lib/time.deor` |
| `t_` | `lib/tasks.deor`, `lib/taskpool.deor` |

For custom wrappers, use a distinct prefix to avoid collisions:

| Prefix | Use |
|---|---|
| `cx_` | Cargo crate wrapper (`cx_json_parse`) |
| `ex_` | Personal/third-party Deor lib (`ex_do_cool_thing`) |

### I/O

```
fn string read_line()
    rust
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim_end_matches('\n').to_string()
```

### Parsing

Wrap parse results in a validator type so the caller can check success:

```
type ParsedInt(int val)
    true

fn ParsedInt parse_int(string src)
    ParsedInt result
    rust
        if let Ok(num) = src.parse::<i32>() {
            result = Some(ParsedInt(num));
        }
    return result
```

```
ParsedInt parsed = parse_int(user_input)
if parsed valid
    int val = (avow parsed)
    print(val)
```

The same pattern works for `ParsedFloat` — swap `i32` for `f64`.

### Cargo Crates

For anything requiring an external crate, add it to `Cargo.toml` manually and wrap it the same way:

```
fn int cx_rand_int(int min, int max)
    rust
        use rand::Rng;
        rand::thread_rng().gen_range(min..=max)
```
