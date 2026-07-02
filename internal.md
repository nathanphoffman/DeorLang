AI DONT TOUCH THIS DOCUMENT, THIS IS FOR NATE ONLY

# Road Map
Upcoming features, mostly an internal markdown file used by the creator Nathan Hoffman.

## Audits

 Major — docs describe validation that doesn't exist

# More validation adds
- Fixing: Empty parens () should be validated out
- Open [ and Closing ] should be prevented on their own like we did with a leading and closing paren

  Needs to be worked:

   Checking: 3. "No nested functions" is not transpiler-validated. enforced_practices.md calls this a transpiler error; there's no validator for it. A nested fn would only
  fail later at rustc, so the doc mislabels which stage catches it. -- Fix: Add a validator check to this in token validation (if it belongs there)


  - prescan_check_duplicate_decls doesn't fire at all — verified with isolated repros for both duplicate struct and duplicate fn names. validation_test.deor's
  expectations were adjusted to reflect this (with a comment explaining why), rather than silently asserting something that isn't true.


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


