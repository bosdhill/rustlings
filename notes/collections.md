# Collections

#core-concepts #data-structures #vectors #strings #hashmaps

> Essential data structures - uses [[ownership]], [[option]], and [[generics]] patterns

**Related Topics**: [[ownership]] | [[option]] | [[generics]] | [[lifetimes]]

## Overview

Rust's standard library includes three important collection types that store multiple values:
- **Vec<T>** (vectors) - growable arrays
- **String** - growable text
- **HashMap<K,V>** - key-value pairs

All collections are stored on the heap and can grow or shrink at runtime.

---

## Vectors

Vectors store values of the same type in a contiguous block of memory.

### Creating Vectors

```rust
// Empty vector with type annotation
let mut v: Vec<i32> = Vec::new();

// Using the vec! macro with initial values
let mut v = vec![1, 2, 3, 4, 5];

// Create with capacity (optimization)
let mut v = Vec::with_capacity(10);
```

### Adding Elements

```rust
let mut v = Vec::new();

v.push(5);      // Add to end
v.push(6);
v.push(7);

// Using vec! macro
let v2 = vec![1, 2, 3];
```

### Accessing Elements

```rust
let v = vec![1, 2, 3, 4, 5];

// Method 1: Direct indexing (will panic if out of bounds)
let third: &i32 = &v[2];
println!("The third element is {}", third);

// Method 2: Using get() - returns Option<&T>
match v.get(2) {
    Some(third) => println!("The third element is {}", third),
    None => println!("There is no third element."),
}

// Safe access with bounds checking
if let Some(element) = v.get(10) {
    println!("Element exists: {}", element);
} else {
    println!("Index out of bounds!");
}
```

