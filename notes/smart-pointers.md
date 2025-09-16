# Smart Pointers & Memory Management

#smart-pointers #box #rc #refcell #arc #mutex #memory-management

> Advanced memory management with smart pointer types


In Rust, a smart pointer is a pointer that refers to some data like *references* (`&`), but with some additional metadata and capabilities that may include:
- *reference counting* - enables data to have multiple owners by keeping track and cleaning up data when none remain
- *ownership* - owning the data the smart pointer points to

Basically, it provides a way to own the data and pass it around for callers to safely manipulate it. 

Smart pointers must implement the [[smart-pointers#`Deref` and `Drop` Traits|`Deref` and `Drop` traits]].

Some examples of stdlib smart pointers are `String` and `Vec<T>`.

## Reference Table

| Smart Pointer                                  | Mutable                              | Usage                                                                                                                                   | Capabilities                | Overhead                          |
| ---------------------------------------------- | ------------------------------------ | --------------------------------------------------------------------------------------------------------------------------------------- | --------------------------- | --------------------------------- |
| [[smart-pointers#Box <T >\|Box<T>]]            | Yes; single owner can dereference    | for "wrapping" type that has unknown compile time size; transferring ownership for a large amount of data; owning value based off trait | None                        | None                              |
| [[smart-pointers#Rc <T >\|Rc<T>]]              | No - immutable only; multiple owners | for when last reference to value is unknown at compile time AND allow multiple owners                                                   | Reference counting          | Minimal; just incr/decr a counter |
| [[smart-pointers#RefCell <T >\|RefCell<T>]]    | Yes; multiple owners but is `unsafe` | for bypassing the borrow checker rules by enforcing borrowing rules at runtime for known valid programs                                 | interior mutability pattern |                                   |
| <code class="language-rust">Ref\<T\></code>    |                                      |                                                                                                                                         |                             |                                   |
| <code class="language-rust">RefMut\<T\></code> |                                      |                                                                                                                                         |                             |                                   |

---
## `Deref` and `Drop` Traits

Smart pointers are usually implemented using structs.

Smart pointers must implement the `Deref` and `Drop` traits
- `Deref` - allows instance to behave like a reference, so you can pass it with `&`, and dereference it with `*`.
- `Drop` - custom code to run when pointer goes out of scope (clean up resources).

### Implementing `Deref` 

First, let's see what happens when we define a type that *Doesn't* implement the `Deref` trait:

``` rust
// MyBox tuple struct that with element of type T
struct MyBox<T>(T);

impl<T> MyBox<T> {
	// `new` returns MyBox holding the given value.
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// Won't compile since it doesn't know how to dereference MyBox -- it isn't a pointer, just a struct
fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y); // ERROR: type `MyBox<{integer}>` cannot be dereferenced
}
```

Implementing `Deref` requires:
- `type Target = T;` syntax for an *associated type* for the `Deref` trait to use. (note: associated types are a slightly different way of declaring a generic parameter).
- Implementing the `deref`  method

``` rust
use std::ops::Deref;

// MyBox tuple struct that with element of type T
struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
	// associated type for the Deref trait to use
    type Target = T;

	// new returns MyBox holding the given value.
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
    
	// deref returns the reference to Target, which can then be dereferenced with '*'.
    fn deref(&self) -> &Self::Target {
	    // .0 accesses the first element of the tuple
        &self.0
    }
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);           // passes
    assert_eq!(5, *(y.deref())); // equivalent statement
}
```

Rust substitutes the `*` operator with a call to the `deref` method and then a plain dereference so we don’t have to think about whether or not we need to call the `deref` method.

`*y` is equivalent to `*(y.deref())`, which is basically saying, "dereference the reference to the `Target`", since `y.deref()` returns the `Target` reference.

### Implementing `Drop`

`Drop` let's you customize what happens just before a value of the implementor is dropped out of scope.

For example, you can use `Drop` to close files or network connections.

> Note: this seems like what Golang is trying to do with [runtime.AddCleanUp](https://pkg.go.dev/runtime#Cleanup), but with no guarantees on *when* it is called, since it depends on when it is determined unreachable by the GC.

The code in defined in the `drop` method will be inserted by the compiler automatically wherever a value goes out of scope.

``` rust
struct CustomSmartPointer {
    data: String,
}

// CustomSmartPointer struct that implements the Drop trait where we would put our cleanup code
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
	// create two instances of CustomSmartPointer and then print CustomSmartPointers created
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
}
```

The `Drop` trait is included automatically in every program via the [prelude](https://doc.rust-lang.org/std/prelude/index.html), so doesn't need to be explicitly imported.

The code above will produce:
``` sh
$ cargo run
   Compiling drop-example v0.1.0 (file:///projects/drop-example)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.60s
     Running `target/debug/drop-example`
CustomSmartPointers created.
Dropping CustomSmartPointer with data `other stuff`!
Dropping CustomSmartPointer with data `my stuff`!
```

An important thing to note is the **drop order**. Variables are dropped in the *reverse order* of their creation, so `d` was dropped before `c`:
1) Scope begins
2) `c` created
3) `d` created
4) Scope ends
5) `d` dropped
6) `c` dropped

### Forcing Cleanup
In order to cleanup a value *explicitly*, for example when using smart pointers that manage locks, you might want to **force the `drop` method** that releases the lock so that other code in the same scope can acquire the lock.

Rust doesn't allow explicit calls to `drop`, so instead you need to call the `std::mem::drop` function provided by the standard library if you want to force a value to be dropped before the end of its scope.

The `std::mem::drop` function is different from the `drop` method in the `Drop` trait. We call it by passing as an argument the value we want to force-drop. The function is in the prelude, so can be invoked like:

``` rust
fn main() {
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    // c.drop() would fail with `explicit use of destructor method`
    drop(c); // passes since it calls `std::mem::drop`
    println!("CustomSmartPointer dropped before the end of main.");
}
```

You don’t have to worry about accidentally cleaning up values still in use: the ownership system that ensures references are always valid also ensures that `drop` gets called *only once* when the value is no longer being used.

---
## `Box<T>`

`Box<T>` is used for allocating a value on the heap and providing a reference to it. The pointer remains on the stack.

It is the most straightforward smart pointer to use. As such, it doesn't offer many capabilities.

Used for:
1. "wrap" a type of an unknown size at compile time in a context where the size is required to be known.
2. Transferring ownership of a large amount of data; avoids copying heap data.
3. Owning a value based off its trait implementation, and not its concrete type.

(2.) means storing the data in a "box" on the heap, so only the pointer data in the stack would need to be copied around instead of the heap data.

The `i32` value `5` gets allocated on the heap, and is pointed to by the `Box`:

```rust
fn main() {
	// Box pointer address is in stack (0xdeadbeef)
	// i32 value is in heap (5)
    let b = Box::new(5);
    println!("b = {b}");
}
```

When going out of scope, both will be deallocated:
-  `5` value stored on the heap
- `Box` pointer address stored on the stack 
### Using `Box<T>` to Enable Recursive Types

Defining recursive types in Rust is an issue, since it needs to know the size types at compile time.

Basically, if we nest the definition, that means the "top-level" type has to account for the size of itself as one of its members, which results in infinite recursion. 
### Naive Approach

For example, consider this [[enums|enum]] definition:
``` rust
enum List {
    Cons(i32, List),
    Nil,
}
```

Then at compile time, that results in:
``` rust
let size = size_of::<List>()
//                    ^ expand this
		 = size_of::<Nil>() + size_of::<Cons>();
//                                        ^ expand this
		 = size_of::<Nil>() + ( size_of::<i32>() + mem::size_of::<List>() )
//                                                                  ^ expand this
// Will error at compile-time with "recursive type `List` has infinite size"
```

Which results in the shape:

![An infinite Cons list](https://doc.rust-lang.org/book/img/trpl15-01.svg)
### Robust Approach Using `Box<T>`

The `Box` type solves this since it has a *known size* at compile time, since a pointer’s size doesn’t change based on the amount of data it’s pointing to.

``` rust
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
```

Which results in the shape:

![A finite Cons list](https://doc.rust-lang.org/book/img/trpl15-02.svg)

Using it like a reference looks like:
``` rust
fn main() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
```

---
## `Rc<T>`

`Rc<T>` is a reference counting type that enables multiple ownership of *immutable references*. Not thread-safe.

When the reference count goes to zero, the value can be cleaned up. 

Reference counting methods:
1. Increment: `Rc::clone` the `Rc<T>` value to increment
2. Decrement: Dropping the `Rc<T>` value to decrement, either when it goes out of scope or by using `std::mem::drop`
3. Show: `Rc::strong_count`  will return the reference count of `Rc<T>`

We use this type when we want to allocate some data on the heap for multiple parts of our program to read and we **can’t determine at compile time which part will finish using the data last**. 

Using `Rc<T>` allows a single value to have multiple owners, and the count ensures that the value remains valid as long as any of the owners still exist.

For example, in graph data structures, multiple edges might point to the same node, and that node is conceptually "owned" by all of the edges that point to it. A node should only be dropped if it is disconnected, i.e. no edges pointing to it, aka nothing owns it.

`Rc<T>` is for use in single-threaded scenarios only.
### Using `Rc<T>` to share data

Suppose we have two lists `b` and `c` that share ownership of a third list `a`

![Two lists that share ownership of a third list](https://doc.rust-lang.org/book/img/trpl15-03.svg)
This won't work with `Box<T>`, since the value of `a` will be moved from `b` to `c` via `Box`, and the `List` type doesn't implement the `Copy` trait. 

### Naive Approach Using `Box<T>`

``` rust
enum List {
	// Cons owns its values
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
	// a owns the head of its List, but points to the allocated elements
    let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    // b owns the head of its List, and then ownership of `a` is transferred via Box
    let b = Cons(3, Box::new(a));
    // Trying to move owned value a to c 
    // Errors with: use of moved value: `a`
    // this breaks ownership rule that there can't be more than one mutable reference to a memory region
    let c = Cons(4, Box::new(a));
}
```

### Naive Approach Using References

``` rust
enum List {
    Cons(i32, &List),
    Nil,
}
```

Another way to solve this is by changing the definition of `Cons`  to hold *references instead of owned values*.

This means specifying lifetime parameters saying that **every element in the list will live at least as long as the entire list**, which may be true in the example, but not in every scenario, like if elements were removed. 

### Robust Approach With `Rc<T>` and `Rc::clone`

Each `Cons` variant will hold a value and an `Rc<T>` pointing to a `List` instead of the value of the `List`:

``` rust
enum List {
    Cons(i32, Rc<List>), // changed from Box<List>
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc // brings `Rc<T>` into scope because it’s not in the prelude

fn main() {
	// a = Rc[ 5, Rc[10, Rc[ Nil ] ] ]
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    // b = [ 3, Rc[ 5, Rc[ 10, Rc[ Nil ] ] ] ] 
    let b = Cons(3, Rc::clone(&a));
    // c = [ 4, Rc[ 5, Rc[ 10, Rc[ Nil ] ] ] ]
    let c = Cons(4, Rc::clone(&a));
}
```

The initialization order will look like:

| Variable | Description                                                                      | Value (pseudo-code)                   | Reference Count |
| -------- | -------------------------------------------------------------------------------- | ------------------------------------- | --------------- |
| `a`      | The thing we want to share is `a`, so `a` will own an `Rc<List>`                 | `Rc[ 5, Rc[10, Rc[ Nil ] ] ]`         | 1               |
| `b`      | `b` will remain an owned `List` but will hold a reference to `a` via `Rc::clone` | `[ 3, Rc[ 5, Rc[ 10, Rc[ Nil ] ] ] ]` | 2               |
| `c`      | `c` will remain an owned `List` but will hold a reference to `a` via `Rc::clone` | `[ 4, Rc[ 5, Rc[ 10, Rc[ Nil ] ] ] ]` | 3               |

We could have called `a.clone()` rather than `Rc::clone(&a)`, but the implementation of `Rc::clone` *doesn’t make a deep copy* of the data unlike most types’ implementations of `clone` do.

Using `Rc::clone` for reference counting visually distinguishes between "deep-copy" clones "reference counting" clones. 

So basically, there is minimal performance overhead of incrementing the reference count when using `Rc::clone`, unless `.clone` is used.

### Showing `Rc<T>` Reference Count with `Rc::strong_count`

``` rust
fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    { // force c to go out of scope
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
```

This function is named `strong_count` rather than `count` because the `Rc<T>` type also has a `weak_count`.
This code prints the following:

``` sh
$ cargo run
   Compiling cons-list v0.1.0 (file:///projects/cons-list)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.45s
     Running `target/debug/cons-list`
count after creating a = 1
count after creating b = 2
count after creating c = 3
count after c goes out of scope = 2
```

When `b` and then `a` go out of scope at the end of `main`, the count is then 0, and the `Rc<List>` is cleaned up completely. 

---
## `RefCell<T>`

`Ref<T>` and `RefMut<T>`, accessed through `RefCell<T>`, a type that enforces the borrowing rules at runtime instead of compile time. It is not thread safe.

It allows for *interior mutability* which allows you to mutate data despite **immutable references** to that data, which is disallowed by borrowing rules. In other words, it is *mutating the value inside an immutable value*.

In order to bypass borrowing rules, we must use `unsafe` code, but only within the API boundary. The outer type is still immutable, and safe.

Recall the borrowing rules:
- At any given time, you can have _either_ one mutable reference or any number of immutable references (but not both).
- References must always be valid.

> Note: 
> 
>  If you break these rules with references (`&`), you’ll get a compiler error. 
>  
>  If you break these rules with `RefCell<T>`, your program will panic and exit.
> 
> The advantage of checking the borrowing rules at runtime instead of at compile time is that certain memory-safe scenarios that are blocked by the borrow checker are allowed. This is because static analysis cannot evaluate every possible program state, so the borrow checker can reject correct programs.  
 
Because `RefCell<T>` allows mutable borrows checked at runtime, you can mutate the value inside the `RefCell<T>` even when the `RefCell<T>` is immutable.


## `Cow<'a, B>` Enum

The `Cow` enum is a clone-on-write smart pointer, it can enclose and provide immutable access to borrowed data, and clone the data lazily when mutation or ownership is required.

Basically it only *clones* when the caller wants to *modify* or *own* the value. Otherwise it just provides *immutable access*.

`Cow` implements `Deref`, which means that you can call *non-mutating* methods directly on the data it encloses. If mutation is desired, `to_mut` will obtain a *mutable reference* to an owned value, cloning if necessary.

Reference counting pointers `Rc::make_mut` and `Arc::make_mut` provide clone-on-write functionality as well.