# Variables and Mutability

#fundamentals #variables #mutability #memory

> Foundation of Rust programming - how data is stored and modified

**Related Topics**: [[ownership]] | [[control#Functions]] | [[generics]] | [[lifetimes]]

## Overview

Variables in Rust are **immutable by default**. This is a key safety feature that prevents accidental data modification and enables the compiler to optimize your code.

---

## Variable Declarations

### Basic Variable Binding

```rust
// Immutable variable (default)
let x = 5;
println!("The value of x is: {}", x);

// x = 6;  // ERROR: cannot assign twice to immutable variable
```

### Mutable Variables

```rust
// Mutable variable with mut keyword
let mut x = 5;
println!("The value of x is: {}", x);

x = 6;  // OK: variable is mutable
println!("The value of x is: {}", x);
```

### Type Annotations

```rust
// Explicit type annotation
let x: i32 = 5;
let y: f64 = 3.14;
let z: bool = true;
let c: char = 'A';

// Rust can often infer types
let a = 5;      // i32 (inferred)
let b = 5.0;    // f64 (inferred)
let s = "hello"; // &str (inferred)

// Sometimes annotation is required
let parsed: i32 = "42".parse().expect("Not a number!");
// or
let parsed = "42".parse::<i32>().expect("Not a number!");
```

---

## Variable Shadowing

Shadowing allows you to declare a new variable with the same name, effectively hiding the previous one.

### Basic Shadowing

```rust
let x = 5;
let x = x + 1;    // Shadow with new value
let x = x * 2;    // Shadow again

println!("The value of x is: {}", x);  // 12
```

### Shadowing vs Mutability

```rust
// Shadowing can change type
let spaces = "   ";        // &str
let spaces = spaces.len(); // usize

// Mutability cannot change type
let mut spaces = "   ";
// spaces = spaces.len();  // ERROR: expected `&str`, found `usize`
```

### Shadowing in Scopes

```rust
let x = 5;

{
    let x = x * 2;  // Shadow in inner scope
    println!("Inner x: {}", x);  // 10
}

println!("Outer x: {}", x);  // 5 (original value restored)
```

---

## Constants vs Variables

### Constants

```rust
// Constants are always immutable and must have type annotation
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

// Constants can be declared in any scope
const MAX_POINTS: u32 = 100_000;

fn main() {
    const LOCAL_CONSTANT: i32 = 42;
    println!("Constant: {}", THREE_HOURS_IN_SECONDS);
}
```

### Variables vs Constants

| Feature | Variables (`let`) | Constants (`const`) |
|---------|-------------------|-------------------|
| Mutability | Can be mutable with `mut` | Always immutable |
| Type annotation | Optional (inferred) | Required |
| Shadowing | Allowed | Not allowed |
| Scope | Block scoped | Any scope |
| Initialization | Runtime or compile-time | Compile-time only |
| Naming | snake_case | SCREAMING_SNAKE_CASE |

---

## Primitive Types

### Integer Types

```rust
// Signed integers
let a: i8 = -128;      // 8-bit signed
let b: i16 = -32_768;  // 16-bit signed
let c: i32 = -2_147_483_648;  // 32-bit signed (default)
let d: i64 = -9_223_372_036_854_775_808;  // 64-bit signed
let e: i128 = -170_141_183_460_469_231_731_687_303_715_884_105_728; // 128-bit
let f: isize = -1;     // Architecture dependent (32 or 64 bit)

// Unsigned integers
let g: u8 = 255;       // 8-bit unsigned
let h: u16 = 65_535;   // 16-bit unsigned
let i: u32 = 4_294_967_295;  // 32-bit unsigned
let j: u64 = 18_446_744_073_709_551_615;  // 64-bit unsigned
let k: u128 = 340_282_366_920_938_463_463_374_607_431_768_211_455; // 128-bit
let l: usize = 100;    // Architecture dependent (32 or 64 bit)

// Number literals
let decimal = 98_222;        // Decimal
let hex = 0xff;              // Hexadecimal
let octal = 0o77;            // Octal
let binary = 0b1111_0000;    // Binary
let byte = b'A';             // Byte (u8 only)
```

### Floating Point Types

```rust
let x: f32 = 2.0;      // 32-bit float
let y: f64 = 3.0;      // 64-bit float (default)

// Scientific notation
let large = 1.23e4;    // 12300.0
let small = 1.23e-4;   // 0.000123
```

### Boolean Type

```rust
let t: bool = true;
let f: bool = false;

// Boolean operations
let and_result = t && f;  // false
let or_result = t || f;   // true
let not_result = !t;      // false
```

### Character Type

```rust
let c: char = 'z';
let z: char = 'ℤ';      // Unicode scalar value
let heart_eyed_cat = '😻';

// Characters are 4 bytes (UTF-8)
println!("Size of char: {}", std::mem::size_of::<char>()); // 4
```

---

## Compound Types

### Tuples

```rust
// Tuple with mixed types
let tup: (i32, f64, u8) = (500, 6.4, 1);

// Destructuring
let (x, y, z) = tup;
println!("x: {}, y: {}, z: {}", x, y, z);

// Accessing by index
let x = tup.0;  // 500
let y = tup.1;  // 6.4
let z = tup.2;  // 1

// Unit tuple (empty tuple)
let unit: () = ();  // Type and value are both ()
```

### Arrays

```rust
// Array with explicit type and size
let a: [i32; 5] = [1, 2, 3, 4, 5];

// Array with repeated value
let b = [3; 5];  // [3, 3, 3, 3, 3]

// Accessing elements
let first = a[0];  // 1
let second = a[1]; // 2

// Arrays have fixed size known at compile time
let months = ["January", "February", "March", "April", "May", "June",
              "July", "August", "September", "October", "November", "December"];

// Getting array length
println!("Array length: {}", a.len());  // 5
```

---

## Memory Layout and Stack vs Heap

### Stack-Allocated Types

```rust
// These types are stored on the stack
let x: i32 = 5;           // 4 bytes on stack
let y: f64 = 3.14;        // 8 bytes on stack
let z: bool = true;       // 1 byte on stack
let array = [1, 2, 3, 4]; // 16 bytes on stack (4 * i32)
let tuple = (1, 2.0);     // 12 bytes on stack (i32 + f64)

// Stack values are copied when assigned
let a = 5;
let b = a;  // a is copied to b, both are valid
println!("a: {}, b: {}", a, b);  // Both still accessible
```

### Heap-Allocated Types

```rust
// String is heap-allocated
let s1 = String::from("hello");
let s2 = s1;  // s1 is moved to s2, s1 no longer valid

// println!("{}", s1);  // ERROR: value borrowed after move
println!("{}", s2);   // OK: s2 owns the string now
```

---

## Variable Initialization and Patterns

### Uninitialized Variables

```rust
let x: i32;  // Declared but not initialized

// x + 1;    // ERROR: use of possibly-uninitialized variable

x = 5;      // Initialize before use
println!("x: {}", x);  // OK: now initialized
```

### Conditional Initialization

```rust
let condition = true;
let number = if condition { 5 } else { 6 };

// Both branches must return same type
// let number = if condition { 5 } else { "six" };  // ERROR: type mismatch
```

### Pattern Matching in Variable Declarations

```rust
// Destructuring tuples
let (a, b, c) = (1, 2, 3);

// Ignoring values with _
let (x, _, z) = (1, 2, 3);  // Ignore middle value

// Destructuring arrays (with patterns)
let [first, second, ..] = [1, 2, 3, 4, 5];  // first=1, second=2

// Using .. to ignore remaining elements
let [head, .., tail] = [1, 2, 3, 4, 5];  // head=1, tail=5
```

---

## Variable Naming Conventions

### Naming Rules

```rust
// Valid variable names
let name = "Alice";
let age = 30;
let is_student = true;
let first_name = "Bob";
let _internal = 42;        // Leading underscore for internal use
let x1 = 1;                // Numbers allowed (not at start)

// Invalid variable names
// let 1x = 1;             // Cannot start with number
// let first-name = "Bob";  // Hyphens not allowed
// let let = 5;            // Keywords not allowed
```

### Naming Conventions

```rust
// snake_case for variables and functions
let user_name = "alice";
let max_connections = 100;

// SCREAMING_SNAKE_CASE for constants
const MAX_POINTS: u32 = 100_000;

// PascalCase for types (structs, enums)
// struct UserAccount { ... }
// enum ConnectionStatus { ... }
```

---

## Scoping and Lifetime

### Block Scoping

```rust
let x = 5;  // x is valid from here

{               // Start new scope
    let y = 10; // y is valid from here
    println!("x: {}, y: {}", x, y);  // Both accessible
}               // y goes out of scope here

// println!("{}", y);  // ERROR: y not in scope
println!("{}", x);     // x still in scope
```

### Variable Lifetime in Functions

```rust
fn main() {
    let x = 5;          // x created

    {
        let y = 10;     // y created
        println!("Inner: x={}, y={}", x, y);
    }                   // y destroyed

    println!("Outer: x={}", x);
}                       // x destroyed
```

---

## Common Patterns and Idioms

### Multiple Variable Declaration

```rust
let (x, y, z) = (1, 2, 3);

// Or separately
let x = 1;
let y = 2;
let z = 3;
```

### Swap Variables

```rust
let mut a = 1;
let mut b = 2;

// Using tuple destructuring
(a, b) = (b, a);  // Swap values
println!("a: {}, b: {}", a, b);  // a: 2, b: 1

// Or using std::mem::swap
std::mem::swap(&mut a, &mut b);
```

### Default Values

```rust
// Using unwrap_or for default values
let config_value = std::env::var("CONFIG").unwrap_or_else(|| "default".to_string());

// Using match for complex defaults
let value = match some_option {
    Some(v) => v,
    None => {
        // Complex default calculation
        expensive_default_calculation()
    }
};
```

### Variable Guards

```rust
fn process_number(n: Option<i32>) {
    let Some(number) = n else {
        println!("No number provided");
        return;
    };

    // number is guaranteed to be valid here
    println!("Processing: {}", number);
}
```

---

## Performance Considerations

### Copy vs Move Types

```rust
// Copy types (stack-allocated primitives)
let x = 5;
let y = x;  // x is copied, both valid
println!("{} {}", x, y);  // OK

// Move types (heap-allocated)
let s1 = String::from("hello");
let s2 = s1;  // s1 is moved, no longer valid
// println!("{}", s1);  // ERROR
println!("{}", s2);   // OK
```

### Memory Efficiency

```rust
// Prefer stack allocation when possible
let numbers = [1, 2, 3, 4, 5];  // Stack-allocated array

// Use heap allocation when size is unknown at compile time
let dynamic_numbers = vec![1, 2, 3, 4, 5];  // Heap-allocated vector

// Consider the size of your data
println!("i32 size: {}", std::mem::size_of::<i32>());      // 4
println!("String size: {}", std::mem::size_of::<String>()); // 24 (on 64-bit)
println!("Vec<i32> size: {}", std::mem::size_of::<Vec<i32>>()); // 24 (on 64-bit)
```

---

## Common Mistakes and Solutions

### Trying to Modify Immutable Variables

```rust
// ❌ Wrong
let x = 5;
// x = 6;  // ERROR: cannot assign twice to immutable variable

// ✅ Correct
let mut x = 5;
x = 6;  // OK
```

### Uninitialized Variable Usage

```rust
// ❌ Wrong
let x: i32;
// println!("{}", x);  // ERROR: use of possibly-uninitialized variable

// ✅ Correct
let x: i32;
x = 5;
println!("{}", x);  // OK
```

### Type Mismatch in Conditional Assignment

```rust
// ❌ Wrong
let condition = true;
// let result = if condition { 5 } else { "five" };  // ERROR: type mismatch

// ✅ Correct
let result = if condition { 5 } else { 0 };  // Same type in both branches
```

### Integer Overflow

```rust
// ❌ Potential problem (in release mode)
let mut x: u8 = 255;
x = x + 1;  // Wraps to 0 in release mode, panics in debug mode

// ✅ Safe approaches
let x: u8 = 255;
let result = x.checked_add(1);  // Returns Option<u8>
match result {
    Some(value) => println!("Result: {}", value),
    None => println!("Overflow detected!"),
}

// Or use saturating arithmetic
let result = x.saturating_add(1);  // Returns 255 (max value)
```

---

## Integration with Other Concepts

### Variables and Ownership

```rust
let s1 = String::from("hello");
let s2 = s1;  // Ownership moved to s2

// This demonstrates how variables relate to [[ownership]]
// The variable binding determines ownership
```

### Variables and Functions

```rust
fn takes_ownership(some_string: String) {  // Variable comes into scope
    println!("{}", some_string);
}  // Variable goes out of scope and `drop` is called

let s = String::from("hello");
takes_ownership(s);  // s is moved into function
// s is no longer valid here
```

### Variables and Pattern Matching

```rust
let tuple = (1, 2, 3);

match tuple {
    (x, y, z) => {  // Variables created from pattern
        println!("x: {}, y: {}, z: {}", x, y, z);
    }
}  // Variables go out of scope
```

---

## See Also

- [[ownership]] - How variables relate to memory management
- [[control#Functions]] - Variables as function parameters and return values
- [[generics]] - Generic type parameters and variable declarations
- [[lifetimes]] - How long variable references are valid
- [[collections]] - Dynamic data storage beyond basic variables

**Practice**: `exercises/01_variables/` | **Review**: [[rust-review-guide#Foundation Phase]]
