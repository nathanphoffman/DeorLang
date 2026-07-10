AI DONT TOUCH THIS DOCUMENT, THIS IS FOR NATE ONLY
---
# Road Map
Upcoming features, mostly an internal markdown file used by the creator Nathan Hoffman.

---
## Working On 


Reviewing docs stopped at immutability from the top

fix for numeric range iteration for in x

thing = thing remove at idx   <---- in this case this statement gets totally dropped no validation, just commented in the rust transpiler, it is invalid
  since only thing remove at idx is needed here but bad design

We shouldnt deploy railway to other projects, make sure the config is sensible

Check to see if block is supported and if documented?

Check to make sure enforced ordering in docs is covered

Check to make sure we cover well that s_join is the way to concat

New web crossover: Update black and white editors to have a more clear highlighting syntax

-
Finalize syntax highlighting and port over latest new web build
---
## Audit Documentation
*Large Lift* | *Critical Priority*

-- prolly about 85% done as of July 2nd

A lot of the documentation needs refreshed it is based on an old AI structure I build in pair-theorizing with AI about the Deor programming language, as I have been implementing it with Claude we have started going off the beaten track and a lot of the docs are either out of date or in great need of adjusting.


---
## Designated Low Priority
- args()/input() destructuring silently drops unrecognized field names (e.g. a typo) instead of erroring — undocumented either way.