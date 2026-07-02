AI DONT TOUCH THIS DOCUMENT, THIS IS FOR NATE ONLY

# Road Map
Upcoming features, mostly an internal markdown file used by the creator Nathan Hoffman.

## Working On 

Working On: We should not allow built-ins to be shadowed or used as names, I saw this in the docs, I want the docs updated and transpiler updated
           `crash` is **not** a reserved keyword — 
          +it's a regular builtin function name with its own argument-count validation (see [Builtins](docs/builtins.md#crash)); nothing stops it from being shadowed as a variable or function name, though doing so is not recommende
          +d.  

Working On:
  - prescan_check_duplicate_decls doesn't fire at all — verified with isolated repros for both duplicate struct and duplicate fn names. validation_test.deor's
  expectations were adjusted to reflect this (with a comment explaining why), rather than silently asserting something that isn't true.

  4. validator_types.md's "literal predicate failure" section is wrong about when the check happens. It shows Squarefeet area = -1 as a compile-time error. In
  reality (tb_validator.deor), codegen just emits Squarefeet::new(-1) returning Option<Squarefeet> — the predicate runs at runtime, not transpile time. As written,
  the doc would lead someone to expect a build failure that won't happen.
  
  5. strings.md: "mixed string/int + is a transpiler error" — no such type-checking exists anywhere in the validators; not enforced.


---

## Audit Documentation
*Large Lift* | *Critical Priority*

-- prolly about 85% done as of July 2nd

A lot of the documentation needs refreshed it is based on an old AI structure I build in pair-theorizing with AI about the Deor programming language, as I have been implementing it with Claude we have started going off the beaten track and a lot of the docs are either out of date or in great need of adjusting.