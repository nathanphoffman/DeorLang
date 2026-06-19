
AI DONT TOUCH THIS DOCUMENT THIS IS FOR NATE ONLY

# In progress

- We have to implement is empty better for the check
it looks like lists can't compare on empty in the transpiler, this fails in hello.deor     listString thing = empty                                      
      if thing is empty                                                                                                                                      
          print("array is empty")      

- Make sure proper import paths / relative paths are understood for docs

# More Stuff

remove from loops or clarify it:
  - r_join[...] in loops.md — references a function that doesn't exist anywhere

  Documented Incorrectly (10 items)
  - collections_test.deor example — calls contains and to_upper which don't exist; should be s_contains/s_to_upper

  ---
  General Concerns
  3. hello.deor line 10 — (hello, world) = test uses = instead of in; syntax error per spec
  4. hello.deor macro inside function — macro defined inside fn void main(), which violates the flat-structure rule


# Done just needs documented

Clarification: validator types can only be compared on bad and avowed, update bad and empty behavior based on this:

Important: Use empty for lists, bad for values, clearup that distinction empty and bad should not be interchangeable.  bad is for validator types empty for lists.  So a list of a validator type could be empty or have multiple bad values inside it, but the values inside could never be empty and the list could never be bad.  This also means lists should be banned from being validator types.  So you cant have a type natesdeal(listString: stringList) that is not allowed.  Also we should prevent [] from being used to initialize an empty array, only empty can do that.  The only place [] should be used is in composing arrays with actual data/variables.
