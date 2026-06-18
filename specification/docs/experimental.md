
### Give Up
The intention of ```giveup``` is to expose Rust ownership to Deor. The reason is that Deor clones every value for developer ease, but it costs substantial performance. Rust ownership transfers the variable into the block it is sent to (such as passing a variable into a function) that has the unexpected side-effect of losing the variable below that function.  This is the default in Rust, but not ideal for Deor's simplicity.  

```giveup``` allows the ownership of rust to be restored
```
listBigStruct big_list = a_value
do_something(giveup big_list)


```

---
### Using
The using block was added to aid with the fact that Deor is not OOP-centric.

```

```



---