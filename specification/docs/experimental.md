# Experimental
These features should not be used

---
### Give Up
The intention of ```giveup``` is to expose Rust ownership to Deor. The reason is that Deor clones every value for developer ease, but it can cost serious performance. Normal rust ownership transfers the variable into the block it is sent to (such as passing a variable into a function) that has the side-effect of losing the variable below that function. This is the default in Rust, but not ideal for Deor's simplicity.  

```giveup``` allows the ownership of rust to be restored
```deor
listBigStruct big_list = a_value
do_something(giveup big_list)

prev_var as "something"
# takes ownership
new_var as giveup prev_var

# prev_var is not available here and below

# but giveup still needs to be flagged (think of giveup preventing a clone each time it is used)
do_something(giveup new_var)

#new_var is not available here and below

```

---
### Using
The using block was added to aid with the fact that Deor is not OOP-centric, it greatly simplifies composing structs, but at a very high cloning cost. Good for small structs and small lists, ok for large structs in small lists or large lists of small structs, but likely quite bad for both large lists and large structs. You should consider using a rust wrapper see: [Rust Interop](docs/interop.md)

```

struct Employee
    string first_name
    string last_name
    int: processed_number

shape listEmployee = list of Employee

using



```