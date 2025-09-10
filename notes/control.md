# Control Flow

#fundamentals #control-flow #pattern-matching #loops

> Essential programming constructs - `match` is crucial for [[enums]] and [[option]]

**Related Topics**: [[enums]] | [[option]] | [[errors]] | [[ownership]]

## Overview

Rust provides powerful control flow constructs that enable you to control program execution. All control flow constructs in Rust are expressions, meaning they can return values.

---

## If Expressions

If expressions allow you to branch your code based on conditions. Unlike many languages, `if` is an expression in Rust, meaning it evaluates to a value.

```rust
// Basic if statement
let number = 6;

if number % 4 == 0 {
    println!("number is divisible by 4");
} else if number % 3 == 0 {
    println!("number is divisible by 3");
} else if number % 2 == 0 {
    println!("number is divisible by 2");
} else {
    println!("number is not divisible by 4, 3, or 2");
}

// If as an expression (returns a value)
let condition = true;
let number = if condition { 5 } else { 6 };  // Both arms must return same type
println!("The value of number is: {}", number);

// Multiple conditions with else if
let score = 85;
let grade = if score >= 90 {
    "A"
} else if score >= 80 {
    "B"
} else if score >= 70 {
    "C"
} else {
    "F"
};
```

### If Let Pattern Matching

The `if let` syntax lets you handle one matching pattern while ignoring the rest:

```rust
let some_value = Some(3);

// Verbose match expression
match some_value {
    Some(3) => println!("three!"),
    _ => (),
}

// More concise if-let
if let Some(3) = some_value {
    println!("three!");
}

// Can include an else clause
if let Some(x) = some_value {
    println!("Got a value: {}", x);
} else {
    println!("No value");
}
```

> 💡 `if let` is perfect for [[option]] and [[errors#Result<T,E> Enum]] handling

---

## Loops

Rust provides three kinds of loops: `loop`, `while`, and `for`. All can be controlled with `break` and `continue`.

### loop

The `loop` keyword creates an infinite loop that runs until explicitly stopped with `break`.

```rust
// Basic infinite loop with break
let mut counter = 0;

let result = loop {
    counter += 1;

    if counter == 10 {
        break counter * 2;  // Returns a value from the loop
    }
};

println!("The result is {}", result);  // Prints "The result is 20"
```

### For

The `for` loop is used to iterate over elements of a collection or a range.

```rust
// Iterating over a collection
let a = [10, 20, 30, 40, 50];

for element in a {
    println!("the value is: {}", element);
}

// Iterating over a range
for number in 1..4 {  // Exclusive range (1, 2, 3)
    println!("{}", number);
}

// Countdown with a reverse range
for number in (1..4).rev() {  // (3, 2, 1)
    println!("{}!", number);
}
println!("LIFTOFF!!!");
```

## While

The `while` loop runs as long as a condition is true.

```rust
// Basic while loop
let mut number = 3;

while number != 0 {
    println!("{}!", number);
    number -= 1;
}

println!("LIFTOFF!!!");

// Looping through a collection with while
let a = [10, 20, 30, 40, 50];
let mut index = 0;

while index < a.len() {
    println!("the value is: {}", a[index]);
    index += 1;
}
```

### While Let Pattern Matching

Similar to `if let`, `while let` keeps executing a block as long as a pattern matches:

```rust
let mut optional = Some(0);

// Continue running the loop as long as optional contains Some value
while let Some(i) = optional {
    if i > 9 {
        println!("Greater than 9, quit!");
        optional = None;
    } else {
        println!("i is {}", i);
        optional = Some(i + 1);
    }
}
```

---

## Pattern Matching with Match

The `match` expression is Rust's most powerful control flow construct. It allows you to compare a value against a series of patterns and execute code based on which pattern matches.

```rust
// Basic match expression
let number = 13;

match number {
    // Match a single value
    1 => println!("One!"),
    // Match multiple values
    2 | 3 | 5 | 7 | 11 | 13 => println!("This is a prime"),
    // Match a range
    13..=19 => println!("A teen"),
    // Default case
    _ => println!("Ain't special"),
}

// Match with enums
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

### Match Exhaustiveness

```rust
// All possible values must be handled
let boolean_value = true;
match boolean_value {
    true => println!("It's true!"),
    false => println!("It's false!"),
}  // ✅ Exhaustive - covers all bool values

// Using _ for catch-all
let number = 13;
match number {
    1 => println!("One"),
    2 => println!("Two"),
    _ => println!("Something else"),  // Handles all other i32 values
}
```

### Match Guards

```rust
let num = Some(4);

match num {
    Some(x) if x < 5 => println!("less than five: {}", x),
    Some(x) => println!("{}", x),
    None => (),
}
```

### Destructuring in Match

```rust
// Destructuring tuples
let point = (3, 5);
match point {
    (0, 0) => println!("Origin"),
    (0, y) => println!("On Y axis at {}", y),
    (x, 0) => println!("On X axis at {}", x),
    (x, y) => println!("Point at ({}, {})", x, y),
}

// Destructuring structs
struct Point { x: i32, y: i32 }
let p = Point { x: 0, y: 7 };

match p {
    Point { x, y: 0 } => println!("On the x axis at {}", x),
    Point { x: 0, y } => println!("On the y axis at {}", y),
    Point { x, y } => println!("On neither axis: ({}, {})", x, y),
}
```

---

## Loop Control

### Break and Continue

```rust
// Basic break and continue
for i in 0..10 {
    if i == 2 {
        continue;  // Skip rest of this iteration
    }
    if i == 8 {
        break;     // Exit the loop
    }
    println!("{}", i);
}
```

### Loop Labels

Labels allow you to specify which loop you want to `break` or `continue` when you have nested loops.

```rust
// Using loop labels
'outer: loop {
    println!("Entered the outer loop");

    'inner: loop {
        println!("Entered the inner loop");

        // This breaks the inner loop
        // break;

        // This breaks the outer loop
        break 'outer;
    }

    println!("This point will never be reached");
}

