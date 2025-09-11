# References, Borrowing, and Ownership

#fundamentals #ownership #memory-safety

> Core concept that enables [[lifetimes]], [[ownership#Structs|structs]], and smart pointers

**Related Topics**: [[lifetimes]] | [[generics]] | [[collections]] | [[errors]]

## Concept Reference Table

| Concept                    | Example                                                        | Rules / Notes                                                                       | Lifetime Implication                           |
| -------------------------- | -------------------------------------------------------------- | ----------------------------------------------------------------------------------- | ---------------------------------------------- |
| Ownership                  | <code class="language-rust">let s = String::from("hi");</code> | Each value has **one owner**. When owner goes out of scope, value is **dropped**.   | Value lives until the owner goes out of scope. |
| Move                       | <code class="language-rust">let s2 = s1;</code>                | Ownership transferred, `s1` no longer valid.                                        | Old owner invalidated.                         |
| Clone (deep copy)          | <code class="language-rust">let s2 = s1.clone();</code>        | Allocates new copy of data on heap. Both valid.                                     | Independent lifetimes.                         |
| Borrow (immutable)         | <code class="language-rust">let r = &s;</code>                 | Multiple immutable refs allowed. Cannot modify while borrowed.                      | Borrowed value must outlive the borrow.        |
| Borrow (mutable)           | <code class="language-rust">let r = &mut s;</code>             | Only **one mutable ref** allowed at a time. No immutable refs at the <br>same time. | Exclusive, temporary ownership.                |
| Dereference                | <code class="language-rust">*r</code>                          | Access value behind a **reference**.                                                | Same lifetime as reference.                    |
| Slice (borrowed view)      | <code class="language-rust">let sub = &s[0..2];</code>         | Non-owning view into data. Follows borrow rules (immut or mut).                     | Slice must not outlive owner.                  |
| Function **ownership in**  | <code class="language-rust">fn takes(s: String)</code>         | Passing by value **moves ownership** into function.                                 | Value dropped when function ends unless moved. |
| Function **borrowing in**  | <code class="language-rust">fn borrow(s: &String)</code>       | Passes reference, does not transfer ownership.                                      | Borrow must outlive function call.             |
| Function return <br>values | <code class="language-rust">fn make() -> String</code>         | Function can **return ownership** to caller.                                        | Caller owns returned value.                    |
| Borrow checker             | _implicit_                                                     | Enforces at compile time: no dangling refs, safe aliasing rules.                    | Prevents invalid lifetimes.                    |
### Ownership Flowchart Diagram

``` mermaid

flowchart TD

    A["Value<br/>(Heap/Stack)"]:::value

    O1["Owner<br/>(let s = String::from(&quot;hi&quot;) )"]:::owner

    O2["Move<br/>(let s2 = s1)"]:::move

    C["Clone<br/>(let s2 = s1.clone())"]:::clone

    B1["Immutable Borrow<br/>(&s)"]:::borrow

    B2["Mutable Borrow<br/>(&mut s)"]:::mutborrow

    F1["Function Takes Ownership<br/>(fn f(s: String))"]:::fnown

    F2["Function Borrows<br/>(fn f(&s))"]:::fnborrow

    R["Return Ownership<br/>(fn f() -&gt; String)"]:::returnval

    S["Slice<br/>(&s[0..n])"]:::slice



    O1 -->|owns| A

    O1 --> O2

    O1 --> C

    O1 --> B1

    O1 --> B2

    O1 --> F1

    O1 --> F2

    F1 -->|drops unless returned| A

    F2 -->|borrow only| A

    O2 -->|new owner| A

    C -->|deep copy| A

    B1 -->|read-only view| A

    B2 -->|exclusive access| A

    R -->|gives back ownership| O1

    O1 --> S

    S -->|borrowed view| A



    classDef value fill:#f8e7a1,stroke:#d9b400,stroke-width:2px;

    classDef owner fill:#c8e6c9,stroke:#2e7d32,stroke-width:2px;

    classDef move fill:#ffe0b2,stroke:#ef6c00,stroke-width:2px;

    classDef clone fill:#bbdefb,stroke:#1565c0,stroke-width:2px;

    classDef borrow fill:#e1bee7,stroke:#6a1b9a,stroke-width:2px;

    classDef mutborrow fill:#ffcdd2,stroke:#b71c1c,stroke-width:2px;

    classDef fnown fill:#fff9c4,stroke:#fbc02d,stroke-width:2px;

    classDef fnborrow fill:#d1c4e9,stroke:#4527a0,stroke-width:2px;

    classDef returnval fill:#b2dfdb,stroke:#00695c,stroke-width:2px;

    classDef slice fill:#f0f4c3,stroke:#827717,stroke-width:2px;
```

**Figure 1** Illustrates ownership transfer rules: values have one owner, ownership can move, and borrows (mutable or immutable) temporarily restrict access. Emphasizes relationships, not time.
### Ownership Movement Diagram

``` mermaid
sequenceDiagram

    participant Caller

    participant Function



    Caller->>Caller: let s = String::from("hi")

    Caller->>Function: pass s (ownership moves)

    Function->>Function: fn takes(s: String)

    Note over Function: Caller no longer owns s

    Function-->>Caller: return s (ownership returned)

    Note over Caller,Function: Caller regains ownership

```
**Figure 2** Shows ownership movement across function calls: caller creates a value, passes it into a function (ownership moves), and regains it only if the function returns ownership.
### Ownership Stack and Heap Allocation Diagrams

``` mermaid
flowchart TD

    %% Nodes

    subgraph Stack["Stack Frame"]

        P["Primitive (i32, bool, char)"]:::primitive

        M["Metadata for Complex Type<br/>(ptr, len, capacity)"]:::meta

    end



    subgraph Heap["Heap Allocation"]

        H["Buffer/Data (e.g., String contents)"]:::heap

    end



    %% Links

    P -->|Copy| P2["Copied Primitive"]:::primitive

    M -->|points to| H

    M -->|"Move (ownership transfer)"| M2["New Owner's Metadata"]:::meta

    M2 --> H

    M -->|Clone| H2["New Heap Allocation (deep copy)"]:::heap



    %% Styles

    classDef primitive fill:#c8e6c9,stroke:#2e7d32,stroke-width:2px;

    classDef meta fill:#bbdefb,stroke:#1565c0,stroke-width:2px;

    classDef heap fill:#ffcdd2,stroke:#b71c1c,stroke-width:2px;

```

**Figure 3** Separates primitives (Copy types) from complex types. Primitives copy directly on the stack. Complex types have stack metadata pointing to heap data; ownership, moves, and clones affect who manages the heap allocation.


``` mermaid
sequenceDiagram

    participant Caller_Stack as Caller (Stack Frame)

    participant Heap as Heap (Allocated Data)

    participant Function_Stack as Function (Stack Frame)



    %% Primitive example

    Caller_Stack->>Caller_Stack: let x: i32 = 42

    Caller_Stack->>Function_Stack: copy x (i32 is Copy)

    Note over Caller_Stack,Function_Stack: Both can use x independently



    %% Complex type example

    Caller_Stack->>Caller_Stack: let s = String::from("hi")

    Caller_Stack->>Heap: allocate "hi"

    Caller_Stack->>Function_Stack: move s (stack metadata)

    Note over Caller_Stack,Function_Stack: Caller loses access to s

    Function_Stack->>Heap: access "hi"

    Function_Stack-->>Caller_Stack: return s (metadata ownership back)

    Note over Caller_Stack: Caller owns heap again
```

**Figure 4** Shows timeline of ownership for both primitives and complex types. Primitives are copied freely. Complex types involve stack metadata moving between caller and function, which controls access to the underlying heap allocation.
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
