# Macros

A macro is a named block of code that is inlined at every `macro_run` call site — equivalent to copy-pasting the macro body at that point in the source.

Macros are useful because they avoid the clone overhead of function calls while still letting you organize repetitive logic. This matters most inside tight loops where calling helper functions repeatedly introduces unnecessary cloning. Unlike Rust macros, Deor macros need no parameter declarations and require no special syntax — they benefit from Rust's compile-time checks because the inlined code goes through the same transpile and compile pipeline.

Because macro bodies are inlined, any variables they declare pollute the caller's scope. Use the `block` keyword inside a macro body to contain variables that should not escape.

Macros are top-level declarations available in any file that imports the file they are defined in — no special import syntax needed.

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