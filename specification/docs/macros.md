
---
### Macros
Macros allow you to insert raw code into lines as part of the transpile pipeline (meaning they act as if you had copy and pasted the raw code within the macro to lines that call it)

They are useful in deor because it eliminates an enormous amount of deor's cloning that can happen by avoiding seperate functions which clone and avoiding the move keyword which steals, yet it still allows you to organize the code. This can become neccessary for performance in large loops where passing off to several functions for readability is not ideal. It also is much more human readable than rust macros and more flexible as it doesn't have to declare parameters or have the same level of safety (yet still benefits from the rust compile-time checks).

Importantly, because macros are literally copy and pasted, their variables will pollute the scope you are executing them in, however, you can always use the ```block``` keyword within your macro to avoid this.

Macros are top-level declarations — they are available in any file that imports the file they are defined in, exactly like functions and structs. No special import syntax needed.

```
macro say_hello
    print(hello)

hello as "Hi There"
macro_run say_hello

hello as "Hi There Again"
macro_run say_hello

# output is "High There"
# and "Hi There Again"

```