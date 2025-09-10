# Errors

#advanced-features #error-handling #result #pattern-matching

> Rust's approach to recoverable errors - builds on [[enums]] and [[option]] patterns

**Related Topics**: [[enums]] | [[option]] | [[traits]] | [[control#Match]]

## Overview

Rust handles errors through two main types:
- **`Option<T>`** - for values that might not exist (covered in [[option]])
- **`Result<T,E>`** - for operations that might fail with an error

This approach eliminates entire classes of runtime errors by making error handling explicit.

---

## `Result<T,E>` Enum

`Result` allows a way to tell us whether it succeeded or failed and at the same time give us
either the type of value we want or error information.

```rust
enum Result<T, E> {
    Ok(T),   // Success case with value of type T
    Err(E),  // Error case with error of type E
}
```

### Why Result over Exceptions?

```rust
// Other languages might throw exceptions
// parse_number("abc")  // throws ParseException

// Rust returns Result - explicit error handling
let result: Result<i32, ParseIntError> = "abc".parse();
match result {
    Ok(number) => println!("Parsed: {}", number),
    Err(error) => println!("Parse failed: {}", error),
}
```

For example, we can match the `Ok` and `Err` variants:

```rust
use std::fs::File;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {error:?}"),
    };
}
```

## Result Error Handling

We can handle different error variants in the match arm instead of just panics:

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            other_error => {
                panic!("Problem opening the file: {other_error:?}");
            }
        },
    };
}
```

## Using `Result` Helper Methods

1. `unwrap` lets us unwrap `Ok` variant and panics with `Err` otherwise

    ```rust
    use std::fs::File;

    fn main() {
        let greeting_file = File::open("hello.txt").unwrap();
    }
    ```

    will panic with:

    ```rust
    thread 'main' panicked at src/main.rs:4:49:
    called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }
    ```

2. `expect` is the same but lets you choose the panic message

    ```rust
    use std::fs::File;

    fn main() {
        let greeting_file = File::open("hello.txt")
            .expect("hello.txt should be included in this project");
    }
    ```

3. **Additional Result Helper Methods:**

    ```rust
    // unwrap_or provides a default value
    let result: Result<i32, &str> = Err("error");
    let value = result.unwrap_or(42);  // Returns 42

    // unwrap_or_else uses a closure for the default
    let value = result.unwrap_or_else(|_err| expensive_default());

    // is_ok() and is_err() for checking without consuming
    if result.is_ok() {
        println!("Success!");
    }
    ```

4. `unwrap_or_else` allows you to branch in the `Err` case for error handling.
   More concise using `unwrap_or_else` Result helper method:

    ```rust
    use std::fs::File;
    use std::io::ErrorKind;

    fn main() {
        let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
            if error.kind() == ErrorKind::NotFound {
                File::create("hello.txt").unwrap_or_else(|error| {
                    panic!("Problem creating the file: {error:?}");
                })
            } else {
                panic!("Problem opening the file: {error:?}");
            }
        });
    }
    ```

---

## The `?` Operator

The `?` operator is Rust's primary error propagation mechanism. It's syntactic sugar for early returns on errors.

Using just match expressions:

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}
```

Using the `?` operator:

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;  // Early return on error
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;      // Early return on error
    Ok(username)  // Success case
}
```

### How `?` Works

The `?` operator does the following:
1. If the `Result` is `Ok(value)`, it extracts `value`
2. If the `Result` is `Err(error)`, it returns `Err(error)` early
3. Error types are automatically converted using `From` trait

```rust
// This function
fn example() -> Result<i32, MyError> {
    let value = risky_operation()?;
    Ok(value * 2)
}

// Is equivalent to:
fn example() -> Result<i32, MyError> {
    let value = match risky_operation() {
        Ok(v) => v,
        Err(e) => return Err(e.into()),  // .into() for type conversion
    };
    Ok(value * 2)
}
```

### `?` with Option

```rust
fn get_first_last_char(s: &str) -> Option<(char, char)> {
    let first = s.chars().next()?;  // Early return if None
    let last = s.chars().last()?;   // Early return if None
    Some((first, last))
}
```

---

## Error Handling Patterns

### Matching on Results

```rust
use std::fs::File;

