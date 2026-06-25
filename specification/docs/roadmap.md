AI DONT TOUCH THIS DOCUMENT, THIS IS FOR NATE ONLY

# validation gaps flagged by ai
  Gaps worth fixing:
  1. return empty / return none — docs say transpiler error; no check exists. return empty is KW_RETURN KW_EMPTY, easy to catch.
    ^^ validation being added -- should be fixed validate

  2. Func shapes as struct fields — prescan_check_struct_fields validates field names but not field types; a filterFunc shape field would pass through.
    ^^ add validation

  3. raw rules — "raw must come from a rust block", "raw can't be used in a Deor expression", "raw can't be a struct field" — none of these are validated.
    ^^ sounds good, add validation

  4. Type validator parameter shadowing its type — type Roll(int Roll) — check_fn_declaration catches this for functions but validator type declarations aren't checked the same way.
    ^^ add validation, fix

  5. const reassignment — const variables can technically be reassigned; the transpiler infers let vs let mut from usage but doesn't catch const val = reassigned_value as an error.
    ^^ add check if it isnt too hard



# Documentation
 ---

- Give better examples of naming for cx and ex in the docs around naming custom libs
-- x here represents their lib first-letter like s in string
-- so a nate external 3rd party lib would be en,  e for external, n for nates_lib.deor
-- c i honestly forget the intention of but similarly we should give useful examples


# Should const be uppercase?
- We should consider making function consts SCREAMING_SNAKE

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


