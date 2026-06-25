AI DONT TOUCH THIS DOCUMENT, THIS IS FOR NATE ONLY


# Fixes
- Fix "for move" Comment
- Clarify any confusing docs for this: name as (f1, f2) struct construction
- Prevent keyword use: func, to, end

# More
- Tell me about 10
- Still confused on this "Roll best = empty for validator types", what is your suggestion?


# Should const be uppercase?
- We should consider making function consts SCREAMING_SNAKE


## Add additional validation

- We should ask claude about any validation gaps, also:
- Validator: We should not allow any & | ^ < > { } unless it is in string data
  


# Documentation
 ---

- Give better examples of naming for cx and ex
-- x here represents their lib first-letter like s in string
-- so a nate external 3rd party lib would be en,  e for external, n for nates_lib.deor
-- c i honestly forget the intention of but similarly we should give useful examples

---

# Getting more info on these

I need to understand more:
17. avow on non-validator variables — docs say transpiler error, but the transpiler just emits .unwrap() on anything.

  10. move ident in multi-arg calls counts as a named variable — do_something(move big_list, other) is valid. Docs don't mention this.


  19. is_snake casing — accepts single lowercase words with no underscores (e.g. myfunc). Both is_camel and is_snake accept a bare lowercase word.

  - ask ai what this means: for move loop form — experimental.md shows it without parentheses; transpiler requires
  them

  - name as (f1, f2) struct construction — variables.md says it's a transpiler error; codegen
  handles it fine

  - Does crash take only 1 string arg? --- I want to make sure it just takes a string error message and that that is documented well and is validated in the transpiler.

- Should we switch mappings to enums in transpiler?

  - Roll best = empty for validator types — types.md says valid, transpiler actively errors
  with "use 'bad' not 'empty'"
 
  - func, to, end keywords — spec lists them as reserved; transpiler detects them by
  string-matching plain IDENTs (fragile)


---


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


