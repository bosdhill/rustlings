# Lifetimes

#advanced-features #lifetimes #memory-safety #references

> Advanced memory safety - builds on [[ownership]] and works with [[generics]] and [[traits]]

**Related Topics**: [[ownership]] | [[generics]] | [[traits]] | [[smart-pointers]]

## Overview

Lifetimes are Rust's way of ensuring that references are valid for as long as needed.  They're annotations that help the borrow checker ensure **references don't outlive the data they point to**.

When compared to scopes, lifetimes are more like compile-time contracts about reference validity, while scopes determine actual object lifetime at runtime.

**Key insight**: Lifetime parameters signify particular lifetimes of values that are borrowed. 

> 💡 **Important**: Lifetime parameters are _inferred_ from the calling scope by the compiler. The compiler looks at the actual references you're passing in, determines their concrete lifetimes, and substitutes those for the generic lifetime parameters.

Lifetimes are a compile-time construct used to ensure all borrows are valid - they have **zero runtime cost**.

## Concept Reference Table

| Concept                           | Example                                                                                          | Notes                                                                               |
| --------------------------------- | ------------------------------------------------------------------------------------------------ | ----------------------------------------------------------------------------------- |
| <br>Lifetime annotation           | <pre><code class="language-rust">fn foo<'a>(x: &'a str)</code></pre>                             | <br>`'a` names a borrow’s lifetime. Doesn’t change runtime behavior.                |
| <br>Multiple lifetimes            | <pre><code class="language-rust">fn bar<'a,'b>(x: &'a str, y: &'b str)</code></pre>              | <br>Each borrow tracked separately; may overlap.                                    |
| <br>Return tied to input          | <pre><code class="language-rust">fn first<'a>(x: &'a str, y: &str) -> &'a str</code></pre>       | <br>Return ref must live at least as long as `'a`.                                  |
| <br>Struct with lifetime          | <pre><code class="language-rust">struct Foo<'a> { <br>s: &'a str<br>}</code></pre>               | <br>Ties reference field to lifetime `'a`.                                          |
| <br>Impl with lifetime            | <pre><code class="language-rust">impl<'a> Foo<'a> { fn get(&self) -> &str { ... } }</code></pre> | <br>Struct methods must respect lifetime constraints.                               |
| <br>Trait bound with lifetime     | <pre><code class="language-rust">fn foo<T: 'a>(x: T)</code></pre>                                | <br>Type `T` must live at least `'a`.                                               |
| <br>`'static` lifetime            | <pre><code class="language-rust">let s: &'static str = "hi";</code></pre>                        | <br>Lives for entire program (string literals, global constants).                   |
| <br>Elision (inferred lifetimes)  | <pre><code class="language-rust">fn foo(x: &str) -> &str</code></pre>                            | <br>Compiler applies lifetime elision rules: parameters → output where unambiguous. |
| <br>Dangling reference prevention | <pre><code class="language-rust">fn bad() -> &str { let s = String::from("x"); &s }</code></pre> | <br>Compile error: ref would outlive owner.                                         |
|                                   |                                                                                                  |                                                                                     |
### Non-overlapping Lifetimes Diagram

``` mermaid
sequenceDiagram

    participant Caller

    participant Function



    Caller->>Caller: let s1 = String::from("hi")

    Caller->>Caller: let r = &s1  // borrow starts ('a)

    Caller->>Function: pass r (&'a str)

    Function->>Function: param x: &'a str

    Function-->>Caller: return &'a str (same ref)

    Note over Caller,Function: Reference is valid as long as 'a is alive

    Caller->>Caller: r ends, 'a borrow ends

    Caller->>Caller: s1 still valid (owner)
```

