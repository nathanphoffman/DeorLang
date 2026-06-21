# Best Practices
Style recommendations for idiomatic recommendations, they are not enforced by the transpiler.

---
## Order of Declaration
1. Imports - Everything else could use it, relies on nothing else in the file
2. Enums - Relies likely on nothing else in the file
3. Consts - They could rely on enums and are important to see (at top)
4. Types - Type validators being types must be defined early
5. Structs - Reliant on most everything above but still structural (so above functions)
6. Shapes - Shapes can reference almost anything above, including structs
7. Functions - Reliant on everything above

---
## Spacing
Add a blank line before `return` in any function body that contains more than one statement. One-liner functions (single expression, no bindings) are exempt. Try to keep blocks spaced out keeping like concepts with one another.

**Recommended:**
```
shape rollResultList = list of RollResult

fn int sum_rolls(rollResultList rolls)
    # is its own thing, just printing new line after these 2 lines
    roll_notice as "Going to roll"
    print(roll_notice);

    # belongs together as it is all part of the loop
    sum as 0
    for roll in rolls
        value in roll
        int val as 0
        if value is not bad
            val = (avow value)
        sum = sum + val

    return sum
```

**Exempt — one-liner, no blank line needed:**
```
fn int square(int val)
    val * val
```
---
## Destructuring

Field order does not matter in destructuring — fields are matched by name. Any subset in any order is valid.
```
struct Employee
    int employee_id
    string first_name
    string last_name

(employee_id, first_name, last_name) in employee
```


Additionally, all `in` extractions should appear before any logic (assignments, expressions, control flow) within their block. Applies to function bodies, loop bodies, and if/else bodies.

**Correct:**
```
fn RollResult roll_die(Die die)

    (sides, label) in die

    min as 1
    int raw = random(min, sides)

    Roll value = raw
    string source = label
    RollResult result = (value, source)

    return result
```
## Naming External Libs
Because deor does not support any third party importing, the standard convention is just to copy and paste deor files into the lib folder. Because of this, it is highly recommended the following pattern be used.

Rust Wrappers lead all functions with **x_func** so a string join would be ```s_join```.  These should closely if not exactly mirror existing rust code in the std library.

Cargo crate wrappers written in rust needed for imports should be of the form **cx_fn** so a cargo serde json tool might be ```cs_json_parse()```.  Note that these are for wrappers, just because something calls cargo does not make it a wrapper, a third party .deor file that relies on some cargo packages does not mean it gets the ```cx_``` prefix.

Finally, a personal third party item you created distributed by a blog, git, or copy paste should use the convention **ex_fn** or external.  So en_do_cool_things() might be the nate library of cool deor tricks.  These might import rust code, use rust blocks, or use cargo, but they are at least doing some logic with deor, if not all logic.