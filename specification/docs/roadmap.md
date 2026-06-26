AI DONT TOUCH THIS DOCUMENT, THIS IS FOR NATE ONLY

# Road Map
Upcoming features, mostly an internal markdown file used by the creator Nathan Hoffman.

##
---
macro_blocks are to remain just open/close no boundary splitting mid-function, no auto-rust insertion, etc.  It limits their power but makes things much more maintainable anything beyond like a simple timer maro block compounds into massive scale issues.


---
## Test Latest Fixes

  2. Func shapes as struct fields — prescan_check_struct_fields validates field names but not field types; a filterFunc shape field would pass through.
    ^^ validation being added -- should be fixed? validate

  3. raw rules — "raw must come from a rust block", "raw can't be used in a Deor expression", "raw can't be a struct field" — none of these are validated.
    ^^ sounds good, add validation -- should be fixed, validate

  4. Type validator parameter shadowing its type — type Roll(int Roll) — check_fn_declaration catches this for functions but validator type declarations aren't checked the same way.
    ^^ add validation, fix -- should be fixed, validate

 ---
## Naming Examples for Libs

- Give better examples of naming for cx and ex in the docs around naming custom libs
-- x here represents their lib first-letter like s in string
-- so a nate external 3rd party lib would be en,  e for external, n for nates_lib.deor
-- c i honestly forget the intention of but similarly we should give useful examples

---

## Audit Documentation
*Large Lift* | *Critical Priority*

-- prolly about 60% done as of June 21st

A lot of the documentation needs refreshed it is based on an old AI structure I build in pair-theorizing with AI about the Deor programming language, as I have been implementing it with Claude we have started going off the beaten track and a lot of the docs are either out of date or in great need of adjusting.

In need of extra auditing:
- Conditionals
- Enforced Practices


## Add Better Onboarding Document
*Large Lift* | *Low Priority*
It would be good to start a doc that takes users from 0 to completion of at least a small sized project


