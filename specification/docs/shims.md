# Shims

Copy-paste Deor wrapper functions for common operations. Each one wraps a Rust function using a `rust` block. Drop the ones you need into your project.

---

## Random

Requires the `rand` crate. Declare the dependency once at the top of any file that uses it.

```
deps
    rand = "0.8"

fn int random(int min, int max)
    rust
        use rand::Rng;
        rand::thread_rng().gen_range(min..=max)
```

---

## Math

```
fn int abs_val(int num)
    rust
        if num < 0 { -num } else { num }

fn int floor_f(float val)
    rust
        val.floor() as i32

fn int ceil_f(float val)
    rust
        val.ceil() as i32

fn int round_f(float val)
    rust
        val.round() as i32

fn float sqrt_f(float val)
    rust
        (val as f64).sqrt()

fn int min_i(int left, int right)
    rust
        if left < right { left } else { right }

fn int max_i(int left, int right)
    rust
        if left > right { left } else { right }

fn int pow_i(int base, int exp)
    rust
        (base as i64).pow(exp as u32) as i32
```

---

## Type Conversion

```
fn float to_float(int num)
    rust
        num as f64

fn int to_int(float num)
    rust
        num as i32

fn string int_to_str(int num)
    rust
        num.to_string()

fn string float_to_str(float num)
    rust
        num.to_string()
```

---

## Parsing

These return a validator type so the caller can check success with `if` / `else`.

```
type ParsedInt(int val)
    true

fn ParsedInt parse_int(string str)
    ParsedInt result = empty
    rust
        if let Ok(n) = str.parse::<i32>() {
            result = Some(ParsedInt(n));
        }
    return result

type ParsedFloat(float val)
    true

fn ParsedFloat parse_float(string str)
    ParsedFloat result = empty
    rust
        if let Ok(n) = str.parse::<f64>() {
            result = Some(ParsedFloat(n));
        }
    return result
```

Usage:
```
ParsedInt result = parse_int(user_input)
if result
    int val = (avow result)
    print(val)

int port = parse_int(port_str) else 8080
```

---

## I/O

```
fn string read_line()
    rust
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim_end_matches('\n').to_string()
```

---

## Parallel Work (Threading)

Deor exposes OS threading through a domain-specific shim file. The pattern: define your input and output structs, declare the matching shapes, and copy the `run_parallel` shim. The rust block inside is invariant — it never changes regardless of what your structs contain. Only the Deor declarations at the top change per domain.

Create a file per work domain, e.g. `PaperWork.deor`:

```
struct PaperWorkItem
    int id
    string payload

struct PaperWorkResult
    int id
    string output

shape paperWorkItems = list of PaperWorkItem
shape paperWorkResults = list of PaperWorkResult
shape paperWorkFn = func of PaperWorkItem to PaperWorkResult

fn paperWorkResults run_parallel(paperWorkItems items, paperWorkFn worker)
    rust
        use std::sync::mpsc;
        use std::thread;
        let (tx, rx) = std::sync::mpsc::channel();
        for item in items.iter() {
            let tx = tx.clone();
            let item = item.clone();
            thread::spawn(move || tx.send(worker(item)).unwrap());
        }
        (0..items.len()).map(|_| rx.recv().unwrap()).collect()
```

Import and use it:

```
(PaperWorkItem, PaperWorkResult, paperWorkItems, paperWorkFn, run_parallel) in "./PaperWork"

fn PaperWorkResult process(PaperWorkItem item)
    (id, payload) in item
    string output = ...
    PaperWorkResult result = (id, output)
    return result

fn void main()
    paperWorkItems items = [...]
    paperWorkResults results = run_parallel(items, process)
```

**Adapting to a different payload type** — only the struct field types change. The rust block is identical:

```
struct PaperWorkItem       # change payload field type here
    int id
    int payload            # e.g. int instead of string

struct PaperWorkResult     # change output field type here
    int id
    int output
```

**Notes:**
- Threads are real OS threads. You can spawn more than your core count — the OS schedules them. For CPU-bound work, results come back in completion order, not spawn order. Include `id` in your result struct if ordering matters.
- The rust block uses type inference throughout — no type names to update when you change struct fields.
- No external crates required. `std::sync::mpsc` and `std::thread` are always available.

---

## String Extras

Operations not covered by the built-in string functions.

```
fn string replace_str(string src, string from, string too)
    rust
        src.replace(from.as_str(), too.as_str())

fn int index_of(string src, string needle)
    rust
        src.find(needle.as_str()).map(|i| i as i32).unwrap_or(-1)

fn string repeat_str(string src, int times)
    rust
        src.repeat(times as usize)

fn bool is_empty_str(string src)
    rust
        src.is_empty()
```