**Figure 1** Shows a single reference borrowed, passed into a function, and returned. The returned reference remains valid only as long as the original borrow ('a) is alive. Demonstrates lifetime propagation.
### Overlapping Lifetimes Diagram

``` mermaid
sequenceDiagram

    participant Caller

    participant Function



    Caller->>Caller: let s1 = String::from("hi")

    Caller->>Caller: let s2 = String::from("bye")

    Caller->>Function: borrow &s1 ('a starts)

    Caller->>Function: borrow &s2 ('b starts)

    Function->>Function: compare(&'a str, &'b str)

    Function-->>Caller: return bool

    Note over Caller,Function: 'a and 'b live until borrows end

    Caller->>Caller: borrows 'a and 'b end, s1 and s2 valid again
```
**Figure 2** Shows two references ('a and 'b) borrowed at the same time and passed into a function. Lifetimes overlap but are tracked independently, so they don’t interfere with each other.

## References and Aliasing
There are two kinds of references:
- Shared reference: `&`
- Mutable reference: `&mut`
Which obey the following rules:
- A **reference** (`&` or `&mut`) cannot outlive its **referent** (value origin)
- A **mutable reference** (`&mut`) cannot be **aliased** (pointed to by more than one reference)

What is aliasing? From the [aliasing chapter](https://doc.rust-lang.org/nomicon/aliasing.html):

> variables and pointers _alias_ if they refer to overlapping regions of memory.

That means that a read only reference and a mutable reference can't exist at the same time. 

See  [[lifetime_causality#Lifetime Intervals|lifetime intervals]]:
``` mermaid
gantt
    dateFormat  X
    axisFormat  %L

    section Aliasing Violation
    Immutable borrow (&T) :e1, 2, 8
    Mutable borrow (&mut T) :crit, 5, 9

    section Use After Move
    Original ownership valid :f1, 0, 5
    Ownership moved (drop point) :milestone, 5, 0
    Access after move (invalid) :crit, 6, 8
```
## Desugared Borrow Checker

This is an example of how lifetimes are interpreted by the borrow checker:
``` rust
// Each let statement implicitly introduces a scope.
let x = 0;
let y = &x;
let z = &y;


// NOTE: `'a: {` and `&'b x` is not valid syntax!
'a: {
    let x: i32 = 0;
    'b: {
        // lifetime used is 'b because that's good enough.
        let y: &'b i32 = &'b x;
        'c: {
            // ditto on 'c
            let z: &'c &'b i32 = &'c y; // "a reference to a reference to an i32" (with lifetimes annotated)
        }
    }
}
```

Since the references are only used within each let statements inner scope, the borrow checker determines the minimum lifetime to be each let statement's scope.

But what if we pass references to outer scopes? That will make the borrow checker infer a larger lifetime
``` rust
let x = 0;
let z;
let y = &x;
// move y's immutable reference to z
// so the reference needs to live as long as z's lifetime (or scope in this case)
z = y;

// desugared
'a: {
    let x: i32 = 0;
    'b: {
        let z: &'b i32;
        'c: {
            // Must use 'b here because the reference to x is
            // being passed to the scope 'b.
            let y: &'b i32 = &'b x;
            z = y;
        }
    }
}
```

## Lifetime vs Scope

A lifetime of a value is **not the same as its scope**, as a lifetime can encompass **disjoint scopes**.

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

## Lifetime Rules in Functions

### The Rules

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

### Lifetime Elision (Inference) Rules

*Lifetime elision* is basically when the Rust compiler infers certain lifetime annotations in function signatures. 

Lifetime elision = lifetime inference

It uses three rules to assign distinct input lifetimes and map output lifetimes based on the presence of single or multiple input references, including `&self` or `&mut self`:
1. **Each input reference gets its own lifetime:** Every reference parameter with an unspecified lifetime is assigned its own unique lifetime parameter.
2. **If exactly one input lifetime, output gets same lifetime:** If there is exactly one input lifetime parameter, that lifetime is assigned to all elided output lifetimes.
3. **If multiple inputs and one is `&self` or `&mut self`, output gets `self`'s lifetime:** If there are multiple input lifetime parameters, but one of them is `&self` or `&mut self` (a method receiver), the lifetime of `self` is assigned to all elided output lifetimes.

Since Rust can often infer lifetimes, you don't always need to write them:

```rust
// These are equivalent:
fn first_word(s: &str) -> &str { /* ... */ }           // Elided
fn first_word<'a>(s: &'a str) -> &'a str { /* ... */ } // Explicit
```

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

## Static Lifetimes

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

## See Also

- [[lifetime_patterns]] - Common lifetime patterns and pitfalls.
- [[lifetime_causality]] - Exploring temporal and causal structure of lifetimes.
- [[ownership]] - Foundation for understanding lifetimes and borrowing
- [[generics]] - Lifetime parameters work similarly to generic type parameters
- [[traits]] - Lifetime bounds in trait definitions and implementations
- [[smart-pointers]] - Alternative to complex lifetime scenarios (`Rc`, `Arc`, `Box`)

**Practice**: `exercises/16_lifetimes/` | **Review**: [[rust-review-guide#Advanced Features Phase]]
