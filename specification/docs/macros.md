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