# Generics

#advanced-features #generics #type-system #traits

> Generic programming in Rust - works closely with [[traits]] and enables [[lifetimes]]

**Related Topics**: [[traits]] | [[lifetimes]] | [[ownership]] | [[errors]]

## Restriction

Restricting generic types => only accept types that implement specific traits (like `std::cmp::PartialOrd`):

```rust
// won't work
fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
```

```rust
$ cargo run
   Compiling chapter10 v0.1.0 (file:///projects/chapter10)
error[E0369]: binary operation `>` cannot be applied to type `&T`
 --> src/main.rs:5:17
  |
5 |         if item > largest {
  |            ---- ^ ------- &T
  |            |
  |            &T
  |
help: consider restricting type parameter `T`
  |
1 | fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
  |             ++++++++++++++++++++++

For more information about this error, try `rustc --explain E0369`.
error: could not compile `chapter10` (bin "chapter10") due to 1 previous error
```

## Method Definitions

When implementing methods for generic structs, you need to declare the generic parameters in the `impl` block. This allows you to define methods that work with the generic types, and you can even add additional generic parameters specific to individual methods.

Example:

```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}
```

We can specify constraints on generic structs with implementing methods that only operate
on structs that hold a specific type, like `Point<f32>`:

```rust
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
```

Generic type parameters in a struct definition aren’t always the same as those you use in that
same struct’s method signatures.

For example, if a method accepts generic types, and returns a new struct, the passed in type may differ from
the struct's.

```rust
struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
```

The purpose of this example is to demonstrate a situation in which some generic parameters are declared with impl and
some are declared with the method definition. Here, the generic parameters X1 and Y1 are declared after impl because
they go with the struct definition. The generic parameters X2 and Y2 are declared after fn mixup because they’re only
relevant to the method.

## Traits as Bounds

When working with generics, the type parameters often must use traits as _bounds_ to stipulate what
functionality a type implements, and allows access to the methods of traits specified in the _bounds_.

```rust
// Define a function `printer` that takes a generic type `T` which
// must implement trait `Display`.
fn printer<T: Display>(t: T) {
    println!("{}", t);
}
```

Bounding restricts the generic to types that conform to the bounds. That is:

```rust
struct S<T: Display>(T);

// Error! `Vec<T>` does not implement `Display`. This
// specialization will fail.
let s = S(vec![1]);
```

## Accessing Generic Instance Methods

```rust
// A trait which implements the print marker: `{:?}`.
use std::fmt::Debug;

trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 { self.length * self.height }
}

#[derive(Debug)]
struct Rectangle { length: f64, height: f64 }
#[allow(dead_code)]
struct Triangle  { length: f64, height: f64 }

// The generic `T` must implement `Debug`. Regardless
// of the type, this will work properly.
fn print_debug<T: Debug>(t: &T) {
    println!("{:?}", t);
}

// `T` must implement `HasArea`. Any type which meets
// the bound can access `HasArea`'s function `area`.
fn area<T: HasArea>(t: &T) -> f64 { t.area() }

fn main() {
    let rectangle = Rectangle { length: 3.0, height: 4.0 };
    let _triangle = Triangle  { length: 3.0, height: 4.0 };

    print_debug(&rectangle);
    println!("Area: {}", area(&rectangle));

    //print_debug(&_triangle);
    //println!("Area: {}", area(&_triangle));
    // ^ TODO: Try uncommenting these.
    // | Error: Does not implement either `Debug` or `HasArea`.
}
```

---

## See Also
- [[traits]] - Essential for generic programming with bounds
- [[lifetimes]] - Generic lifetime parameters
- [[ownership]] - Generic ownership patterns
- [[errors]] - `Result<T,E>` is a generic enum

**Practice**: `exercises/14_generics/` | **Review**: [[rust-review-guide#Advanced Features Phase]]
