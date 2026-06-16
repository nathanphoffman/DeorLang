# Syntax.md Changes
- What is deps in syntax.md ?

# Functions.md Changes
- Suggest break in this block "Void functions cannot contain return statements. Early exit is a transpiler error — all conditional paths must be expressed with if/else block structure:"

# variables.md
- Why would this "area as 9 # transpiler error — int or Squarefeet?" be a transpiler error?

# types.md
- I feel like bytes should be 'raw' the rust pass through type, the idea being anyone using bytes or dicts should just use rust code to handle it.
- Validator types make it sound almost like a conditional body on a type is optional but suggested, it is not optional, it is required
- this throws a transpiler error now for literal values like this: Squarefeet bad = -1     # None — predicate fails silently
- I think we have switch to is not for this "if not area"
- lists can also be initialized to empty "Initializing to Empty" this is equivalent to [], [] is not allowed
- structs can contain other structs, I might have contradicted myself on this in the past.  Also this is wrong "All struct fields are always public. There are no visibility modifiers on fields. Structs carry data only — no methods, no encapsulation. When a struct is importable, all its fields are accessible via destructuring. If you want to restrict access to a struct’s internals, mark the struct itself private rather than its fields."  struct fields have no accessibility, instead top level declarations structs, shapes, functions, types have private/public with public assumed which exists for the purpose of importing.  private prevents it from being imported.

shapes.md
- this is wrong roomList result = [], see comment on arrays it is roomList result = empty
- 

