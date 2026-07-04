# Learning Deor
I will spend only a few words in this section, **please skip to the next heading if you are short on time**. 

My name is Nathan Hoffman, I am the creator behind Deor. I want this document to be useful for people coding for the first time and technical people, hence why I am using two blocks that can be ignored by either party "Newcomers" and "Techies". This language also serves as a great starting point for people who struggle learning Rust, as it transpiles to Rust, has some strong Rust similarities, and allows raw rust to be written inside of rust blocks.

To give you an idea of why I created Deor, you have to understand my background: I was born into a sea of Commodore Amiga hardware and software in the late 80s through mid-90s when my dad's computer store closed; throughout much of my life, I have loved programming, and anytime I move away from it, I come right back like a boomerang. I began with C and JavaScript the age of 13 in the early 2000s; I moved to PHP for fun in the mid-late 00s, and then finally landed programming as a career starting around 2014, so it would not be an exaggeration to say I have been coding for a quarter century. It wasn't all roses though, in the 2020s I started to have doubts about my programming career due to a series of personal struggles, yet even while questioning it my love was so strong I still did it as a career and for fun. In 2026 everything changed, and my code love grew so strong that it may be the pinnacle of my coding love: so when I say I can stand behind this language, know it does come from enthusiasm that has been building all my life, with an energy helped by AI learning. It takes me back to my days on my dad's Amiga looking at console text scroll across the startup console in wonder.

Note: While AI helped with some Deor code and documentation, **I have never had AI directly generate anything on this page** These are **my words**, thank you for spending the time to read them! :)

## 1. Storing Information: *(Compiler Implicit as)*
> **Newcomers**: Variables are portions of information, think names, numbers, and lists. If you tell a computer to say hello, it needs to know what hello is, not just how to say it. Variables have names that are used to reference the value in the code. For some values you just type the value (like numbers), for others which are hard for the computer to determine from the context, what are called strings (human writing), you use "" to wrap the string. 

```deor
times_to_say_hello as 10
should_i_say_hello as true
fahrenheit as 75.5
message as "hello, the temperature is "
```

You will see other ways to declare variables in this documentation, but ```as``` is best when the context of the value is obvious (like it is above) but when assigning things from outside as we will see later, it can be less obvious, in which case it is better to use the explicit type (covered next!)

> **Techies**: Although these assignments look implicit, they actually are validated by the compiler, and the Rust compiler will hard-fail if it can't determine a good type to assign. All variables also get clone() applied in the Rust output, the Rust compiler will strip away cloned copy-types (so there is no added performance cost for copy-types), but it exists to make dealing with reference-types like Strings, Structs, and Vectors easier for the user at a cost to performance. To avoid this performance hit, the move keyword exists to allow ownership, and rust blocks exist for manual control (more on these later)


## 2. Types of Variables *(Explicit Types)*
> **Newcomers**: Data types expose the underbelly of computers. They tell the computer how to manage their memory, this was more important back in the day (and remains today for high-performance applications), but even back in the 60s some programming languages like Fortran allowed omitting these, so don't feel bad if you don't like them. These "data types" tell the computer what is expected to be stored in the variable both for safety and memory regulation, and it can matter in some languages where smaller or bigger values were expected. The computer can't know what information is coming from outside, and so it can't know how to prepare storage for it, afterall. By providing computers the type they can both guard against bad data being processed and improve performance by limiting how much memory is reserved for that value. Fahrenheit is obviously never going to be 1,000 trillion, but a computer (and non-Americans ;) ) don't know that! Deor simplifies things by making the default for these types very large, so 99.99% of the time you don't need to worry about it, and for safety the Rust code Deor generates is already safe: so the only real remaining factors are clarity and communicating expectations.

```deor
int times_to_say_hello = 10
bool should_i_say_hello = true
float fahrenheit = 75.5
string message = "hello, the temperature is "
```

The symbol = is used instead of as, a hold-over from older languages like C to make the convention more familiar to programmers who know what data types are, allowing people who don't care, to use the more friendly `as`
- ```int``` is a whole number (comes from math: integer)
- ```bool``` a true/false value (comes from George Boole, a founder of logical/discrete mathematics)
- ```float``` is a decimal value (it comes from the name floating-point calculation, basically a storage form of scientific notation) and can be inaccurate to use for extremely heavy amounts of math and financial data over many transactions due to limits of storage and binary accuracy. For regular day applications, you don't need to worry it has an accuracy of ~16 digits.
- ```string``` a portion of human-text comes from (a string of characters) a hold-over from early languages like C which treated strings as lists of individual characters.


> **Techies**: Floats use Double Precision 64-bit, and Integers use long precision 64-bit. This decision was made due to modern hardware and because it vastly simplifies types, for more explicit types you can use rust blocks.



