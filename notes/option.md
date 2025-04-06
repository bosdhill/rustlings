# Option

## `Option` Enum vs Null Values
The Option type encodes the very common scenario in which a value could be something or it could be nothing. This allows the compiler to check whether you've handled all the cases you should be handling.

The `Option` enum is an alternative to `null` or `nil` exception handling. As such, Rust does not have nulls, but it does have an enum that can encode the concept of a value being present or absent. This enum is `Option<T>`, and it is [defined by the standard library](https://doc.rust-lang.org/std/option/enum.Option.html) as follows:

``` rust
enum Option<T> {
    None,
    Some(T),
}
```

Here are some examples of using `Option` values to hold number types and char types:
``` rust
let some_number = Some(5);
let some_char = Some('e');

let absent_number: Option<i32> = None;
```

Why is having `Option<T>` any better than having null? Because `Option<T>` and `T` are different types, the compiler won't let us use an `Option<T>` value as if it were definitely a valid value:
``` rust
let x: i8 = 5;
let y: Option<i8> = Some(5);
// ERROR! Will not compile
let sum = x + y;
```

## Unwrapping Options
To get the inner value from an `Option`, you need to "unwrap" it. There are several methods:

``` rust
// The unwrap method extracts the value in Some, or panics if None
let x = Some(5).unwrap(); // x = 5
// let y = None.unwrap(); // This would panic!

// expect is like unwrap but with a custom error message
let x = Some(5).expect("Value should be present"); // x = 5
// let y = None.expect("Value missing!"); // Panic with message "Value missing!"

// unwrap_or provides a default value when None
let x = Some(5).unwrap_or(0); // x = 5
let y = None.unwrap_or(0);    // y = 0

// unwrap_or_else uses a closure to generate the default
let x = Some(5).unwrap_or_else(|| expensive_computation()); // x = 5
let y = None.unwrap_or_else(|| expensive_computation());    // y = result of expensive_computation()
```

## `match`
The `match` expression is the primary way to handle `Option` values:

``` rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);     // Some(6)
let none = plus_one(None);    // None
```

### If-let
For simpler cases when you only care about one pattern, `if let` provides more concise syntax:

``` rust
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

### While-Let
Similar to `if let`, `while let` keeps executing a block as long as a pattern matches:

``` rust
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

### Nested `Some(Some(x))`
When dealing with nested Options, pattern matching handles them elegantly:

``` rust
let nested: Option<Option<i32>> = Some(Some(42));

match nested {
    Some(Some(x)) => println!("Double Some: {}", x),
    Some(None) => println!("Outer Some, inner None"),
    None => println!("None"),
}

// Can also be handled with if-let
if let Some(Some(x)) = nested {
    println!("Got value: {}", x);
}

// Or chained methods
let result = nested.and_then(|inner| inner);  // Some(42)
```