fn handle_file_operation() {
    match File::open("config.txt") {
        Ok(file) => {
            println!("File opened successfully!");
            // Use file...
        }
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => {
                println!("Config file not found, using defaults");
            }
            std::io::ErrorKind::PermissionDenied => {
                eprintln!("Permission denied when opening config file");
            }
            other => {
                eprintln!("Unexpected error: {:?}", other);
            }
        }
    }
}
```

### Converting Errors

```rust
#[derive(Debug)]
enum MyError {
    Io(std::io::Error),
    Parse(std::num::ParseIntError),
}

// Implement From trait for automatic conversion with ?
impl From<std::io::Error> for MyError {
    fn from(error: std::io::Error) -> Self {
        MyError::Io(error)
    }
}

impl From<std::num::ParseIntError> for MyError {
    fn from(error: std::num::ParseIntError) -> Self {
        MyError::Parse(error)
    }
}

// Now ? works seamlessly
fn read_and_parse_number() -> Result<i32, MyError> {
    let contents = std::fs::read_to_string("number.txt")?;  // io::Error -> MyError
    let number = contents.trim().parse()?;                 // ParseIntError -> MyError
    Ok(number)
}
```

### Custom Error Types

```rust
use std::fmt;

#[derive(Debug)]
struct ValidationError {
    field: String,
    message: String,
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.field, self.message)
    }
}

impl std::error::Error for ValidationError {}

fn validate_age(age: i32) -> Result<i32, ValidationError> {
    if age < 0 {
        Err(ValidationError {
            field: "age".to_string(),
            message: "cannot be negative".to_string(),
        })
    } else if age > 150 {
        Err(ValidationError {
            field: "age".to_string(),
            message: "unrealistically high".to_string(),
        })
    } else {
        Ok(age)
    }
}
```

---

## When to Use What

### Result vs Option vs Panic

| Situation | Use | Example |
|-----------|-----|----------|
| Value might not exist | [[option#Option]] | Database lookup, array indexing |
| Operation might fail | `Result<T,E>` | File I/O, network requests, parsing |
| Programming error | `panic!` | Array out of bounds, assertion failures |
| Unrecoverable error | `panic!` | Out of memory, corrupted data |

### Best Practices

```rust
// ✅ Good: Propagate errors up the call stack
fn process_config() -> Result<Config, ConfigError> {
    let raw_config = read_config_file()?;
    let parsed_config = parse_config(&raw_config)?;
    validate_config(&parsed_config)?;
    Ok(parsed_config)
}

// ✅ Good: Handle errors at the appropriate level
fn main() {
    match process_config() {
        Ok(config) => run_application(config),
        Err(error) => {
            eprintln!("Failed to load config: {}", error);
            std::process::exit(1);
        }
    }
}

// ❌ Avoid: Using unwrap() without good reason
let config = read_config_file().unwrap();  // Will panic on any error

// ✅ Better: Use expect() with descriptive message
let config = read_config_file()
    .expect("Config file is required for application to run");
```

---

## Integration with Other Concepts

### Result and Collections

```rust
// Collecting Results - stops on first error
let numbers: Result<Vec<i32>, _> = vec!["1", "2", "not_a_number", "4"]
    .into_iter()
    .map(|s| s.parse::<i32>())
    .collect();

// Partition successes and failures
let (successes, failures): (Vec<_>, Vec<_>) = vec!["1", "2", "not_a_number", "4"]
    .into_iter()
    .map(|s| s.parse::<i32>())
    .partition(Result::is_ok);
```

### Result and Generics

```rust
// Generic error handling
fn try_parse<T, E>(input: &str) -> Result<T, E>
where
    T: std::str::FromStr<Err = E>,
{
    input.parse()
}
```

---

## See Also
- [[enums]] - Foundation of the `Result<T,E>` enum
- [[option]] - For values that might not exist (null safety)
- [[traits]] - `Result<T,E>` implements many useful traits
- [[built-ins]] - Standard traits like `From`, `Into`, `Debug`, `Display` used in error handling
- [[control]] - Pattern matching for error handling
- [[generics]] - Result is a generic enum

**Practice**: `exercises/13_error_handling/` | **Review**: [[rust-review-guide#Advanced Features Phase]]
