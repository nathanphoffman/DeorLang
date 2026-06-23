AI DONT TOUCH THIS DOCUMENT, THIS IS FOR NATE ONLY

# New Questions
- Has private been implemented for imports, does it work properly?

- Should we switch mappings to enums?

- Explain ordering of the imports in the docs, suggest a single import file, but explain how it must be ordered and how imports per file could be preferred in some cases as the import order is then handled by the importer (fact check all of this with the transpiler code).  I am thinking to present both options as equally valid but lean towards centralized importer.  Maybe do an upside downside comparison.

- I would like there to be centralized pipelines called from main, most are there but I want codegen to be its own pipeline.  It would be cool if it could be a pipeline for each codegen but each would need to return its full output before continuing to the next not sure that is possible or performant

# New Findings
As should always be implicit ownership, right now the 2nd line below is allowed, it should not be, which makes me wonder how much of this is correct today

a as b <------- allowed, ownership
int a as b <---  not allowed, transpiler validator error
a as move b <--- not allowed, transpiler validator should direct them to drop the move as "as" is already an implicit move
int a = b   <---  allowed, not ownership, a rust clone
int a = move b <--  allowed, ownership


# Bad no longer -> valid comparison only
Bad should be removed, no user can intentionally assign something bad/none or return it. Instead only comparisons are allowed and rather than bad it is valid (the inverse of bad) and the bad keyword is dropped.

If a user wants to define something null they just do this:
NatesInt num 
^ num is considered not valid (none in Rust)

This code should work for testing if/boolean
if num is valid  # this is false
if num is not valid # this is true

explicit assignment should not be allowed:
num = valid # not allowed
num = not valid # not allowed
num = invalid # not even a keyword

The reason for this is if a value is ever known for something, it should never be removed, only changed.  And anytime a validator type is changed, it already runs a valid check (sets it to none in Rust if it doesnt pass)

So to recap, remove bad, add a valid in the same idea (but opposite of bad) but it should only be allowed in comparisons.

Additionally, make it so an empty definition is allowed and is not valid by default.

Implementation wise I think what we should do is still use none under the hood, but when the user does a comparison on valid just replace it with not none in rust, and it should work out I think.  So we are just not not none on not valid checks.  I am ok with the not (!) redundancy if it makes it easier to implement in rust

Other issues
  - for move loop form — experimental.md shows it without parentheses; transpiler requires
  them

# New audit June 21st

  Direct contradictions

  - name as (f1, f2) struct construction — variables.md says it's a transpiler error; codegen
  handles it fine

  - Roll best = empty for validator types — types.md says valid, transpiler actively errors
  with "use 'bad' not 'empty'"
 
  - func, to, end keywords — spec lists them as reserved; transpiler detects them by
  string-matching plain IDENTs (fragile)



## Audit Documentation
*Large Lift* | *Critical Priority*

-- prolly about 60% done as of June 21st

A lot of the documentation needs refreshed it is based on an old AI structure I build in pair-theorizing with AI about the Deor programming language, as I have been implementing it with Claude we have started going off the beaten track and a lot of the docs are either out of date or in great need of adjusting.

In need of extra auditing:
- Conditionals
- Enforced Practices

---
# Smaller items that are in no hurry
---

## Add Better Onboarding Document
*Large Lift* | *Low Priority*
It would be good to start a doc that takes users from 0 to completion of at least a small sized project


## Add additional validation
- Validator: We should not allow any & | ^ < > { } unless it is in string data
  - Some of this has already been done, but not quite all yet validator is being built out
- Validator: Do we check for rust keywords like mut? we should have an exception to catch rust-named keywords that are not in rust blocks
  - No but this is a todo
  
## Crash has 2 Arguments
Crash wraps panic! and so likely provides 2 arguments, the second of which is for string iterpolation which is anti-Deorian, so we might want to force it to 1 argument, string only and force s_join or something similar.

---
# Quick Notes
---

### Macros have been organized but maybe too organized

### Bad has been changed to valid
Bad has been changed to valid, flipping the logic upside down and now not defining a value makes the value not valid.

### For using rust modules this is really easy
There is an example project created that demonstrates json, basically a deor wrapped rust block is all you need you just use cargo to build it and the standard cargo file simple.  Deor will support no native external importing other than copy and paste libs.

### Const abandoned as of June 19th
Const is a difficult feature to add because Rust requires these to be of a-non deor compatible type for string that will conflict with deor strings as compile time strings cant live on the heap.  The lift is not worth the gain.

### List of Validator Type -- Possible Bug?
Is `shape rollList = list of Roll` valid (a list whose element type is a validator type)? It would be `Vec<Option<Roll>>` in Rust. Iterating it gives `Option<Roll>` elements, each truthy/falsy. Useful but semantics need to be explicit — especially around `at end =`, `remove at`, and whether a failed predicate on assignment silently inserts `None` into the list.

So to summarize this issue there are basically two ways to think of a list:

A list that allows Options and trusts them, which means some could be none (unsafe)
Or a list that does not allow a value to append -- which in rust nothing happens but in deor it will check

### Should Macro be more Obvious?
Right now macro just uses ```macro``` and ```macro_run``` I wonder if we should consider something more identifyable to scream that there is code you are missing (and not seeing by running your eye over it)

I am considering ```MACRO``` or ```__MACRO__``` or something similar.  The one counter argument is technically functions also don't stand out any more than macro_run does and being a keyword macro_run might stand-out more.  The counter argument to that though is at least functions are contained with the variables you pass it, whereas macro can do anything to the outer scope, making its impact even higher and less visible.

---
