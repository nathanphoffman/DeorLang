# Macros

A macro is a named block of code that is inlined at every `macro_run` call site — equivalent to copy-pasting the macro body at that point in the source.

Macros are useful because they avoid the clone overhead of function calls while still letting you organize repetitive logic. This matters most inside tight loops where calling helper functions repeatedly introduces unnecessary cloning. Unlike Rust macros, Deor macros need no parameter declarations and require no special syntax — they benefit from Rust's compile-time checks because the inlined code goes through the same transpile and compile pipeline.

Because macro bodies are inlined, any variables they declare pollute the caller's scope. Use the `block` keyword inside a macro body to contain variables that should not escape.

Macros can be declared at the top level or inside a function body. A top-level macro is available to any file that imports the file it is defined in — no special import syntax needed. A macro declared inside a function body is locally scoped: it exists only within that block and is not visible outside it.

You can call other macros with `macro_run` from inside a macro body — nesting calls is fully supported. However, defining a `macro` inside another macro body is a compile error. Define all macros at the appropriate scope and call them with `macro_run`.

```
macro say_hello
    print(hello)

fn void greet()
    hello as "Hi There"
    macro_run say_hello

    hello as "Hi There Again"
    macro_run say_hello

# output is "Hi There"
# and "Hi There Again"
```

---

## `block` Inside Macros

Because a macro body is copy-pasted at the call site, any variables it declares become part of the caller's scope. If the macro is called more than once, or if its internal variable names conflict with the caller's names, this causes a compile error.

Use `block` inside the macro body to create an isolated scope. Variables declared inside `block` do not escape:

Deor:
```
macro compute_area
    block
        length as 10
        width as 5
        area as length * width
        print(area)

fn void run()
    macro_run compute_area
    macro_run compute_area    # safe — block variables do not leak between calls
```

Rust:
```rust
{
    let length = 10;
    let width = 5;
    let area = length * width;
    println!("{}", area);
}
{
    let length = 10;
    let width = 5;
    let area = length * width;
    println!("{}", area);
}
```

Without `block`, the second `macro_run` would fail to compile because `length`, `width`, and `area` would already be declared in scope.

If the macro only reads variables from the caller's scope without declaring any of its own, `block` is not needed.

---

## `macro_block` — Wrap a Body With Open and Close Code

`macro_block` is a pre-processor construct that runs before tokenization. It lets you define reusable open/close Deor snippets that wrap an indented body at each call site.

Three keywords form the system:

- `macro_block_open name` — Deor code injected before the body
- `macro_block_close name` — Deor code injected after the body
- `macro_block name` — the call site; its indented body is sandwiched between open and close

The body is dedented one level and handed to the tokenizer normally — you can use any Deor syntax inside, including nested `macro_block` calls, `macro_run` calls, and `rust` blocks.

**Definition:**

```
macro_block_open timer
    int _timer_start = now_ms()

macro_block_close timer
    int _timer_elapsed = elapsed_ms(_timer_start)
    string _timer_str = n_to_str(_timer_elapsed)
    string _timer_sfx = "ms"
    print(s_join([_timer_label, _timer_str, _timer_sfx]))
```

**Usage:**

```
string _timer_label = "[timer] load: "
macro_block timer
    tokenList raw_tokens = collect_all_tokens_with_all_imports(input_path)
```

After expansion:

```
string _timer_label = "[timer] load: "
int _timer_start = now_ms()
tokenList raw_tokens = collect_all_tokens_with_all_imports(input_path)
int _timer_elapsed = elapsed_ms(_timer_start)
string _timer_str = n_to_str(_timer_elapsed)
string _timer_sfx = "ms"
print(s_join([_timer_label, _timer_str, _timer_sfx]))
```

The close body can freely read variables declared in the open body (`_timer_start`) and variables the caller placed in scope before the call (`_timer_label`).

The body of `macro_block_open` and `macro_block_close` follows standard Deor indentation. If the close needs nested code, write it with the natural extra levels:

```
macro_block_close checked
    if error_count > 0
        print("failed")
```

---

### Rules

- `macro_block` definitions cannot appear inside a `macro_block` body — the preprocessor will error.
- Definitions are picked up from **directly imported files**. Define `macro_block_open` / `macro_block_close` in a library file, import it, and use `macro_block name` anywhere in the importing file. Only immediate imports are scanned — transitive imports are not visible.
- `macro_block` differs from `macro` / `macro_run` in two ways: it operates on raw source text before the lexer runs, and it wraps a variable body of caller-supplied code rather than inlining a fixed snippet.