> 💡 Use `get()` for safe access - it returns [[option#Option]] instead of panicking

### Iterating Over Vectors

```rust
let v = vec![100, 32, 57];

// Iterate over references (immutable)
for i in &v {
    println!("{}", i);
}

// Iterate over mutable references
let mut v = vec![100, 32, 57];
for i in &mut v {
    *i += 50;  // Dereference to modify the value
}

// Iterate and take ownership
for i in v {
    println!("{}", i);
    // v is no longer accessible after this loop
}
```

### Vector Ownership Rules

```rust
let mut v = vec![1, 2, 3, 4, 5];

let first = &v[0];        // Immutable borrow
// v.push(6);             // ERROR: can't mutate while borrowed
println!("The first element is: {}", first);
v.push(6);                // OK: immutable borrow scope ended
```

> ⚠️ Cannot have mutable and immutable references simultaneously

### Storing Different Types with Enums

```rust
// Use enums to store different types in a vector
#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
];

for cell in &row {
    match cell {
        SpreadsheetCell::Int(i) => println!("Integer: {}", i),
        SpreadsheetCell::Float(f) => println!("Float: {}", f),
        SpreadsheetCell::Text(s) => println!("Text: {}", s),
    }
}
```

---

## Strings

Strings are UTF-8 encoded text. Rust has two main string types:
- `String` - owned, growable string
- `&str` - string slice, usually borrowed

### String vs &str

```rust
// String literals are &str
let s1 = "hello";              // &str (string slice)
let s2 = String::from("hello"); // String (owned)
let s3 = "hello".to_string();   // String (owned)

// Converting between types
let s4: String = s1.to_string();  // &str -> String
let s5: &str = &s2;               // String -> &str
```

### Creating and Updating Strings

```rust
// Creating empty strings
let mut s = String::new();
let mut s = String::from("initial contents");

// Adding to strings
s.push_str("bar");           // Append string slice
s.push('!');                 // Append single character

// Using format! macro
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = format!("{}{}", s1, s2);  // s1 and s2 still valid

// Concatenation with +
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2;  // s1 moved here, s2 still valid
```

### String Indexing (Not Allowed!)

```rust
let s = String::from("hello");
// let h = s[0];  // ERROR: Strings don't support indexing

// Why? UTF-8 encoding means not all characters are 1 byte
let hello = String::from("Здравствуйте");  // Cyrillic
// First character (З) takes 2 bytes in UTF-8!
```

### String Slicing and Iteration

```rust
let hello = "Здравствуйте";

// Slicing (be careful with UTF-8 boundaries!)
let s = &hello[0..4];  // First 4 bytes (first 2 characters)

// Safe iteration over characters
for c in hello.chars() {
    println!("{}", c);
}

// Iteration over bytes
for b in hello.bytes() {
    println!("{}", b);
}

// Getting length
println!("Length in chars: {}", hello.chars().count());
println!("Length in bytes: {}", hello.len());
```

### Common String Methods

```rust
let mut s = String::from("  Hello, World!  ");

// Trimming
let trimmed = s.trim();

// Case conversion
let lower = s.to_lowercase();
let upper = s.to_uppercase();

// Checking contents
if s.contains("World") {
    println!("Found World!");
}

// Replacing
let replaced = s.replace("World", "Rust");

// Splitting
let words: Vec<&str> = s.trim().split_whitespace().collect();
```

---

## HashMaps

Hash maps store key-value pairs using a hash function for fast lookups.

### Creating HashMaps

```rust
use std::collections::HashMap;

// Empty HashMap
let mut scores = HashMap::new();

// Adding key-value pairs
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

// From vectors using collect()
let teams = vec![String::from("Blue"), String::from("Yellow")];
let initial_scores = vec![10, 50];
let scores: HashMap<_, _> = teams.into_iter()
    .zip(initial_scores.into_iter())
    .collect();
```

### Accessing Values

```rust
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);

// Using get() - returns Option<&V>
let team_name = String::from("Blue");
match scores.get(&team_name) {
    Some(score) => println!("Score: {}", score),
    None => println!("Team not found"),
}

// With copied() and unwrap_or()
let score = scores.get(&team_name).copied().unwrap_or(0);
```

> 💡 `get()` returns [[option#Option]] - use pattern matching or helper methods

### Iterating Over HashMaps

```rust
for (key, value) in &scores {
    println!("{}: {}", key, value);
}
```

### Updating Values

```rust
let mut scores = HashMap::new();

// Overwriting values
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Blue"), 25);  // Overwrites 10

// Only insert if key doesn't exist
scores.entry(String::from("Yellow")).or_insert(50);
scores.entry(String::from("Blue")).or_insert(50);  // Won't overwrite

// Update based on old value
let text = "hello world wonderful world";
let mut map = HashMap::new();

for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;  // Dereference to modify the value
}
println!("{:?}", map);  // {"hello": 1, "world": 2, "wonderful": 1}
```

### HashMap Ownership

```rust
use std::collections::HashMap;

let field_name = String::from("Favorite color");
let field_value = String::from("Blue");

let mut map = HashMap::new();
map.insert(field_name, field_value);
// field_name and field_value are moved and no longer valid

// For Copy types, values are copied
let mut map = HashMap::new();
let key = 1;
let value = 2;
map.insert(key, value);
// key and value are still valid (i32 implements Copy)

// Using references requires lifetimes
let mut map = HashMap::new();
let key = "color";
let value = "blue";
map.insert(key, value);  // &str references are copied
```

### Entry API Patterns

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert("Blue".to_string(), 10);

// Complex update logic with entry
let team = "Blue".to_string();
let entry = scores.entry(team);

match entry {
    std::collections::hash_map::Entry::Occupied(mut e) => {
        *e.get_mut() += 10;  // Add to existing score
    }
    std::collections::hash_map::Entry::Vacant(e) => {
        e.insert(5);  // Insert default score
    }
}

// Shorthand with or_insert_with
scores.entry("Red".to_string())
    .or_insert_with(|| expensive_calculation());
```

---

## Iterator Fundamentals

All collections work with Rust's powerful iterator system.

### Basic Iterator Pattern

```rust
let v = vec![1, 2, 3];

// iter() creates an iterator over references
let sum: i32 = v.iter().sum();

// into_iter() creates an iterator that takes ownership
let v2 = vec![1, 2, 3];
let sum: i32 = v2.into_iter().sum();  // v2 is consumed

// iter_mut() creates an iterator over mutable references
let mut v = vec![1, 2, 3];
for item in v.iter_mut() {
    *item *= 2;
}
```

### Map, Filter, Collect Pattern

```rust
let numbers = vec![1, 2, 3, 4, 5];

// Transform and collect
let doubled: Vec<i32> = numbers
    .iter()
    .map(|x| x * 2)
    .collect();

// Filter and collect
let evens: Vec<&i32> = numbers
    .iter()
    .filter(|&x| x % 2 == 0)
    .collect();

// Chain operations
let processed: Vec<String> = numbers
    .iter()
    .filter(|&x| x % 2 == 0)
    .map(|x| format!("Even: {}", x))
    .collect();
```

> 💡 This pattern is fundamental for functional programming in Rust

---

## Memory and Performance

### Collection Performance Characteristics

| Operation | Vec<T> | String | HashMap<K,V> |
|-----------|---------|---------|---------------|
| Access by index | O(1) | N/A | N/A |
| Access by key | O(n) | N/A | O(1) average |
| Insert at end | O(1) amortized | O(1) amortized | O(1) average |
| Insert at position | O(n) | O(n) | N/A |
| Remove | O(n) | O(n) | O(1) average |

### Memory Layout

```rust
// Vectors store elements contiguously
let v = vec![1, 2, 3, 4, 5];
// Memory: [1][2][3][4][5] (contiguous)

// HashMaps use bucket arrays + linked lists
let mut map = HashMap::new();
map.insert("key1", 1);
map.insert("key2", 2);
// Memory: Hash table with buckets containing key-value pairs
```

### Capacity vs Length

```rust
let mut v = Vec::with_capacity(10);
println!("Length: {}, Capacity: {}", v.len(), v.capacity()); // 0, 10

v.push(1);
println!("Length: {}, Capacity: {}", v.len(), v.capacity()); // 1, 10

// Avoid reallocations by pre-allocating capacity
let mut v = Vec::with_capacity(1000);  // Won't reallocate for first 1000 items
```

---

## Common Patterns and Best Practices

### Choosing the Right Collection

- **Vec<T>**: When you need indexed access or ordered data
- **String**: For text data, UTF-8 encoded
- **HashMap<K,V>**: For key-based lookups, unordered data
- **VecDeque<T>**: For double-ended queue operations
- **BTreeMap<K,V>**: For sorted key-value pairs

### Error Handling with Collections

```rust
use std::collections::HashMap;

fn get_score(scores: &HashMap<String, i32>, team: &str) -> Option<i32> {
    scores.get(team).copied()
}

fn get_score_or_default(scores: &HashMap<String, i32>, team: &str) -> i32 {
    scores.get(team).copied().unwrap_or(0)
}

// Using ? operator with collections
fn find_first_even(numbers: Vec<i32>) -> Option<i32> {
    numbers.into_iter().find(|&x| x % 2 == 0)
}
```

### Collection Initialization Patterns

```rust
use std::collections::HashMap;

// Vec initialization
let numbers = (1..=5).collect::<Vec<i32>>();
let words = vec!["hello", "world"];
let zeros = vec![0; 10];  // 10 zeros

// HashMap from iterator
let scores: HashMap<&str, i32> = [
    ("Alice", 100),
    ("Bob", 90),
].iter().cloned().collect();

// Or using macro (if you have the maplit crate)
// let scores = hashmap!{"Alice" => 100, "Bob" => 90};
```

---

## See Also

- [[ownership]] - Critical for understanding collection ownership patterns
- [[option]] - HashMaps return `Option<V>` for lookups, vectors use for bounds checking
- [[generics]] - All collections are generic over their element types
- [[lifetimes]] - Important when storing references in collections
- [[iterators]] - Powerful functional programming patterns (Lesson 18)

**Practice**: `exercises/05_vecs/`, `exercises/09_strings/`, `exercises/11_hashmaps/` | **Review**: [[rust-review-guide#Core Concepts Phase]]
