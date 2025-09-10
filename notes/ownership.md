# References, Borrowing, and Ownership

#fundamentals #ownership #memory-safety

> Core concept that enables [[lifetimes]], [[ownership#Structs|structs]], and smart pointers

**Related Topics**: [[lifetimes]] | [[generics]] | [[collections]] | [[errors]]

## Mutable vs Immutable References

Multiple immutable references are allowed because no one who is just reading the data has the ability to affect anyone else's reading of the data.

A reference's scope starts from where it is introduced and continues through the last time that reference is used:

```rust
// Multiple immutable references - allowed
let s = String::from("hello");
let r1 = &s;    // immutable reference 1
let r2 = &s;    // immutable reference 2
println!("{} and {}", r1, r2);  // both references are valid here
```

Mutable references have one big restriction: if you have a **mutable reference** to a value, you can have **no other references** to that value.

```rust
// Mutable reference exclusivity
let mut s = String::from("hello");
let r1 = &mut s;    // mutable reference
// let r2 = &mut s; // ERROR: cannot borrow `s` as mutable more than once
// let r3 = &s;     // ERROR: cannot borrow `s` as immutable while it's borrowed as mutable
r1.push_str(", world");
println!("{}", r1);  // OK
```

## Ownership Rules

1. Each value in Rust has an **owner**.
2. There can only be one **owner** at a time.
3. When the **owner** goes out of scope, the value will be **dropped**.

> 💡 These rules enable [[lifetimes]] and make smart pointers necessary for shared ownership

This is for *heap-allocated values*. For *stack-based values* for primitive types, they can be copied and
ownership semantics do not apply.

See [the stack and the heap](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#the-stack-and-the-heap).

```rust
// Ownership example
{
    let s = String::from("hello");  // s owns the String
    // do stuff with s
}  // s goes out of scope, String is dropped

// Ownership transfer (move)
let s1 = String::from("hello");
let s2 = s1;  // s1's ownership moves to s2
// println!("{}", s1);  // ERROR: s1 is no longer valid
```

## Borrowing vs Ownership

Borrowing will take a reference to a variable and do something with it, but cannot drop it.

Ownership is a mutable reference to some variable that can drop it.

**Borrowing** = an immutable or mutable reference to some variable, that is temporary to do some operation, but will *return ownership*

```rust
// Borrowing example
fn calculate_length(s: &String) -> usize {  // borrows s immutably
    s.len()
}  // s goes out of scope, but doesn't drop the String because it doesn't have ownership

let s1 = String::from("hello");
let len = calculate_length(&s1);  // passing a reference (&s1) is borrowing
println!("The length of '{}' is {}.", s1, len);  // s1 is still valid here
```

**Ownership** = a mutable reference to some variable, that after going out of scope will *drop the value*

```rust
// Ownership transfer example
fn take_ownership(s: String) {  // s comes into scope
    println!("{}", s);
}  // s goes out of scope and `drop` is called, freeing memory

let s1 = String::from("hello");
take_ownership(s1);  // s1's value moves into the function
// println!("{}", s1);  // ERROR: s1 is no longer valid
```

# Structs

## Ownership of Struct Data

It's possible for structs to store references to data owned by something else, but to do so requires the use of [[lifetimes]].

```rust
struct User {
    active: bool,
    username: &str,
    email: &str,
    sign_in_count: u64,
}
```

## Update Syntax

Struct update syntax is a concise way to copy fields from one struct instance to another:

```rust
fn main() {
    // --snip--

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
}
```

vs

```rust
fn main() {
    // --snip--

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
}
```

## Methods

### Ownership

The same rules apply to struct methods. Usually, you would only want to borrow (i.e. **immutable reference** to `self`), and not take ownership (i.e **mutable reference** to `self`):

```rust
impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }
}
```

_Note: we don’t want to take ownership, and we just want to read the data in the struct, not write to it. If we wanted to change the instance that we’ve called the method on as part of what the method does, we’d use `&mut self` as the first parameter. Having a method that takes ownership of the instance by using just `self` as the first parameter is rare; this technique is usually used when the method transforms self into something else and you want to prevent the caller from using the original instance after the transformation._

### Automatic Dereferencing

Rust has a feature called _automatic referencing and dereferencing_. Calling methods is one of the few places in Rust with this behavior.

Here’s how it works: when you call a method with `object.something()`, Rust automatically adds in `&`, `&mut`, or `*` so the **object matches the signature of the method**. In other words, the following are the same:

```rust
p1.distance(&p2); // cleaner
(&p1).distance(&p2); // messy
```

This automatic referencing behavior works because methods have a **clear receiver**—the type of `self`. Given the receiver and name of a method, Rust can figure out definitively whether the method is reading (`&self`), mutating (`&mut self`), or consuming (`self`).

---

## See Also
- [[lifetimes]] - How long references are valid
- [[generics]] - Generic ownership patterns
- [[smart-pointers]] - Alternative ownership models (`Box`, `Rc`, `Arc`)
- [[collections]] - Ownership in data structures
- [[traits]] - Ownership in trait methods

**Practice**: `exercises/06_move_semantics/` | **Review**: [[rust-review-guide#Foundation Phase]]
