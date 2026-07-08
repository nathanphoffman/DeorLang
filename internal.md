AI DONT TOUCH THIS DOCUMENT, THIS IS FOR NATE ONLY
---
# Road Map
Upcoming features, mostly an internal markdown file used by the creator Nathan Hoffman.

---
## Working On 

Add enforced ordering using best practices:

Imports - Everything else could use it, relies on nothing else in the file
Enums - Relies likely on nothing else in the file
Structs - Reliant on most everything above but still structural (so above functions)
Shapes - Shapes can reference almost anything above
Types - Type validators being types must be defined early
Macros (macros that depend on another must be listed below its dependency)
Functions - Reliant on everything above

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