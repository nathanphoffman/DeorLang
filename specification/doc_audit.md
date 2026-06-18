# For Nate
- Look at how ranges work in transpiler vs doc vs my expectation
- See if this is true in transpiler today: Because they are built-ins, they accept literals directly. User-defined functions require named variables
- Do we have any form of union types in the transpiler?

# Transpiler Add
- We should extend avow / validator types
  - Positive num = -5 no longer panics, it assigns none under the hood (lie in deor syntax)
  - However users can also assign a lie to a Validator Type (Option) value: Positive num = lie  (lie is a none)
  - if num is lie  -- this works and is the same as if num is not Positive
  - if num is Positive -- this works and is the same as if num is not lie I like it because you can avoid negation
  - avow now works naturally with this language, you don't avow a lie
  - this also allows error handling rust style (when paired with crash)
- We need to make sure lists [...], and functions only take named variables when passed
- Validator: Change type alias to PascalCase
- Validator: We should not allow any & | ^ < > { } unless it is in string data
- Validator: Do we check for rust keywords like mut? we should have an exception to catch rust-named keywords that are not in rust blocks
- While lists can be given data myList as [one,two,three], they cannot be assigned [] only empty so myList as empty not myList as []
- Add validation to prevent primitives/structs from being assigned to none, only Custom Types can be assigned none

# Doc Fixes
- Specify names as empty is fine in the docs and so is for arrays listString names = empty, we should not allow []
- Check import we don't want to enforce import I like () better

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

## Naming
- enums, structs, and custom types (type validators) MUST be PascalCase
  - think structure = PascalCase
  - the logic behind this is it stands out boldly, but blends together as boldness > readability
- shapes must be camelCase
  - think aliasing = camelCase
  - the logic behind this is it stands out fairly well, but blends together as boldness = readability
- variable and function names must be snake_case
  - think runtime items = snake_case
  - the logic behind this is that these are very important to be readable as readability > boldness
- constants must be SCREAMING_SNAKE
  - think runtime item but it SCREAMS louder than the rest
  - the logic behind this is that these are THE MOST IMPORTANT to be read attention > readability > boldness


