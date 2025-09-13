## Common Lifetime Patterns

### The Lifetime Diamond Problem

```rust
// This is tricky - what lifetime should the return have?
fn choose_str<'a, 'b>(x: &'a str, y: &'b str, choose_first: bool) -> &??? str {
    if choose_first { x } else { y }
}

// Solution: Both inputs must have the same lifetime
fn choose_str<'a>(x: &'a str, y: &'a str, choose_first: bool) -> &'a str {
    if choose_first { x } else { y }
}
```

### Self-Referential Structs (Advanced)

```rust
// This doesn't work - can't borrow from self
// struct SelfRef<'a> {
//     data: String,
//     reference: &'a str,  // Can't refer to self.data
// }

// Solutions: Pin, Rc/RefCell, or external libraries like ouroboros
```

### Working with Closures

```rust
// Closures capture references with their own lifetimes
fn create_closure<'a>(s: &'a str) -> impl Fn() -> &'a str {
    move || s  // Closure must not outlive 's'
}
```

---

## Troubleshooting Lifetime Errors

### Common Error: "Borrowed value does not live long enough"

```rust
// Problem
fn get_string_ref() -> &str {
    let s = String::from("hello");
    &s  // ERROR: s dropped here
}

// Solutions:
// 1. Return owned data
fn get_string() -> String {
    String::from("hello")
}

// 2. Take input reference
fn get_prefix(s: &str) -> &str {
    &s[0..5]
}

// 3. Use 'static data
fn get_static() -> &'static str {
    "hello"  // String literal
}
```

### Common Error: "Cannot infer appropriate lifetime"

```rust
// Problem - ambiguous lifetime
// fn ambiguous(x: &str, y: &str) -> &str { x }

// Solution - be explicit
fn explicit<'a, 'b>(x: &'a str, _y: &'b str) -> &'a str { x }
```

---