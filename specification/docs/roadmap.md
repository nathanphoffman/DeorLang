# Roadmap
Note the roadmap is more to list out my raw notes and get feedback, however it is subject to frequent change and is not written in a singular viewpoint, I frequently will switch from personal to general throughout this.
---

## Make file imports global
I decided this is easiest for brevity

## Good Candidates for Macro Simplification
  1. is_mut/mut_kw guard — 4 lines, appears 8 times in codegen_stmt.deor. Closes over mut_names and a target variable name. This is the clear winner.
  2. gen_expr call + destructure — 3 lines, appears 10+ times throughout codegen. Closes over tokens, val_pos, ctx. Every statement handler does this before
  emitting code.
  3. Peek name token — 6 lines, appears 5 times in tokens_validation.deor. This is exactly what you were already trying to do with validate_ident — peek at
  the token after a keyword and run the naming checks.
  4. min3 + style check pair — pairs naturally with #3 into a single macro that does the full peek-and-validate in one shot.

## Audit Documentation
*Large Lift* | *Critical Priority*

A lot of the documentation needs refreshed it is based on an old AI structure I build in pair-theorizing with AI about the Deor programming language, as I have been implementing it with Claude we have started going off the beaten track and a lot of the docs are either out of date or in great need of adjusting.

In need of extra auditing:
- Conditionals
- Enforced Practices

## Private
*Small Lift* | *Medium Priority*
Filters the item in from being imported, done on a file level sould be easy to implement with the transpiler in the existing import logic which is stage 1 of the pipeline.

## Transpiler Performance Improvement
*Large Lift* | *Medium Priority*

The codegen loop in generate_rust_from_tokens builds output like this:

  output = s_cat(output, pr_code(result))

  In Rust, s_cat(a, b) allocates a new String and copies a entirely into it on every call. Since output grows with each declaration processed, by the time
  you're on the 100th function, you're copying ~50KB just to append a few hundred bytes. Do that for 200 declarations and you've done roughly O(n²) total
  copying in output size.

  Same pattern appears inside gen_block / gen_stmt — each statement appends to the growing code string by the same mechanism.

  The fix is to accumulate into a Vec<String> and join once at the end, which is already done in some places (e.g. stmts in gen_block) — but the top-level
  output accumulator and several intermediate builders still use the s_cat pattern. In Rust terms, this is the difference between repeated String::push_str in
  a loop (O(n²)) and collect::<Vec<_>>().join("") (O(n)).

  Current cost: ~3.1s out of ~3.6s total runtime. It's the last major bottleneck.

## Individual File Mapping Output
Another important logical step is to have the files outputted 

## Add Better Onboarding Document
*Large Lift* | *Low Priority*
It would be good to start a doc that takes users from 0 to completion of at least a small sized project

---
# Smaller items that are in no hurry
---

## Should empty and [] stay?
*Small Lift* | *Medium Priority*
Right now both empty and [] exist which violates the core concepts in Deor, there should be a limited number of ways to write code two explicit ways seems odd. Taking the side of empty (which I am now leaning away from):
- Pro: it reads more like a book -- aligns with core principles
- Pro: it gets rid of more [] symbols -- aligns with core principles
- Lean-Con: fn([]) to makes more contextual sense than fn(empty) but if it's more than one param it must be named anyway, giving neither a strong edge -- so this is a pretty specific use case, one param and empty both needed.
- Con: it is harder to recognize and in code that reads entirely like a book it might be getting too hard to pick out
- Con: [1,2,3] when filled in is already used for defining literal array data and () is might too cluttered

## Add additional validation
- Validator: We should not allow any & | ^ < > { } unless it is in string data
  - Some of this has already been done, but not quite all yet validator is being built out
- Validator: Do we check for rust keywords like mut? we should have an exception to catch rust-named keywords that are not in rust blocks
  - No but this is a todo
  
## Add Async Lib Shims If they Don't Exist
Try to make it more like Go channels if possible

## Crash has 2 Arguments
Crash wraps panic! and so likely provides 2 arguments, the second of which is for string iterpolation which is anti-Deorian, so we might want to force it to 1 argument, string only and force s_join or something similar.

## Remove insert (only support end)
```list at end = value``` is already supported, but we should remove insert which I believe is also in the transpiler. 

---
# Quick Notes
---

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
