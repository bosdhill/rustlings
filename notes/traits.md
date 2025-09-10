# Traits

#advanced-features #traits #interfaces #generics

> Rust's interface system - essential for [[generics]] and enables polymorphism

**Related Topics**: [[generics]] | [[lifetimes]] | [[ownership]] | [[errors]]

Traits ~ Interfaces

## Implementing Trait on a Type

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}
```

Note that the implemented method type signature can be changed when it
comes to borrowing semantics, and making self mutable or not.

## Calling Trait Methods

Users of the crate can call the trait methods on instances of NewsArticle
and Tweet in the same way we call regular methods.

The only difference is that the user must bring the trait into scope
as well as the types.

```rust
use aggregator::{Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}
```

## Default Trait Implementations

Types can override default implementations, which are good to have
if we just want a type to have a trait without implementing its
methods, and don't have to modify any types that implement the trait.

```rust
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}
```

Default implementations can call other methods in the same trait, even
if those other methods don’t have a default implementation.

In this way, a trait can provide a lot of useful functionality and
only require implementors to specify a small part of it.

```rust
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}
```

Implementors would only be required to implement summarize_author:

```rust
impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}
```

Note that a trait can't have _non-overridable_ methods.

## Traits as Parameters

Accept any types that implement a trait:

```rust
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```

this is syntactic sugar for _Trait Bounds_ which look like:

```rust
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```

## Generic Bound Syntax

The impl Trait syntax is convenient and makes for more concise code in
simple cases, while the fuller trait bound syntax can express more
complexity in other cases, for ex:

```rust
// Using impl Trait is appropriate if we want this function to allow
// item1 and item2 to have different types (as long as both types
// implement Summary).
pub fn notify(item1: &impl Summary, item2: &impl Summary) {

// If we want to force both parameters to have the same type, however,
// we must use a trait bound, like this:
pub fn notify<T: Summary>(item1: &T, item2: &T) {
```

So basically one allows for different type implementors while the
other allows for only a single type implementor.

## Multiple Trait Bounds with `+`

We can also specify more than one trait bound:

```rust
pub fn notify(item: &(impl Summary + Display)) {
```

The + syntax is also valid with trait bounds on generic types:

```rust
pub fn notify<T: Summary + Display>(item: &T) {
```

With the two trait bounds specified, the body of notify can call summarize and use {} to format item.

Rust has alternate syntax for specifying trait bounds inside a where clause after the function signature. So, instead of writing this:

```rust
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
```

we can use a where clause, like this:

```rust
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
```

## Returning Types That Implement Traits

We can also use the impl Trait syntax in the return position to return a value of some type that implements a trait, as shown here:

```rust
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}
```

---

## Advanced Trait Concepts

Beyond basic trait definitions and implementations, Rust's trait system enables powerful patterns like dynamic dispatch, associated types, and complex trait bounds. These advanced concepts allow for flexible, reusable code while maintaining Rust's performance guarantees.

### Trait Objects and Dynamic Dispatch

```rust
trait Draw {
    fn draw(&self);
}

struct Circle;
struct Square;

impl Draw for Circle {
    fn draw(&self) {
        println!("Drawing a circle");
    }
}

impl Draw for Square {
    fn draw(&self) {
        println!("Drawing a square");
    }
}

// Static dispatch - compiler knows exact type
fn draw_static<T: Draw>(shape: T) {
    shape.draw();
}

// Box<dyn Trait> explanation:
// Box allocates data on the heap and gives us a pointer to it
// dyn Draw means "any type that implements Draw"
// Box<dyn Draw> is a "trait object" - a fat pointer containing:
//   - pointer to the data
//   - pointer to the vtable (method dispatch table)

// Dynamic dispatch - type determined at runtime
fn draw_dynamic(shapes: Vec<Box<dyn Draw>>) {
    for shape in shapes {
        shape.draw();
    }
}

fn main() {
    let shapes: Vec<Box<dyn Draw>> = vec![
        Box::new(Circle),
        Box::new(Square),
    ];
    draw_dynamic(shapes);
}
```

### Associated Types

```rust
trait Iterator {
    type Item;  // Associated type

    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter {
    current: i32,
}

impl Iterator for Counter {
    type Item = i32;  // Concrete associated type

    fn next(&mut self) -> Option<Self::Item> {
        self.current += 1;
        Some(self.current)
    }
}
```

### Supertraits

```rust
// Display is a supertrait of Summary
trait Summary: std::fmt::Display {
    fn summarize(&self) -> String {
        format!("Summary: {}", self)  // Can use Display methods
    }
}

struct Article {
    title: String,
    content: String,
}

// Must implement Display first
impl std::fmt::Display for Article {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.title)
    }
}

// Then can implement Summary
impl Summary for Article {}
```

### Blanket Implementations

```rust
// Implement for all types that already implement Display
impl<T: std::fmt::Display> ToString for T {
    fn to_string(&self) -> String {
        format!("{}", self)
    }
}

// Now any type implementing Display automatically gets ToString
let number = 42;
let string = number.to_string();  // Works because i32 implements Display
```

---

## See Also
- [[generics]] - Traits enable generic programming with type bounds
- [[lifetimes]] - Lifetime bounds on traits and trait objects
- [[ownership]] - Ownership semantics in trait methods (`&self`, `&mut self`, `self`)
- [[enums]] - Implementing traits for enums and pattern matching
- [[built-ins]] - Standard library traits like `From`, `Into`, `Display`, `Debug`
- [[errors]] - Error handling with trait-based conversions
