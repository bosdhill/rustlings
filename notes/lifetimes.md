# Lifetimes

Lifetimes tells the compiler to explictly keep _references valid_ even if the borrowed value goes out of scope, e.g.
"make sure parameter 'a' lives as long as parameter 'b' so that the return
value is valid".

Lifetime parameters signify particular lifetimes of values that are borrowed.

> Note: Lifetime parameters are _inferred_ from the calling scope by the compiler. The compiler looks at the actual references you're passing in. It then determines what concrete lifetimes those references have in the calling context. It substitutes those concrete lifetimes for the generic lifetime parameters.

Basically, it is a construct used by the compiler to ensure all borrows are valid.

A lifetime of a value is not the same as its scope:
``` rust
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

## Syntax

Explicit annotation of a type has the form &'a T where 'a has already been introduced. This lifetime syntax indicates 
that the lifetime of foo may not exceed that of 'a.

``` rust
foo<'a>
// `foo` has a lifetime parameter `'a`
```

the lifetime of foo cannot exceed that of either 'a or 'b:

``` rust
foo<'a, 'b>
// `foo` has lifetime parameters `'a` and `'b`
```

When used as parameters in a function:
``` rust
// `print_refs` takes two references to `i32` which have different
// lifetimes `'a` and `'b`. These two lifetimes must both be at
// least as long as the function `print_refs`.
fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("x is {} and y is {}", x, y);
}
```

The following will error because the lifetime of `_x` will not outlive `_y`.

There are also no arguments to the function that will force `'a` to live longer.
``` rust
// A function which takes no arguments, but has a lifetime parameter `'a`.
fn failed_borrow<'a>() {
    let _x = 12;

    // ERROR: `_x` does not live long enough
    let _y: &'a i32 = &_x;
    // Attempting to use the lifetime `'a` as an explicit type annotation 
    // inside the function will fail because the lifetime of `&_x` is shorter
    // than that of `_y`. A short lifetime cannot be coerced into a longer one.
}
```

When a function has a lifetime parameter like <'a>, it means the function can accept or return references with that 
particular lifetime. The lifetime parameter is a way to tell the compiler:
"these references need to live at least this long.": `fn failed_borrow<'a>()`

This is saying "the reference stored in _y must have the lifetime 'a.": `let _y: &'a i32 = &_x;`

Since `'a` can be any lifetime, that means it _may outlive_ the function call, and since `_x` goes out of scope
after the function call, it will not live long enough to be used with `'a`.

## Functions

For function signatures with lifetime parameters:
1) any reference (declared in the function) must be annotated with a lifetime.
2) any referencde returned must have the same lifetime as _an input_ or be declared _static_.

Note that returning references without input is banned if it would result 
in returning references to invalid data.

Invalid example:
``` rust
fn invalid_output<'a>() -> &'a String { &String::from("foo") }
```
The above is invalid: `'a` must live longer than the function.
Here, `&String::from("foo")` would create a `String`, followed by a
reference. Then the data is dropped upon exiting the scope, leaving
a reference to invalid data to be returned.

Valid examples:
``` rust
// One input reference with lifetime `'a` which must live
// at least as long as the function.
fn print_one<'a>(x: &'a i32) {
    println!("`print_one`: x is {}", x);
}

// Mutable references are possible with lifetimes as well.
fn add_one<'a>(x: &'a mut i32) {
    *x += 1;
}

// Multiple elements with different lifetimes. In this case, it
// would be fine for both to have the same lifetime `'a`, but
// in more complex cases, different lifetimes may be required.
fn print_multi<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("`print_multi`: x is {}, y is {}", x, y);
}

// Returning references that have been passed in is acceptable.
// However, the correct lifetime must be returned.
fn pass_x<'a, 'b>(x: &'a i32, _: &'b i32) -> &'a i32 { x }
```

## Structs
Struct methods are annotated similar to functions:
``` rust
struct Owner(i32);

impl Owner {
    // Annotate lifetimes as in a standalone function.
    fn add_one<'a>(&'a mut self) { self.0 += 1; }
    fn print<'a>(&'a self) {
        println!("`print`: {}", self.0);
    }
}

fn main() {
    let mut owner = Owner(18);

    owner.add_one();
    owner.print();
}
```

Annotation of lifetimes in structures are also similar to functions:
``` rust
// A type `Borrowed` which houses a reference to an
// `i32`. The reference to `i32` must outlive `Borrowed`.
#[derive(Debug)]
struct Borrowed<'a>(&'a i32);

// Similarly, both references here must outlive this structure.
#[derive(Debug)]
struct NamedBorrowed<'a> {
    x: &'a i32,
    y: &'a i32,
}

// An enum which is either an `i32` or a reference to one.
#[derive(Debug)]
enum Either<'a> {
    Num(i32),
    Ref(&'a i32),
}

fn main() {
    let x = 18;
    let y = 15;

    let single = Borrowed(&x);
    let double = NamedBorrowed { x: &x, y: &y };
    let reference = Either::Ref(&x);
    let number    = Either::Num(y);

    println!("x is borrowed in {:?}", single);
    println!("x and y are borrowed in {:?}", double);
    println!("x is borrowed in {:?}", reference);
    println!("y is *not* borrowed in {:?}", number);
}
```

## Traits

Annotation of lifetimes in trait methods basically are similar to functions. Note that impl may have annotation of lifetimes too.

``` rust
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

1) `T: 'a`: All references in `T` must outlive lifetime `'a`. (For example lifetime of fields in a struct / enum type).
2) `T: Trait + 'a`: Type `T` must implement trait `Trait` and all 
references in `T` must outlive `'a`.

``` rust
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

``` rust
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
1) Making a constant with the `static` declaration.
2) Making a string literal which has type: `&'static str`.

``` rust
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

``` rust
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

## Static Trait Bounds

As a trait bound, it means the type does not contain any non-static references. Eg. the receiver can hold on to the type for as long as they want and it will never become invalid until they drop it.

A reference with a `'static` lifetime (`&'static T`) is actually a reference that lives for the entire program. These typically point to compile-time constants or string literals.

Owned data (without any references inside it) automatically satisfies the `T: 'static` bound, even though the variable itself might not live for the entire program. This is because the bound is about the type's contents, not the variable's actual lifetime.

``` rust
use std::fmt::Debug;

fn print_it( input: impl Debug + 'static ) {
    println!( "'static value passed in is: {:?}", input );
}

fn main() {
    // i is owned and contains no references, thus it's 'static:
    let i = 5;
    print_it(i);

    // oops, &i only has the lifetime defined by the scope of
    // main(), so it's not 'static:
    print_it(&i);
}
```
will output:
```
error[E0597]: `i` does not live long enough
  --> src/lib.rs:15:15
   |
15 |     print_it(&i);
   |     ---------^^--
   |     |         |
   |     |         borrowed value does not live long enough
   |     argument requires that `i` is borrowed for `'static`
16 | }
   | - `i` dropped here while still borrowed
```