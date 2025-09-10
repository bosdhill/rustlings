# Lifetimes

#advanced-features #lifetimes #memory-safety #references

> Advanced memory safety - builds on [[ownership]] and works with [[generics]] and [[traits]]

**Related Topics**: [[ownership]] | [[generics]] | [[traits]] | [[smart-pointers]]

## Overview

Lifetimes are Rust's way of ensuring that references are valid for as long as needed. They tell the compiler to explicitly keep _references valid_ and prevent dangling references.

**Key insight**: Lifetime parameters signify particular lifetimes of values that are borrowed.

> 💡 **Important**: Lifetime parameters are _inferred_ from the calling scope by the compiler. The compiler looks at the actual references you're passing in, determines their concrete lifetimes, and substitutes those for the generic lifetime parameters.

Lifetimes are a compile-time construct used to ensure all borrows are valid - they have **zero runtime cost**.

### Why Lifetimes Matter

```rust
// This would be unsafe - returning a reference to local data
fn get_string() -> &str {
    let s = String::from("hello");
    &s  // ERROR: `s` dropped here while still borrowed
}  // `s` goes out of scope

// Safe version - caller owns the data
fn get_prefix(text: &str) -> &str {
    &text[0..5]  // OK: reference has same lifetime as input
}
```

---

## Lifetime vs Scope

Your original example perfectly illustrates this distinction:

A lifetime of a value is not the same as its scope:

```rust
// Lifetimes are annotated below with lines denoting the creation
// and destruction of each variable.
// `i` has the longest lifetime because its scope entirely encloses
// both `borrow1` and `borrow2`. The duration of `borrow1` compared
// to `borrow2` is irrelevant since they are disjoint.
fn main() {
    let i = 3; // Lifetime for `i` starts. ────────────────┐
    //                                                     │
    { //                                                   │
        let borrow1 = &i; // `borrow1` lifetime starts. ──┐│
        //                                                ││
        println!("borrow1: {}", borrow1); //              ││
    } // `borrow1` ends. ─────────────────────────────────┘│
    //                                                     │
    //                                                     │
    { //                                                   │
        let borrow2 = &i; // `borrow2` lifetime starts. ──┐│
        //                                                ││
        println!("borrow2: {}", borrow2); //              ││
    } // `borrow2` ends. ─────────────────────────────────┘│
    //                                                     │
}   // Lifetime ends. ─────────────────────────────────────┘
```

**Key Point**: A lifetime is not the same as scope. The lifetime of `i` encompasses both borrows, even though they don't overlap.

---

## Lifetime Annotation Syntax

Lifetime annotations describe the relationships between lifetimes but don't change how long references live.

### Basic Syntax

```rust
&'a T      // Reference to T with lifetime 'a
&'a mut T  // Mutable reference to T with lifetime 'a
```

### In Function Signatures

```rust
foo<'a>           // Function with one lifetime parameter
foo<'a, 'b>       // Function with two lifetime parameters
```

### Function Parameters with Lifetimes

```rust
// Two references with potentially different lifetimes
fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("x is {} and y is {}", x, y);
}

// When lifetimes are the same, you can use one parameter
fn print_same<'a>(x: &'a i32, y: &'a i32) {
    println!("x is {} and y is {}", x, y);
}
```

### Why This Fails

Your analysis of why this fails is exactly right!

```rust
// This function declares lifetime 'a but has no input references
fn failed_borrow<'a>() {
    let _x = 12;  // Local variable

    // ERROR: `_x` does not live long enough
    let _y: &'a i32 = &_x;
    // Problem: 'a could be any lifetime, potentially longer than this function!
}
```

**Why it fails**:
- The function promises to work with lifetime `'a`
- `'a` could be longer than the function's execution
- `_x` dies when the function ends
- We can't return a reference to dead data

**Key insight**: Lifetime parameters must be connected to input references or be `'static`.

---

## Lifetime Rules in Functions

### The Rules (Your Original Notes Are Correct!)

1. **Input references** must be annotated with lifetimes
2. **Output references** must have the same lifetime as an input reference OR be `'static`
3. **No dangling references** - can't return references to local data

### Invalid Examples

```rust
// ERROR: Returning reference to local data
fn invalid_output<'a>() -> &'a String {
    &String::from("foo")  // String dies at end of function
}

// ERROR: No connection between input and output lifetimes
fn disconnected<'a, 'b>(x: &'a str) -> &'b str {
    "hello"  // Where does 'b come from?
}
```

### Valid Examples

```rust
// Simple case: input and output have same lifetime
fn first_word<'a>(s: &'a str) -> &'a str {
    s.split_whitespace().next().unwrap_or("")
}

// Multiple inputs, output connected to one
fn longer<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// Different lifetimes for different parameters
fn print_and_return<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    println!("Printing: {}", y);  // y can have shorter lifetime
    x  // Return value must match 'a
}

// Mutable references work the same way
fn modify_and_return<'a>(x: &'a mut i32) -> &'a mut i32 {
    *x += 1;
    x
}
```

