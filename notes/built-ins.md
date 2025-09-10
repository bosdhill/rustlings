# Built-in Traits & Standard Library

#built-ins #traits #std-library #derive #common-traits

> Essential traits provided by the standard library that power Rust's type system

## 🏗️ Overview

Rust provides a rich set of built-in traits that define common behaviors for types. These traits enable powerful features like automatic memory management, type conversions, formatting, and comparisons. Understanding these traits is crucial for writing idiomatic Rust code.

Most built-in traits can be automatically implemented using the `#[derive]` attribute, but you can also implement them manually when custom behavior is needed.

## 📋 Core Trait Categories

### Memory Management
- **[[built-ins#🧬 Clone|Clone]]** - Explicit copying of values
- **[[built-ins#📄 Copy|Copy]]** - Implicit copying for simple types
- **[[built-ins#🗑️ Drop|Drop]]** - Cleanup when values go out of scope

### Type Conversion
- **[[built-ins#🔄 From and Into|From and Into]]** - Type conversions
- **[[built-ins#🔗 AsRef and AsMut|AsRef and AsMut]]** - Reference conversions

### Formatting & Display
- **[[built-ins#Debug]]** - Developer-friendly formatting
- **[[built-ins#Display]]** - User-friendly formatting

### Comparison & Ordering
- **[[built-ins#⚖️ PartialEq and Eq|PartialEq and Eq]]** - Equality comparisons
- **[[built-ins#📊 PartialOrd and Ord|PartialOrd]]** - Ordering comparisons
- **[[built-ins#🔢 Hash|Hash]]** - Hashing for collections

### Defaults & Initialization
- **[[built-ins#🏁 Default|Default]]** - Default value construction

## 🧬 Clone

**Purpose**: Provides explicit, potentially expensive copying of values.

```rust
#[derive(Clone)]
struct Person {
    name: String,
    age: u32,
}

let person1 = Person {
    name: String::from("Alice"),
    age: 30
};
let person2 = person1.clone(); // Explicit copy
// person1 is still valid after cloning
```

### When to Use Clone
- When you need to duplicate owned data (like `String`, `Vec`)
- When `Copy` is too restrictive (heap-allocated data)
- When you want explicit control over when copying happens

### Manual Implementation
```rust
struct CustomData {
    values: Vec<i32>,
    metadata: String,
}

impl Clone for CustomData {
    fn clone(&self) -> Self {
        Self {
            values: self.values.clone(),
            metadata: self.metadata.clone(),
        }
    }
}
```

## 📄 Copy

**Purpose**: Enables implicit copying for simple, stack-only data.

```rust
#[derive(Copy, Clone)]  // Copy requires Clone
struct Point {
    x: i32,
    y: i32,
}

let p1 = Point { x: 1, y: 2 };
let p2 = p1; // Implicit copy - p1 still valid
println!("p1: {:?}, p2: {:?}", p1, p2);
```

### Copy Rules
- Only stack-only data can implement `Copy`
- No heap allocations (`String`, `Vec`, etc.)
- All fields must also implement `Copy`
- Cannot implement both `Copy` and `Drop`

### Built-in Copy Types
```rust
// Primitive types implement Copy
let x: i32 = 5;
let y = x; // Copy, not move

// Tuples of Copy types are Copy
let tuple: (i32, f64, char) = (1, 2.0, 'c');
let tuple2 = tuple; // Copy

// Arrays of Copy types are Copy (if reasonably sized)
let arr: [i32; 5] = [1, 2, 3, 4, 5];
let arr2 = arr; // Copy
```

## 🗑️ Drop

**Purpose**: Custom cleanup when values go out of scope.

```rust
struct FileHandler {
    filename: String,
}

impl Drop for FileHandler {
    fn drop(&mut self) {
        println!("Closing file: {}", self.filename);
        // Custom cleanup logic here
    }
}

{
    let handler = FileHandler {
        filename: String::from("data.txt")
    };
    // ... use handler
} // drop() called automatically here
```

### Drop Guidelines
- Usually not needed - Rust handles cleanup automatically
- Use for custom resources (file handles, network connections)
- Cannot call `drop()` manually - use `std::mem::drop()` instead
- Cannot implement both `Copy` and `Drop`

## 🔄 From and Into

**Purpose**: Fallible and infallible type conversions.

### From Trait
```rust
// String from &str
let s: String = String::from("hello");

// Custom conversion
struct Person {
    name: String,
}

impl From<&str> for Person {
    fn from(name: &str) -> Self {
        Person {
            name: name.to_string()
        }
    }
}

let person = Person::from("Alice");
```

### Into Trait (Automatic)
```rust
// Into is automatically implemented when From exists
let person: Person = "Bob".into(); // Calls From::from internally

// Function that accepts anything convertible to String
fn greet<T: Into<String>>(name: T) {
    let name_string = name.into();
    println!("Hello, {}!", name_string);
}

greet("Alice");           // &str -> String
greet(String::from("Bob")); // String -> String (identity conversion)
```

### Error Handling with TryFrom/TryInto
```rust
use std::convert::TryFrom;

struct Age(u8);

impl TryFrom<i32> for Age {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value >= 0 && value <= 255 {
            Ok(Age(value as u8))
        } else {
            Err("Age must be between 0 and 255")
        }
    }
}

// Usage
let age = Age::try_from(25)?; // Ok(Age(25))
let invalid = Age::try_from(-5); // Err("Age must be between 0 and 255")
```

## 🔗 AsRef and AsMut

**Purpose**: Cheap reference conversions.

```rust
// AsRef allows functions to accept different string types
fn print_it<T: AsRef<str>>(input: T) {
    println!("{}", input.as_ref());
}

print_it("hello");              // &str
print_it(String::from("world")); // String
print_it(&String::from("test")); // &String

// AsMut for mutable references
fn make_uppercase<T: AsMut<str>>(mut input: T) {
    input.as_mut().make_ascii_uppercase();
}

let mut s = String::from("hello");
make_uppercase(&mut s);
println!("{}", s); // "HELLO"
```

### Common AsRef Implementations
```rust
// Path handling
use std::path::Path;

fn process_file<P: AsRef<Path>>(path: P) {
    let path = path.as_ref();
    println!("Processing: {}", path.display());
}

process_file("file.txt");           // &str
process_file(Path::new("file.txt")); // &Path
process_file(std::path::PathBuf::from("file.txt")); // PathBuf
```

## 🖨️ Debug

**Purpose**: Developer-friendly formatting with `{:?}`.

```rust
#[derive(Debug)]
struct User {
    id: u64,
    name: String,
    active: bool,
}

let user = User {
    id: 1,
    name: String::from("Alice"),
    active: true,
};

println!("{:?}", user);   // User { id: 1, name: "Alice", active: true }
println!("{:#?}", user);  // Pretty-printed format
```

### Manual Debug Implementation
```rust
use std::fmt;

struct Person {
    first: String,
    last: String,
    age: u32,
}

impl fmt::Debug for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Person")
            .field("name", &format!("{} {}", self.first, self.last))
            .field("age", &self.age)
            .finish()
    }
}
```

## 🎭 Display

**Purpose**: User-friendly formatting with `{}`.

```rust
use std::fmt;

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

let p = Point { x: 1, y: 2 };
println!("{}", p); // (1, 2)

// Display enables ToString automatically
let s = p.to_string(); // "(1, 2)"
```

### Display vs Debug
```rust
#[derive(Debug)]
struct Temperature(f64);

impl fmt::Display for Temperature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.1}°C", self.0)
    }
}

let temp = Temperature(23.456);
println!("{}", temp);    // 23.5°C (user-friendly)
println!("{:?}", temp);  // Temperature(23.456) (debug info)
```

## ⚖️ PartialEq and Eq

**Purpose**: Equality comparison with `==` and `!=`.

```rust
#[derive(PartialEq)]
struct Person {
    name: String,
    age: u32,
}

let p1 = Person { name: "Alice".to_string(), age: 30 };
let p2 = Person { name: "Alice".to_string(), age: 30 };
println!("{}", p1 == p2); // true
```

### PartialEq vs Eq
- **PartialEq**: General equality, may not be reflexive (like `f64::NAN`)
- **Eq**: Strict equality, must be reflexive, symmetric, and transitive

```rust
// f64 implements PartialEq but not Eq because NaN != NaN
assert!(f64::NAN != f64::NAN);

#[derive(PartialEq, Eq)] // Eq requires PartialEq
struct IntWrapper(i32); // i32 implements Eq, so this can too
```

### Custom Equality Logic
```rust
struct User {
    id: u64,
    name: String,
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id // Only compare IDs
    }
}

impl Eq for User {} // Assert that our equality is reflexive
```

## 📊 PartialOrd and Ord

**Purpose**: Ordering comparisons with `<`, `>`, `<=`, `>=`.

```rust
#[derive(PartialEq, PartialOrd)]
struct Height(f64);

let h1 = Height(5.8);
let h2 = Height(6.0);
println!("{}", h1 < h2); // true

// Ord enables sorting
#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Priority(u32);

let mut priorities = vec![Priority(3), Priority(1), Priority(2)];
priorities.sort();
println!("{:?}", priorities); // [Priority(1), Priority(2), Priority(3)]
```

### Manual Ordering Implementation
```rust
use std::cmp::Ordering;

#[derive(Eq, PartialEq)]
struct Person {
    name: String,
    age: u32,
}

impl Ord for Person {
    fn cmp(&self, other: &Self) -> Ordering {
        // Sort by age first, then by name
        self.age.cmp(&other.age)
            .then_with(|| self.name.cmp(&other.name))
    }
}

impl PartialOrd for Person {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
```

## 🔢 Hash

**Purpose**: Enables use as keys in `HashMap` and `HashSet`.

```rust
use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq)]
struct UserId(u64);

let mut users = HashMap::new();
users.insert(UserId(1), "Alice");
users.insert(UserId(2), "Bob");
```

### Manual Hash Implementation
```rust
use std::hash::{Hash, Hasher};

struct Person {
    id: u64,
    name: String,
}

impl Hash for Person {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Only hash the ID for consistent behavior
        self.id.hash(state);
    }
}

impl PartialEq for Person {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Person {}
```

## 🏁 Default

**Purpose**: Provides sensible default values.

```rust
#[derive(Default)]
struct Config {
    debug: bool,    // false
    port: u16,      // 0
    name: String,   // empty string
}

let config = Config::default();
println!("{:?}", config); // Config { debug: false, port: 0, name: "" }

// Using ..Default::default() for partial initialization
let custom_config = Config {
    port: 8080,
    ..Default::default()
};
```

### Manual Default Implementation
```rust
struct DatabaseConfig {
    host: String,
    port: u16,
    timeout: u64,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            host: String::from("localhost"),
            port: 5432,
            timeout: 30,
        }
    }
}

// Usage patterns
let db_config = DatabaseConfig::default();
let custom_db = DatabaseConfig {
    host: String::from("prod.db.com"),
    ..Default::default()
};
```

## 🔧 Derive Macro Patterns

### Common Derive Combinations
```rust
// Basic data types
#[derive(Debug, Clone, PartialEq, Eq)]
struct BasicData {
    id: u64,
    name: String,
}

// Comparable and sortable
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct SortableItem {
    priority: u32,
    name: String,
}

// Usable as HashMap keys
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct HashableKey {
    category: String,
    id: u64,
}

// Configuration struct
#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct AppConfig {
    enabled: bool,
    max_connections: u32,
    database_url: String,
}
```

### When NOT to Derive
```rust
// Don't derive Clone for expensive-to-clone types
struct LargeDataset {
    data: Vec<f64>, // Millions of elements
}

// Implement manually with optimization
impl Clone for LargeDataset {
    fn clone(&self) -> Self {
        // Maybe use Arc<> or implement copy-on-write
        println!("Warning: Cloning large dataset");
        Self { data: self.data.clone() }
    }
}

// Don't derive PartialEq when identity matters more than content
struct Database {
    connection_id: u64,
    data: HashMap<String, String>,
}

impl PartialEq for Database {
    fn eq(&self, other: &Self) -> bool {
        self.connection_id == other.connection_id
    }
}
```

## 🎯 Best Practices

### Trait Selection Guidelines
1. **Always derive `Debug`** - Essential for debugging
2. **Use `Clone` sparingly** - Only when you actually need copying
3. **Prefer `Copy` when possible** - More efficient than `Clone`
4. **Implement `Display` for user-facing types** - Better error messages
5. **Use `Default` for configuration** - Enables `..Default::default()`

### Performance Considerations
```rust
// Good: Efficient copying for small types
#[derive(Copy, Clone)]
struct Point2D { x: f64, y: f64 }

// Avoid: Expensive cloning
#[derive(Clone)] // Consider Arc<> instead
struct HugeDataset {
    matrix: Vec<Vec<f64>>, // Thousands x thousands
}

// Better: Reference counting for shared data
use std::sync::Arc;

#[derive(Clone)]
struct SharedData {
    dataset: Arc<HugeDataset>, // Cheap to clone
    metadata: String,          // Small, fine to clone
}
```

### Error Message Improvements
```rust
use std::fmt;

#[derive(Debug)]
struct ValidationError {
    field: String,
    message: String,
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Validation failed for '{}': {}", self.field, self.message)
    }
}

impl std::error::Error for ValidationError {}

// Now errors are much more readable
let error = ValidationError {
    field: "email".to_string(),
    message: "Invalid format".to_string(),
};

println!("{}", error); // "Validation failed for 'email': Invalid format"
```

## 🔗 Integration with Other Concepts

### With Generics
```rust
// Generic functions that require certain traits
fn print_and_clone<T: Debug + Clone>(item: T) -> T {
    println!("Item: {:?}", item);
    item.clone()
}

// Trait bounds in structs
#[derive(Debug)]
struct Container<T: Clone + Debug> {
    items: Vec<T>,
}

impl<T: Clone + Debug> Container<T> {
    fn add(&mut self, item: T) {
        println!("Adding: {:?}", item);
        self.items.push(item);
    }
}
```

### With Collections
```rust
use std::collections::{HashMap, HashSet};

// Types used as keys must implement Hash + Eq
#[derive(Hash, Eq, PartialEq, Debug)]
struct ProductId(String);

// Types in sorted collections need Ord
#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
struct Priority(u32);

let mut products: HashMap<ProductId, String> = HashMap::new();
products.insert(ProductId("laptop".to_string()), "Dell XPS".to_string());

let mut queue: std::collections::BTreeSet<Priority> = std::collections::BTreeSet::new();
queue.insert(Priority(1));
queue.insert(Priority(5));
```

### With Error Handling
```rust
use std::error::Error;
use std::fmt;

// Custom error types should implement Debug, Display, and Error
#[derive(Debug)]
struct ParseError {
    input: String,
    position: usize,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Parse error at position {} in '{}'", self.position, self.input)
    }
}

impl Error for ParseError {}

// From/Into for error conversion
impl From<std::num::ParseIntError> for ParseError {
    fn from(err: std::num::ParseIntError) -> Self {
        ParseError {
            input: format!("Parse int error: {}", err),
            position: 0,
        }
    }
}
```

## 📚 Learning Path

### Prerequisites
- ✅ Understand [[traits]] - Basic trait concepts
- ✅ Complete [[generics]] - Generic programming with trait bounds
- ✅ Familiarity with [[collections]] - HashMap/HashSet requirements

### Next Steps
1. Practice deriving common trait combinations
2. Implement manual trait implementations for custom behavior
3. Use traits as bounds in generic functions
4. Integrate with [[errors]] and [[collections]]
5. Explore advanced traits in [[iterators]]

---

*🏗️ Built-in traits are the foundation of idiomatic Rust - master them to write clean, efficient code!*
