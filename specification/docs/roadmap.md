AI DONT TOUCH THIS DOCUMENT, THIS IS FOR NATE ONLY

# Road Map
Upcoming features, mostly an internal markdown file used by the creator Nathan Hoffman.

# Add to best practices:
- Spacing can be used between functions (double return)  -- no function should be less than 2 extra spaces away (3 returns) from each other or other blocks.  All other blocks are 1 extra space

## New Bug Finds
Bugs found and fixed along the way (all pre-existing, unrelated to earlier work this session):

  confirmed it is working: - for idx in (start, end) where a param is named end → infinite loop. Root cause: end is a reserved keyword (used in list at end = x), and referencing it as a plain
  value silently emits /* unknown_primary */, producing an unbounded start.. range. I hit this live — multiple runaway processes had to be killed. Worked around it
  in the example; the transpiler doesn't currently stop you from naming something end.

  check it is working?: raw name = rust <block> (the one form the old raw-validator required) never worked in codegen either — confirmed in the earlier raw-rules work, documented with a
  working example instead.

  - prescan_check_duplicate_decls doesn't fire at all — verified with isolated repros for both duplicate struct and duplicate fn names. validation_test.deor's
  expectations were adjusted to reflect this (with a comment explaining why), rather than silently asserting something that isn't true.



## New Audit

 Major — docs describe validation that doesn't exist

  Working On: 2. raw rules in enforced_practices.md are overstated, and its own example contradicts the rule. Only two checks exist: check_raw_assignment (must come from a
  rust block) and check_raw_in_binding (catches string val = raw_var, a typed binding only). The doc claims "consuming a raw variable outside a rust block is a
  transpiler error" and lists int cnt = len(index) as an error case — but passing a raw var as a function argument or into len() isn't checked at all. Worse, the
  doc's own "Correct" example passes index into lookup(index, search_key) — a plain function call outside a rust block — and calls it fine, directly undercutting
  the rule stated two lines above it.

  Needs to be worked:

   3. "No nested functions" is not transpiler-validated. enforced_practices.md calls this a transpiler error; there's no validator for it. A nested fn would only
  fail later at rustc, so the doc mislabels which stage catches it. -- Fix: Add a validator check to this in token validation (if it belongs there)

  4. validator_types.md's "literal predicate failure" section is wrong about when the check happens. It shows Squarefeet area = -1 as a compile-time error. In
  reality (tb_validator.deor), codegen just emits Squarefeet::new(-1) returning Option<Squarefeet> — the predicate runs at runtime, not transpile time. As written,
  the doc would lead someone to expect a build failure that won't happen.
  
  5. strings.md: "mixed string/int + is a transpiler error" — no such type-checking exists anywhere in the validators; not enforced.

  Minor

  - builtins.md:144 says input_list has type strList — the actual type used everywhere in lib/ is stringList. Isolated typo, not a systemic rename miss.
  - syntax.md lists crash as a reserved keyword; it's actually just a regular builtin function name (no KW_CRASH in the lexer), just with special arg-count
  validation.
  - none is a real blocked identifier (lexer rejects it as a keyword) but isn't mentioned in any doc.
  - shapes.md/validator_types.md don't mention that struct-field validation only checks for func-shape fields — an invalid/undefined type name in a struct field
  silently passes the prescan and only errors later at codegen, which could be confusing to debug.
  - examples.md, experimental.md, shims.md are still 3-line stubs (examples.md = "Coming soon", experimental.md = empty, shims.md just redirects to libs.md) —
  matches what roadmap.md already tracks as unfinished.



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

## Audit Documentation
*Large Lift* | *Critical Priority*

-- prolly about 75% done as of July 1st

A lot of the documentation needs refreshed it is based on an old AI structure I build in pair-theorizing with AI about the Deor programming language, as I have been implementing it with Claude we have started going off the beaten track and a lot of the docs are either out of date or in great need of adjusting.

In need of extra auditing:
- Conditionals
- Enforced Practices


## Add Better Onboarding Document
*Large Lift* | *Low Priority*
It would be good to start a doc that takes users from 0 to completion of at least a small sized project


