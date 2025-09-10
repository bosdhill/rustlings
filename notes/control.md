# Control Flow

#fundamentals #control-flow #pattern-matching #loops

> Essential programming constructs - `match` is crucial for [[enums]] and [[option]]

**Related Topics**: [[enums]] | [[option]] | [[errors]] | [[ownership]]

## If

If expressions allow you to branch your code based on conditions. If the condition evaluates to `true`, the code block executes; if it's `false`, it skips the block.

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

// If in a let statement
let condition = true;
let number = if condition { 5 } else { 6 };
println!("The value of number is: {}", number);
```

## Loops

Rust provides several ways to perform repeated operations through loops.

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

## Match

The `match` expression allows you to compare a value against a series of patterns and execute code based on which pattern matches.

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

## if let

The `if let` syntax lets you handle one matching pattern while ignoring the rest.

```rust
// Match with Option<T>
let some_u8_value = Some(3);
match some_u8_value {
    Some(3) => println!("three"),
    _ => (),
}

// Same logic using if let (cleaner)
if let Some(3) = some_u8_value {
    println!("three");
}
```

## Labels

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
```

---

## See Also
- [[enums]] - Pattern matching is essential for enums
- [[option]] - `if let` and `match` patterns for `Option<T>`
- [[errors]] - Pattern matching for error handling
- [[ownership]] - Control flow affects ownership and borrowing

**Practice**: `exercises/02_functions/`, `exercises/03_if/` | **Review**: [[rust-review-guide#Foundation Phase]]
```
