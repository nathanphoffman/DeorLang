# Deor Language Specification

Deor is a small, highly-procedural, tabbed-block language that transpiles to Rust. It enforces near-book readability, explicit typing, and predictable control flow — and exposes a `rust` block for when you need the full language.

Its goal is to provide a comfier entrance point to Rust with simple syntactical sugar and such uniform composition rules that there is little room for debate, and when there is debate -- then it offers ``rust`` blocks for that.

1) To create your first project in minutes vist [Getting Started](getting_started)

2) To learn about our core principles visit [Core Principles](core_principles)

3) To read the documentation visit [Documentation Index](index)

It took inspiration from, the following languages in this order:
- **Python** - block scopes and heavily opinionated readability -- Python doesn't go far enough! ;) 
- **Rust** - transpiles to it, allows raw rust blocks, supports a lot of similar patterns
- **Go** - a simple ultra light-weight base syntax
- **C#** - data types and data typing style
- **DD Design** - built in validator types
- **Haskell/React State** - immutable objects with exposed (de)construction tools (in/with/as)

