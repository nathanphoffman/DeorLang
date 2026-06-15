# Enums

An `enum` declares a named set of variants. Each variant is a distinct value of that type. Enums are camelCase names; variants are PascalCase.

## Declaration

```
enum colorTag
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
colorTag background = Blue
colorTag foreground = White
```

```rust
let background: ColorTag = ColorTag::Blue;
let foreground: ColorTag = ColorTag::White;
```

---

## Checking Variants

Use `if` / `else if` with `is`. No pattern matching — the same `is` operator used for all equality in Deor.

```
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

Exhaustiveness is not enforced in v1 — write an `else` branch as a catch-all when needed.

---

## In Structs and Function Signatures

Enums work as struct fields, function parameters, and return types — the same as any other type.

```
struct Theme
    string name
    colorTag background
    colorTag foreground

fn string describe(colorTag color)
    if color is Red
        result as "red"
        return result
    else if color is Green
        result as "green"
        return result
    result as "other"
    return result

fn main()
    name as "Ocean"
    colorTag background = Blue
    colorTag foreground = White
    theme as (name, background, foreground)
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

---

## Conversion Notes

| Deor | Rust |
|---|---|
| `enum colorTag` + indented variants | `#[derive(Clone, Copy, PartialEq, Debug)] enum ColorTag { ... }` |
| `colorTag color = Red` | `let color: ColorTag = ColorTag::Red;` |
| `if color is Red` | `if color == ColorTag::Red` |
| enum as struct field | field type becomes `ColorTag` |
| enum as function param | param type becomes `ColorTag` |

Enum types derive `Clone`, `Copy`, `PartialEq`, and `Debug`. `Copy` is safe because enum variants carry no heap data — assigning or passing an enum never moves it.
