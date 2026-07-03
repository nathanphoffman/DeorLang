AI DONT TOUCH THIS DOCUMENT, THIS IS FOR NATE ONLY
---
# Road Map
Upcoming features, mostly an internal markdown file used by the creator Nathan Hoffman.

---
## Working On 

Docs are likely incorrect for this:
  2. I hit one real transpiler quirk: list-shape (stringList) params compile to owned Vec<String> here, not the &Vec<T> the docs describe — needed an explicit
  &lines in xs_run. Noting it since it's the same category of "docs vs. actual compiler" mismatch as the string-concat bug earlier.

Finalize syntax highlighting and port over latest new web build
---
## Audit Documentation
*Large Lift* | *Critical Priority*

-- prolly about 85% done as of July 2nd

A lot of the documentation needs refreshed it is based on an old AI structure I build in pair-theorizing with AI about the Deor programming language, as I have been implementing it with Claude we have started going off the beaten track and a lot of the docs are either out of date or in great need of adjusting.


---
## Designated Low Priority
- args()/input() destructuring silently drops unrecognized field names (e.g. a typo) instead of erroring — undocumented either way.