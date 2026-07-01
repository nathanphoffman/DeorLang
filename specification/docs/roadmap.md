AI DONT TOUCH THIS DOCUMENT, THIS IS FOR NATE ONLY

# Road Map
Upcoming features, mostly an internal markdown file used by the creator Nathan Hoffman.

# Add to best practices:
- Spacing can be used between functions (double return)  -- no function should be less than 2 extra spaces away (3 returns) from each other or other blocks.  All other blocks are 1 extra space

# More types long and double precision?
- Consider adding long and double

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
---
- imports: we may want to be more explicit about how order actually matters with examples, I have seen for example that incorrect import ordering does cause error messages.
libs: make sure these are up to date with the current /libs folder.  I think lib/list.deor does a poor job explaining the substitution, it can be anything not just T (confirm this is the case), and also the example doesn't look like it is properly subbing out the Ts for everything?  maybe we should hgave a Function (after substitution) column.  All libs should also lead with a single letter not mp or ti.  Choose a single letter that does not conflict with existing lib letters.
---
- loops: I don't love this example 

found as false
for item in items
    matching in item
    if matching
        found = true
        break

What is matching in item?  first off if it is a prop it should be (matching) in item (i think that is required?)  secondly what the heck is matching supposed to be in this example?

- macros: if there are rust blocks here, label clearly which ones are Deor and which ones are Rust
- move: if there are rust blocks here, label clearly which ones are deor and which ones are rust
- move: "Standard Deor (no move) is always correct; "  But if thats the case why ever use move?
- operators: strings and libs link is broken, types-forced wrab link doesn't seem to exist
- index: remove roadmap link from the index


--- Should be done the stuff above


----
Enum docs need updated to reflect the new value format

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