### Lifetime Elision Rules

Rust can often infer lifetimes, so you don't always need to write them:

```rust
// These are equivalent:
fn first_word(s: &str) -> &str { /* ... */ }           // Elided
fn first_word<'a>(s: &'a str) -> &'a str { /* ... */ } // Explicit

// Rule 1: Each input reference gets its own lifetime
// Rule 2: If exactly one input lifetime, output gets same lifetime
// Rule 3: If multiple inputs and one is &self or &mut self, output gets self's lifetime
```

---

## Lifetimes in Structs

### Storing References in Structs

When structs contain references, you must specify lifetimes:

```rust
// Struct that borrows data
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'.");

    let i = ImportantExcerpt {
        part: first_sentence,
    };
    // `i` can't outlive `novel` because it borrows from it
}
```

### Methods on Structs with Lifetimes

```rust
struct Owner(i32);

impl Owner {
    // Usually lifetime elision handles these
    fn get_value(&self) -> i32 { self.0 }

    // Explicit lifetimes when needed
    fn add_one<'a>(&'a mut self) { self.0 += 1; }
    fn print<'a>(&'a self) {
        println!("`print`: {}", self.0);
    }
}

// Methods on generic lifetime structs
impl<'a> ImportantExcerpt<'a> {
    // Lifetime elision: input and output lifetimes match
    fn announce_and_return_part(&self) -> &str {
        println!("Attention please: {}", self.part);
        self.part
    }
}
```

### Multiple References in Structs

```rust
// Single lifetime - all references must live equally long
#[derive(Debug)]
struct Borrowed<'a>(&'a i32);

#[derive(Debug)]
struct NamedBorrowed<'a> {
    x: &'a i32,
    y: &'a i32,  // Same lifetime as x
}

// Multiple lifetimes - references can have different lifetimes
#[derive(Debug)]
struct TwoRefs<'a, 'b> {
    x: &'a i32,
    y: &'b i32,  // Can outlive or be outlived by x
}

// Enums with lifetimes
#[derive(Debug)]
enum Either<'a> {
    Num(i32),      // Owned data - no lifetime needed
    Ref(&'a i32),  // Borrowed data - needs lifetime
}

fn main() {
    let x = 18;
    let y = 15;

    let single = Borrowed(&x);
    let double = NamedBorrowed { x: &x, y: &y };
    let reference = Either::Ref(&x);
    let number = Either::Num(y);  // y is copied, not borrowed

    println!("x is borrowed in {:?}", single);
    println!("x and y are borrowed in {:?}", double);
    println!("x is borrowed in {:?}", reference);
    println!("y is *not* borrowed in {:?}", number);
}
```

---

## Lifetimes in Traits

Annotation of lifetimes in trait methods basically are similar to functions. Note that impl may have annotation of lifetimes too.

```rust
// A struct with annotation of lifetimes.
#[derive(Debug)]
struct Borrowed<'a> {
    x: &'a i32,
}

// Annotate lifetimes to impl.
// Remember: Default is to implement default values for a struct.
impl<'a> Default for Borrowed<'a> {
    fn default() -> Self {
        Self {
            x: &10,
        }
    }
}

fn main() {
    let b: Borrowed = Default::default();
    println!("b is {:?}", b);
}
```

## Bounded Lifetimes

Similar to generics, lifetimes can be bounded with `:`:

1. `T: 'a`: All references in `T` must outlive lifetime `'a`. (For example lifetime of fields in a struct / enum type).
2. `T: Trait + 'a`: Type `T` must implement trait `Trait` and all
   references in `T` must outlive `'a`.

```rust
use std::fmt::Debug; // Trait to bound with.

#[derive(Debug)]
struct Ref<'a, T: 'a>(&'a T);
// `Ref` contains a reference to a generic type `T` that has
// some lifetime `'a` unknown by `Ref`. `T` is bounded such that any
// *references* in `T` must outlive `'a`. Additionally, the lifetime
// of `Ref` may not exceed `'a`.

// A generic function which prints using the `Debug` trait.
fn print<T>(t: T) where
    T: Debug {
    println!("`print`: t is {:?}", t);
}

// Here a reference to `T` is taken where `T` implements
// `Debug` and all *references* in `T` outlive `'a`. In
// addition, `'a` must outlive the function.
fn print_ref<'a, T>(t: &'a T) where
    T: Debug + 'a {
    println!("`print_ref`: t is {:?}", t);
}

