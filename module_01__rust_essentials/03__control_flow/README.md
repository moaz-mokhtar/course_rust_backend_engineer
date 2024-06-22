# Control Flow in Rust


## If Else Statement
The if else statement in Rust allows you to execute different code blocks based on whether a condition is true or false. It's a fundamental control flow mechanism for making decisions in your code.

```rust
if condition {
    // Code to execute if the condition is true
} else {
    // Code to execute if the condition is false
}
```

## Looping Statements
Rust provides several looping constructs to repeatedly execute code blocks.
- Loop Statement: An infinite loop that continues until explicitly broken.
- While Statement: Executes a block of code as long as a condition is true.
- For Loop Statement: Iterates over a sequence of items, executing a block of code for each item.
```rust
// Infinite loop
loop {
    // Code to execute indefinitely
}

// While loop
while condition {
    // Code to execute as long as the condition is true
}

// For loop
for item in collection {
    // Code to execute for each item in the collection
}
```

## Pattern Matching with Match Expression
Pattern matching with the match expression is a powerful feature in Rust that allows you to compare a value against a series of patterns and execute code based on which pattern matches. It's particularly useful for handling enums and optionals.
```rust
match value {
    pattern1 => {
        // Code to execute if pattern1 matches
    },
    pattern2 => {
        // Code to execute if pattern2 matches
    },
    _ => {
        // Code to execute if none of the above patterns match
    }
}
```

## Option Enum
The Option enum is a way to handle the possibility of absence of a value. It has two variants: Some(T) for a value and None for the absence of a value. This is particularly useful for avoiding null pointer exceptions and making your code safer.
```rust
let some_value: Option<i32> = Some(5);
let no_value: Option<i32> = None;

match some_value {
    Some(value) => println!("Got a value: {}", value),
    None => println!("No value"),
}
```




## References
- Rust book - control flow https://doc.rust-lang.org/book/ch03-05-control-flow.html 
- Rust book - match control flow https://doc.rust-lang.org/book/ch06-02-match.html 
- Rust book - The Option enum https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html?highlight=option#the-option-enum-and-its-advantages-over-null-values 
- Rust Reference - match expressions https://doc.rust-lang.org/reference/expressions/match-expr.html 