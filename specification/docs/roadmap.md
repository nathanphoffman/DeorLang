AI DONT TOUCH THIS DOCUMENT, THIS IS FOR NATE ONLY

# Road Map
Upcoming features, mostly an internal markdown file used by the creator Nathan Hoffman.

# Add to best practices:
- Spacing can be used between functions (double return)  -- no function should be less than 2 extra spaces away (3 returns) from each other or other blocks.  All other blocks are 1 extra space

# More types long and double precision?
- Consider adding long and double

## Doc Corrections
- block structure: I am not quite sure what you are saying with brace nexting in conversion notes can you word this more simply?
- syntax: Trailing commas are not encouraged and that should be relegated to best practices, not here -- Deor emphasizes human readablity humans dont put commas after the last item in a list.
- Enum docs need updated to reflect the new value format, enums can now have values like  

I think the syntax is like

enum string Color
  BLUE = "blue"
  RED = "red"

but check the transpiler

----


---
# Footnotes

##
---

---
## Test Latest Fixes

  -- test to make sure args() works properly

  2. Func shapes as struct fields — prescan_check_struct_fields validates field names but not field types; a filterFunc shape field would pass through.
    ^^ validation being added -- should be fixed? validate

  3. raw rules — "raw must come from a rust block", "raw can't be used in a Deor expression", "raw can't be a struct field" — none of these are validated.
    ^^ sounds good, add validation -- should be fixed, validate

  4. Type validator parameter shadowing its type — type Roll(int Roll) — check_fn_declaration catches this for functions but validator type declarations aren't checked the same way.
    ^^ add validation, fix -- should be fixed, validate

 ---


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


