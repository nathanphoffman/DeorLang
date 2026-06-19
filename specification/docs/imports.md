# Imports

Imports use the `import` keyword followed by a path

```
import "models/customer.deor"
```

All imports are pulled in everywhere, there is no way to scope down specific imported items, or privatize them. So naming functions, enums, structs, etc. well is important to preventing collisions.

If you import the same file twice, it just ignores the later imports and imports it once.

There is no way to make any exposed root level declaration private like structs, functions, etc. so use good naming practices and descriptive naming.  This also enforces good naming conventions and smaller project sizes which is ideal Deor logic.

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
