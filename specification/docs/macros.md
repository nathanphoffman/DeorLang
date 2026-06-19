
---
### Macros
Macros allow you to insert raw code into lines as part of the transpile pipeline (meaning they act as if you had copy and pasted the raw code within the macro to lines that call it)

They are extra useful in deor because it eliminates an enormous amount of cloning that can happen by avoiding seperate functions, yet it still allows you to organize the code. This can become neccessary for performance in large loops where passing off to several functions for readability is not ideal (due to deors cloning process). It also is much more human readable than rust macros and more flexible as it doesn't have the same level of safety and pickyness.

Macros automatically wrap themselves in bare blocks {} a rust convention but this is hidden to the user, this means that variables will not pollute anything outside of the macro (important as the macro code is literally copy and pasted everywhere you have the macro_run command).

Macros can be defined and imported in files like all other root level declarations in deor, which allows for ease of organization.

The good news with macros is that while they are transpiled unsafetly (for the purpose of ease of use), the final output is still built by rust which means that dangling variable names, and incompatible types will be found by the rust compiler.

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