AI DONT TOUCH THIS DOCUMENT, THIS IS FOR NATE ONLY

# Road Map
Upcoming features, mostly an internal markdown file used by the creator Nathan Hoffman.

# We need a way to read input from console
thinking of (first,second) in input()

# Should we go back to strict ordering?
- I am feeling inspired by fortran, like all variable definitions at the top of the block, unless they are moved
- This is complicated though by macros, especially if you want to be clear about what variables are being passed in
- On second thought maybe not a good idea but still worth consideration

## Doc Corrections
- best-practices: lets add something about avoiding deep nesting, and keeping functions small and using functions if not in a loop, and if in a loop using macros (to keep performance reasonable).  Files should also be kept to a reasonable length.
- built-ins: Range built in needs the whole in (list) stuff
- collections: be more specific about the rust equivalent blocks, it looks like rust and deor are just mashed together.
- conditionals: In Conditionals why do we have two blocks that look the same?
- destructuring: the shadowing example uses world in tab as an example, I don't know what world is, maybe (name) in employee ?
- enforced_practices: claims that we have tuple returns, do we?  I thought this might have been removed
- enums: we need to differentiate deor from rust again, worth checking the docs to make sure there are no other violations
- examples: I think we should delete this section for now
- experimental: delete section
- functions: again clearly tag deor and rust sections.  Also functions seems to agree there are no tuple returns, so check to see what is true about tuples, as other code says we do support tuples.
- functions: the shapes-func shapes is not defined
- immutability: clearly label deor vs rust blocks



Libs:
- Give better examples of naming for cx and ex in the docs around naming custom libs
-- x here represents their lib first-letter like s in string
-- so a nate external 3rd party lib would be en,  e for external, n for nates_lib.deor
-- c i honestly forget the intention of but similarly we should give useful examples

## Confirm macros allow for macros within macros

## Talk about adding values to enums
I am thinking of something like

enum int Color
  Red = 1
  Blue = 2
  Red = 3

or

enum string Color
  Red = "Red"
  Blue = "Blue"
  Yellow = "Yellow"

enums without types should just be how enums work today

So someone can do

(Red, Blue) in Color

value as "Red"
value2 as Red <-- this is not allowed and should throw an error

if Red is value
  print("is red")

enum Color
  Red
  Blue
  Yellow



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


