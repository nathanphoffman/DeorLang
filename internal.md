AI DONT TOUCH THIS DOCUMENT, THIS IS FOR NATE ONLY
---
# Road Map
Upcoming features, mostly an internal markdown file used by the creator Nathan Hoffman.

---
## Working On 

Transpiler highlighting in vscode and site that is missing:
- import keyword (vscode and web)
- float (web) -- check other primitives on web too
- macro ,macro_run (web)
- return (web)
- Validator Types, like listString (web)
- lib types (web) from rust like TaskPool (web)
- const variable names (web)
- where clause for import substitution (web)
- else if and else, only if itself is highlighted (web)

GITHUB UPDATE Update git config to see my language on github
  I created my own language and transpiler written in it, it transpiles to rust.  Now must of the code is actually my own language Deor which looks nothing like Rust, but github flags all of it as being Rust, is there a way I can tell github my language or does it need to be big enough for github to show it?Examined configuration options and contribution pathways for custom language recognitionExamined configuration options and contribution pathways for custom language recognitionYou needn't wait for GitHub's imprimatur (official sanction). Add a .gitattributes file at the repo root:
  *.deor linguist-language=YourLangName
  This forces GitHub's Linguist (the underlying heuristic engine) to tag those files correctly, regardless of size.
  Two caveats:

  If Deor isn't in Linguist's known-languages list yet, linguist-language won't work by itself — you'd instead mark the transpiled Rust output as generated/vendored so it's excluded from stats:

Add Railway setups for the 2 sites and deploy

---
## Audit Documentation
*Large Lift* | *Critical Priority*

-- prolly about 85% done as of July 2nd

A lot of the documentation needs refreshed it is based on an old AI structure I build in pair-theorizing with AI about the Deor programming language, as I have been implementing it with Claude we have started going off the beaten track and a lot of the docs are either out of date or in great need of adjusting.


---
## Designated Low Priority
- args()/input() destructuring silently drops unrecognized field names (e.g. a typo) instead of erroring — undocumented either way.