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