println!("Exited the outer loop");

// Using continue with labels
'outer: for x in 0..3 {
    'inner: for y in 0..3 {
        if x == 1 && y == 1 {
            continue 'outer;  // Skip to next iteration of outer loop
        }
        println!("({}, {})", x, y);
    }
}
```

---

## Functions

Functions consists of a set of one or more statements (ending with `;`) but must end with an expression.

The expression can be evaluated implicitly.
### Basic Function Syntax

```rust
// Function with parameters and return type
fn add_numbers(x: i32, y: i32) -> i32 {
    x + y  // Expression return (no semicolon)
}

// Function with explicit return
fn multiply(x: i32, y: i32) -> i32 {
    return x * y;  // Early return with semicolon
}

// Function with no return value (returns unit type ())
fn print_sum(x: i32, y: i32) {
    println!("Sum: {}", x + y);
}
```

### Function Expressions vs Statements

```rust
fn example() -> i32 {
    let x = 5;  // Statement (ends with semicolon)

    let y = {   // Block expression
        let x = 3;
        x + 1   // Expression (no semicolon) - returns 4
    };

    y + x  // Expression return - returns 9
}
```

---

## Common Patterns

### Early Returns

```rust
fn divide(dividend: f64, divisor: f64) -> Option<f64> {
    if divisor == 0.0 {
        return None;  // Early return
    }
    Some(dividend / divisor)
}
```

### Loop with Accumulator

```rust
fn factorial(n: u32) -> u32 {
    let mut result = 1;
    let mut i = 1;

    loop {
        if i > n {
            break;
        }
        result *= i;
        i += 1;
    }

    result
}
```

### Iterator-Style Control Flow

```rust
// Traditional loop
let mut sum = 0;
for i in 1..=10 {
    if i % 2 == 0 {
        sum += i;
    }
}

// Functional style (preview of [[iterators]])
let sum: i32 = (1..=10)
    .filter(|&x| x % 2 == 0)
    .sum();
```

---

## See Also
- [[enums]] - Pattern matching is essential for enums
- [[option]] - `if let` and `match` patterns for `Option<T>`
- [[errors]] - Pattern matching for error handling
- [[ownership]] - Control flow affects ownership and borrowing

**Practice**: `exercises/02_functions/`, `exercises/03_if/` | **Review**: [[rust-review-guide#Foundation Phase]]
