
# Go Back To
- Conditionals
- Enforced Practices


# For Nate
- Do we have any form of union types in the transpiler?
- Check on empty vs [] for lists

# Transpiler Add
- We should extend avow / validator types -- Ask about avow I am not sure we should actually do this.
  - Positive num = -5 no longer panics, it assigns none under the hood (lie in deor syntax)
  - However users can also assign a lie to a Validator Type (Option) value: Positive num = lie  (lie is a none)
  - if num is lie  -- this works and is the same as if num is not Positive
  - if num is Positive -- this works and is the same as if num is not lie I like it because you can avoid negation
  - avow now works naturally with this language, you don't avow a lie
  - this also allows error handling rust style (when paired with crash)
- Validator: Change type alias to PascalCase
- Validator: We should not allow any & | ^ < > { } unless it is in string data
- Validator: Do we check for rust keywords like mut? we should have an exception to catch rust-named keywords that are not in rust blocks
- While lists can be given data myList as [one,two,three], they cannot be assigned [] only empty so myList as empty not myList as []
- Add validation to prevent primitives/structs from being assigned to none, only Custom Types can be assigned none

# Deor Pickyness
- All functions limited to no more than 3 parameters
- All function calls must be passed an explicit argument (no magic data)
  - this is even true for system functions for in range(start,end) not for in range(1,10) 
  - this is even true for errors throw http_exception not throw "HTTP OUT OMG!"
  - and in ALL cases
- new lists must be composed with empty if not assigned data
- if lists are assigned data they can be assigned literally like listStuff = [employee, item2, item3]
  - however, importantly, these items must be named.  not ["Nate", "stuff", "thing"] they follow
  - the same rull as function variable passing (except of course they can have more than three)

