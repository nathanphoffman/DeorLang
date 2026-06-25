# Macros

A macro is a named block of code that is inlined at every `macro_run` call site — equivalent to copy-pasting the macro body at that point in the source.

Macros are useful because they avoid the clone overhead of function calls while still letting you organize repetitive logic. This matters most inside tight loops where calling helper functions repeatedly introduces unnecessary cloning. Unlike Rust macros, Deor macros need no parameter declarations and require no special syntax — they benefit from Rust's compile-time checks because the inlined code goes through the same transpile and compile pipeline.

Because macro bodies are inlined, any variables they declare pollute the caller's scope. Use the `block` keyword inside a macro body to contain variables that should not escape.

Macros can be declared at the top level or inside a function body. A top-level macro is available to any file that imports the file it is defined in — no special import syntax needed. A macro declared inside a function body is locally scoped: it exists only within that block and is not visible outside it.

Macros cannot be nested inside other macro bodies — the expander is single-pass, so a `macro` definition inside another macro's body will not be recognized and will produce unexpected output.

```
macro say_hello
    print(hello)

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

```
macro compute_area
    block
        length as 10
        width as 5
        area as length * width
        print(area)

macro_run compute_area
macro_run compute_area    # safe — block variables do not leak between calls
```

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

`macro_block` is a pre-processor construct that runs before tokenization. It lets you define reusable open/close code snippets that wrap an indented body at each call site.

Three keywords form the system:

- `macro_block_open name` — code injected before the body
- `macro_block_close name` — code injected after the body
- `macro_block name` — the call site; its indented body is sandwiched between open and close

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

The indented body under `macro_block timer` is the content being wrapped. After expansion this becomes:

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

**Indentation in definitions**

The body of `macro_block_open` and `macro_block_close` follows standard Deor indentation. If the close needs nested code, write it with the natural extra indentation levels:

```
macro_block_close checked
    if error_count > 0
        print("failed")
```

The preprocessor preserves relative indentation when expanding at any call site depth.

**Definitions are picked up from imported files (one level deep).** You can define `macro_block_open` / `macro_block_close` in a separate file, import that file, and then use `macro_block name` anywhere in the importing file.

`macro_block` differs from `macro` / `macro_run` in two ways: it operates on raw source text before the lexer runs, and it wraps a variable body of caller-provided code rather than inlining a fixed snippet.