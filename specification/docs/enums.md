# Enums

An `enum` declares a named set of variants. Each variant is a distinct value of that type. Enums are enforced as PascalCase names.

## Declaration

```
enum ColorTag
    Red
    Green
    Blue
    Orange
    Purple
    Yellow
    White
    Black
```

```rust
#[derive(Clone, Copy, PartialEq, Debug)]
enum ColorTag {
    Red,
    Green,
    Blue,
    Orange,
    Purple,
    Yellow,
    White,
    Black,
}
```

Any number of variants. Each is a plain name — no associated data.

---

## Assignment

```
ColorTag background = Blue
ColorTag foreground = White
```

```rust
let background: ColorTag = ColorTag::Blue;
let foreground: ColorTag = ColorTag::White;
```

---

## Checking Variants

Use `if` / `else if` with `is`. No pattern matching — the same `is` operator used for all equality in Deor.

```deor
if background is Blue
    msg as "blue background"
    print(msg)
else if background is Red
    msg as "red background"
    print(msg)
else
    msg as "other"
    print(msg)
```

```rust
if background == ColorTag::Blue {
    println!("{}", "blue background");
} else if background == ColorTag::Red {
    println!("{}", "red background");
} else {
    println!("{}", "other");
}
```

Exhaustiveness is not enforced — write an `else` branch as a catch-all when needed.

---

## In Structs and Function Signatures

Enums work as struct fields, function parameters, and return types — the same as any other type.

```deor
struct Theme
    string name
    ColorTag background
    ColorTag foreground

fn string describe(ColorTag color)
    other as "other"
    red as "red"
    green as "green"

    if color is Red    
        return red
    else if color is Green
        return green
    else 
        return other

fn main()
    name as "Ocean"
    ColorTag background = Blue
    ColorTag foreground = White
    Theme theme = (name, background, foreground)
    string label = describe(background)
    print(label)
```

```rust
struct Theme {
    name: String,
    background: ColorTag,
    foreground: ColorTag,
}

fn describe(color: ColorTag) -> String {
    if color == ColorTag::Red {
        return "red".to_string();
    } else if color == ColorTag::Green {
        return "green".to_string();
    }
    return "other".to_string();
}
```