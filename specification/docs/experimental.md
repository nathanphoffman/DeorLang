# Experimental
These features should not be used

---
### Give Up
As of June 19th 2026, another problem has come up: ```as``` unintentionally is avoided by the cloning process, so as takes ownership of whatever is assigned to it.  This is actually an unintended good practice that I am going to keep.  The question here is do we change the wording of give up to ```as```.  I like this for brevity and it keeps the number of reserved keywords at a minimum which continues to grow.

```
call_fn(as arg1, as arg2)
```

The intention of ```giveup``` is to expose Rust ownership to Deor. The reason is that Deor clones every value for developer ease, but it can cost serious performance. Normal rust ownership transfers the variable into the block it is sent to (such as passing a variable into a function) that has the side-effect of losing the variable below that function. This is the default in Rust, but not ideal for Deor's simplicity.  

```giveup``` allows the ownership of rust to be restored
```deor
listBigStruct big_list = a_value
do_something(giveup big_list)

# allows the collection/item to not be constantly cloned
for giveup (item in collection) 

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

Any function in the using list that has no argument provided, has the object passed to it as its first argument.  Each function then chains on the output of the above function so that any non-argument functions form a pipe chain.  If something provides an argument, it is ignored.  Additionally, each time a () function is called on the pipe chain, the values of the object are deconstructed automatically (hence why last_name works below)

This entire procedure is quite expensive and is why it is still experimental and being considered to be removed.

```

struct Employee
    string first_name
    string last_name
    int: processed_number

shape listEmployee employee_list = list of Employee

using employee_list
    set_first_name()
    get_record_by_lastname(last_name)
    set_last_name()
    

```
