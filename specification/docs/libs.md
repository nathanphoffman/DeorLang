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

Typed channels over a shared thread pool. Combines `lib/taskpool.deor` (which it imports automatically) with a parameterized channel pair so values of type `T` can be sent between threads safely.

```
import "lib/tasks.deor" where T = Report
```

After substitution with `T = Report`:

| Name | Kind | Description |
|---|---|---|
| `ReportSender` | type alias | The sender half of a `Report` channel |
| `ReportReceiver` | type alias | The receiver half (mutex-wrapped) |
| `ReportChanPair` | struct | Both halves together |
| `tReportList` | shape | `list of ReportReceiver` |
| `reportSenderFunc` | shape | `func of ReportSender to void` |
| `task_runner` | macro | Sets up the pool + channel in one step |

| Function (before substitution) | Signature | Description |
|---|---|---|
| `t_T_chan_make` | `→ TChanPair` | Create a paired sender/receiver channel |
| `t_T_chan_send` | `TSender, T → void` | Send a value through the channel |
| `t_T_chan_recv` | `TReceiver → T` | Block until a value arrives |
| `t_T_spawn` | `TaskPool, TSender, tSenderFunc → void` | Dispatch a task onto the pool |

### `task_runner` macro

The `task_runner` macro sets up a `TaskPool` and a typed channel in one step. After it runs, three bindings are in scope:

| Binding | Type | Description |
|---|---|---|
| `pool` | `TaskPool` | The shared thread pool |
| `snd` | `TSender` | Channel sender — clone and pass to each spawned task |
| `rcv` | `TReceiver` | Channel receiver — call `t_T_chan_recv` to collect results |

### Example

```
import "lib/tasks.deor" where T = Report

shape reportFunc = func of ReportSender to void

fn void process(ReportSender snd)
    Report result = (...)
    t_report_chan_send(snd, result)

fn void main()
    macro_run task_runner   # injects: pool, snd, rcv

    int job_count = 5
    for idx in range(job_count)
        reportFunc worker = process
        t_report_spawn(pool, snd, worker)

    for idx in range(job_count)
        Report result = t_report_chan_recv(rcv)
        print(result)
```

### Primitive types

All Deor primitive types work as `T`:

```
import "lib/tasks.deor" where T = float
import "lib/tasks.deor" where T = int
import "lib/tasks.deor" where T = string
```

Primitives are automatically mapped to their Rust equivalents inside the generated `rust` blocks (`float` → `f64`, `int` → `i32`, `string` → `String`), so the generated channel type is always valid Rust.

### Importing multiple types

Each `where T = ...` import is independent. If you need channels for two types, import twice with different type names:

```
import "lib/tasks.deor" where T = Report
import "lib/tasks.deor" where T = Error
```

This produces separate `ReportSender`/`ReportReceiver` and `ErrorSender`/`ErrorReceiver` types with no name collisions.