fn main() {
    let x = 7;
    let ref_x = Ref(&x);

    print_ref(&ref_x);
    print(ref_x);
}
```

## Coercing Longer Lifetimes to Shorter Ones

The compiler will automatically pick the shorter of the two lifetimes,
and effectively coerces the longer lifetime to the shorter one:

```rust
// Here, Rust infers a lifetime that is as short as possible.
// The two references are then coerced to that lifetime.
fn multiply<'a>(first: &'a i32, second: &'a i32) -> i32 {
    first * second
}

// `<'a: 'b, 'b>` reads as lifetime `'a` is at least as long as `'b`.
// Here, we take in an `&'a i32` and return a `&'b i32` as a result of coercion.
fn choose_first<'a: 'b, 'b>(first: &'a i32, _: &'b i32) -> &'b i32 {
    first
}

fn main() {
    let first = 2; // Longer lifetime

    {
        let second = 3; // Shorter lifetime

        println!("The product is {}", multiply(&first, &second));
        println!("{} is the first", choose_first(&first, &second));
    };
}
```

will output:

```
The product is 6
2 is the first
```

## Static Lifetimes (lifetime which is remainder of the program)

Static lifetimes are used to represent data pointed to by the reference
will exist for the lifetime of the program, and may be coerced into
shorter lifetimes.

The data is _read-only_, and a variable can be made static by:

1. Making a constant with the `static` declaration.
2. Making a string literal which has type: `&'static str`.

```rust
// Make a constant with `'static` lifetime.
static NUM: i32 = 18;

// Returns a reference to `NUM` where its `'static`
// lifetime is coerced to that of the input argument.
fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
    &NUM
}

fn main() {
    {
        // Make a `string` literal and print it:
        let static_string = "I'm in read-only memory";
        println!("static_string: {}", static_string);

        // When `static_string` goes out of scope, the reference
        // can no longer be used, but the data remains in the binary.
    }

    {
        // Make an integer to use for `coerce_static`:
        let lifetime_num = 9;

        // Coerce `NUM` to lifetime of `lifetime_num`:
        let coerced_static = coerce_static(&lifetime_num);

        println!("coerced_static: {}", coerced_static);
    }

    println!("NUM: {} stays accessible!", NUM);
}
```

Since `'static` references only need to be valid for the _remainder of a program's life_, they can be created while the program is executed. Just to demonstrate, the below example uses `Box::leak` to dynamically create `'static` references. In that case it definitely doesn't live for the entire duration, but only from the leaking point onward.

```rust
extern crate rand;
use rand::Fill;

fn random_vec() -> &'static [usize; 100] {
    let mut rng = rand::thread_rng();
    let mut boxed = Box::new([0; 100]);
    boxed.try_fill(&mut rng).unwrap();
    Box::leak(boxed)
}

fn main() {
    let first: &'static [usize; 100] = random_vec();
    let second: &'static [usize; 100] = random_vec();
    assert_ne!(first, second)
}
```

### Higher-Ranked Trait Bounds (HRTB)

```rust
// Function that works with any lifetime
fn apply_to_string<F>(f: F) -> String
where
    F: for<'a> Fn(&'a str) -> &'a str,  // "for any lifetime 'a"
{
    f("hello world").to_string()
}
```

### Lifetime Bounds

```rust
// T must outlive 'a
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display + 'a,  // T must live at least as long as 'a
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() { x } else { y }
}
```

---

## The `'static` Lifetime

Your examples demonstrate the `'static` lifetime perfectly. Let me expand on this:

### Static References vs Static Bounds

```rust
// 1. 'static reference - points to data that lives for entire program
let s: &'static str = "Hello, world!";  // String literal

// 2. 'static bound - type contains no non-static references
use std::fmt::Debug;

fn print_it(input: impl Debug + 'static) {
    println!("'static value passed in is: {:?}", input);
}

fn main() {
    // Owned data satisfies 'static bound (no internal references)
    let i = 5;
    print_it(i);  // Works - i32 is 'static

    // References don't satisfy 'static bound unless they're 'static
    print_it(&i); // ERROR - &i is not 'static

    // But string literals are 'static references
    print_it("hello"); // Works - &'static str
}
```

### When to Use 'static

```rust
// Spawning threads - data must be 'static
use std::thread;

fn spawn_thread() {
    let data = vec![1, 2, 3];

    // This won't work - data doesn't live long enough
    // thread::spawn(move || println!("{:?}", &data));

    // This works - move ownership into the thread
    thread::spawn(move || println!("{:?}", data));
}
```

---

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

## See Also

- [[ownership]] - Foundation for understanding lifetimes and borrowing
- [[generics]] - Lifetime parameters work similarly to generic type parameters
- [[traits]] - Lifetime bounds in trait definitions and implementations
- [[smart-pointers]] - Alternative to complex lifetime scenarios (`Rc`, `Arc`, `Box`)

**Practice**: `exercises/16_lifetimes/` | **Review**: [[rust-review-guide#Advanced Features Phase]]
