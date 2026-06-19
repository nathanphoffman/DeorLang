# Imports

Imports use the `import` keyword followed by a path

```
import "models/customer.deor"
```

All imports are pulled in everywhere, there is no way to scope down specific imported items, or privatize them. So naming functions, enums, structs, etc. well is important to preventing collisions.

Although you can put the imports whereever you want (as long as they are at the top of the file), since they are all global anyway, it is easier to create an imports.deor, and just import that one file from main.

main.deor
```
import "imports.deor"
```

imports.deor
```
import "models/customer.deor"
import "utility.deor"
```
