AI DONT TOUCH THIS DOCUMENT, THIS IS FOR NATE ONLY

# Road Map
Upcoming features, mostly an internal markdown file used by the creator Nathan Hoffman.

## Working On 

Working On: We should not allow built-ins to be shadowed or used as names, I saw this in the docs, I want the docs updated and transpiler updated
           `crash` is **not** a reserved keyword — 
          +it's a regular builtin function name with its own argument-count validation (see [Builtins](docs/builtins.md#crash)); nothing stops it from being shadowed as a variable or function name, though doing so is not recommende
          +d.  


  Turns out this is actually a pretty big pain due to Rusts borrow &str vs String, sort of fixed but not completely as of 7/2
  ---------
  Working On: 3. strings.md's mixed string/int + claim — confirmed false, and not just "unenforced." I ran it end to end: "count: " + count (int) transpiles, compiles, and actually runs correctly, printing count: 5. + on strings becomes format!(),
  which happily accepts any Display-able type. There's no type-checking near + anywhere in the validators. This isn't a gap in enforcement — the docs describe a restriction that doesn't exist and the feature actually works as written.


  Began initial work:
  -- The decision was not checking for dup imports is fine for now, but checking for dup in the same file should be done, the docs have been updated but not sure about the transpiler
  -- I am also considering something like a main flag like USE_EXPLICIT
  1. prescan_check_duplicate_decls doesn't fire — confirmed, and it's worse than the existing tracking comment says. I tested duplicate struct Foo and duplicate fn add_values in the same file: both transpile with zero errors. But it's not
  that the check's own logic is broken — it's that importer/dedup.deor's deduplicate_decls runs before validation (during import-merging) and silently keeps only the first declaration with any given name, dropping the second entirely
  from the token stream. So the validator never even sees a duplicate to flag — by the time it runs, there's only one Foo left. This isn't "produces a confusing rustc error later" either — I checked the generated Rust and it's clean,
  single-definition, compiles fine. The second declaration (and whatever the user wrote in it) is just silently discarded, no error anywhere in the pipeline. dedup.deor's own comment says it exists for legitimate diamond-import cases
  (same file imported twice via different paths), but it can't currently tell that apart from a genuine same-file naming mistake.


  Nothing begun (I think?), this is a pretty big bug and needs fixed:
  2. validator_types.md's "compile-time" claim — confirmed false. I transpiled, compiled, and ran Squarefeet area = 0 against a val > 0 predicate: it transpiles clean, rustc compiles clean, and only at runtime does is not valid correctly
  trigger. tb_validator.deor emits Type::new(val) -> Option<Type> — the predicate is genuinely a runtime check. Lines 70 and 225 need fixing.



---

## Proposed Features
I think we should consider adding in import hints that do nothing other than import the file as per normal import, but allow you to specify what you are using from it to help trace function use in a file

## Audit Documentation
*Large Lift* | *Critical Priority*

-- prolly about 85% done as of July 2nd

A lot of the documentation needs refreshed it is based on an old AI structure I build in pair-theorizing with AI about the Deor programming language, as I have been implementing it with Claude we have started going off the beaten track and a lot of the docs are either out of date or in great need of adjusting.