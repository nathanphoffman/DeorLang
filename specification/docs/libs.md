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
| `s_to_upper` | `string → string` | Uppercase |
| `s_to_lower` | `string → string` | Lowercase |
| `s_contains` | `string, string → bool` | True if the string contains the needle |
| `s_starts_with` | `string, string → bool` | True if the string starts with the prefix |
| `s_ends_with` | `string, string → bool` | True if the string ends with the suffix |
| `s_split` | `string, string → stringList` | Split on delimiter, returns a `stringList` |

```
import "lib/string.deor"

string sentence = "  hello world  "
string trimmed = s_trim(sentence)
bool found = s_contains(trimmed, "world")
stringList words = s_split(trimmed, " ")
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

```
import "lib/list.deor" where T = int

lIntList scores = [10, 20, 30]
int total = l_int_sum(scores)
int best = l_int_max(scores)
lIntList top = l_int_slice(scores, 0, 2)
```

`l_T_sort`, `l_T_sum`, `l_T_min`, and `l_T_max` require the element type to implement `Ord` / `Copy` in the generated Rust. They work naturally for `int`, `float`, and `string`. For custom structs, use a `rust` block instead.

---

## `lib/tstack.deor`

Parameterized stack (last-in, first-out). Import once per element type.

```
import "lib/tstack.deor" where T = string
```

After substitution with `T = string`, the shape is named `tStringStack` and functions are prefixed `t_string_`:

| Function (before substitution) | Signature | Description |
|---|---|---|
| `t_T_make` | `→ tTStack` | Create an empty stack |
| `t_T_size` | `tTStack → int` | Number of elements |
| `t_T_get` | `tTStack, int → T` | Element at index (zero-based) |

Push and pop use the standard list operations `at end =` and `remove at`:

```
import "lib/tstack.deor" where T = string

tStringStack history = t_string_make()
history at end = "first"
history at end = "second"
int depth = t_string_size(history)
string top = history at (depth - 1)
history remove at (depth - 1)
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
