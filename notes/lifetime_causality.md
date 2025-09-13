## Lifetime Faults as Causality Violations

Rust’s borrow checker is essentially enforcing a **_partial order of validity_** between **references** and their **referents**. 

 Let’s treat lifetimes as **intervals on a timeline** and use causality-style notation:
- **⊆** (subset relation): Lifetime A is _contained within_ Lifetime B.
    → A ⊆ B means _A happens entirely during B_.
- **⊀** (not precedes / no containment): A does not live long enough to contain B.
    → A ⊀ B means _A does not outlive B_, so ordering is broken.

``` mermaid
graph LR

    subgraph Lifetimes
        A["Referent lifetime"] 
        B["Reference lifetime"]
    end

    %% Valid Case
    A -->|A ⊇ B| B

    subgraph Faults
        F1["Dangling Reference: B continues after A ends"]
        F2["Reference Outlives Referent: claim 'a ⊇ 'b, but 'b ⊀ 'a"]
        F3["Mismatched Lifetimes: 'short ⊀ 'long"]
        F4["Aliasing Violation: mutable + shared overlap (time conflict)"]
        F5["Use After Move: ownership transferred, later use assumes ownership still valid"]
        F6["Unconstrained Lifetime: 'a not tied to input/output → no causal anchor"]
        F7["Self-referential Struct: field depends on itself (cycle in order graph)"]
        F8["Returning Local Borrow: local 'b ends, ref claims 'a continues"]
    end

    %% Map examples
    A -.->|violated causality| F1
    A -.->|violated containment| F2
    B -.->|lifetime too short| F3
    A -.->|concurrent access hazard| F4
    A -.->|token already moved| F5
    A -.->|undefined relation| F6
    A -.->|cycle| F7
    A -.->|temporal mismatch| F8

    style A fill:#bbf,stroke:#333,stroke-width:2px
    style B fill:#bbf,stroke:#333,stroke-width:2px
    style F1 fill:#fbb,stroke:#333,stroke-width:1px
    style F2 fill:#fbb,stroke:#333,stroke-width:1px
    style F3 fill:#fbb,stroke:#333,stroke-width:1px
    style F4 fill:#fbb,stroke:#333,stroke-width:1px
    style F5 fill:#fbb,stroke:#333,stroke-width:1px
    style F6 fill:#fbb,stroke:#333,stroke-width:1px
    style F7 fill:#fbb,stroke:#333,stroke-width:1px
    style F8 fill:#fbb,stroke:#333,stroke-width:1px
```
## Lifetime Intervals

**lifetimes are intervals**; references are events that must happen _within_ those intervals. The borrow checker ensures no reference “escapes” beyond its valid causal interval.

``` mermaid
gantt
    dateFormat  X
    axisFormat  %L

    section Valid Case
    Referent lifetime :a1, 0, 10
    Reference lifetime (⊆ referent) :a2, 2, 8

    section Dangling Reference
    Referent lifetime :b1, 0, 5
    Reference lifetime (extends too far) :b2, 2, 8

    section Reference Outlives Referent
    Caller 'a lifetime :c1, 0, 12
    Function-local 'b lifetime :c2, 4, 7
    Returned ref claims 'a but only has 'b :crit, 4, 7

    section Mismatched Lifetimes
    Expected 'long :d1, 0, 12
    Provided 'short :d2, 0, 4

    section Aliasing Violation
    Immutable borrow (&T) :e1, 2, 8
    Mutable borrow (&mut T) :crit, 5, 9

    section Use After Move
    Original ownership valid :f1, 0, 5
    Ownership moved (drop point) :milestone, 5, 0
    Access after move (invalid) :crit, 6, 8
```
## Fault Categories

Every fault is some form of:
- **Invalid ordering** (`ref after drop`), or    
- **Conflicting concurrency** (`mut + shared`), or
- **Unanchored causality** (`'a without source`).

| Fault Category                  | Causal Relation Broken                                                                 |
| ------------------------------- | -------------------------------------------------------------------------------------- |
| **Dangling Reference**          | Reference continues *after* the value it points to is dropped                          |
| **Reference Outlives Referent** | `'a` (caller lifetime) is claimed to contain `'b` (function-local), but `'b ⊀ 'a`      |
| **Mismatched Lifetimes**        | Expected `'long` but provided `'short`; violates `'short ⊆ 'long` ordering             |
| **Aliasing Violation**          | Immutable borrow and mutable borrow overlap in time                                    |
| **Use After Move**              | Ownership transfer event occurs, but later usage assumes ownership was not transferred |
| **Unconstrained Lifetime**      | Introduces a lifetime without causal ties to any input/output                          |
| **Self-referential Structs**    | Struct field wants to causally depend on itself                                        |
| **Returning Local Borrow**      | Reference to local created in function outlives function return (temporal mismatch)    |

## Fault: Reference Outlives Referent

### Example 1 

The following is an example of a returning a reference that will be invalid since it will outlive its referent:

``` rust
fn as_str(data: &u32) -> &str {
    let s = format!("{}", data);
    &s
}
```

What this says is that:
- Function receives a **reference** `&u32` with some lifetime `'a`.
- It produces a **reference** `&s` to a **referent** `str` value in the function scope with some lifetime `'b`.
- It returns the reference `&s` with a promise that will it live as long as `&u32`, or in other works, that it will have the same lifetime `'a`.

It is impossible to satisfy this contract, since the lifetime of `'b` will end when the function returns, which means the **referent** `str` value will be dropped.

```rust
// The callsite with lifetime a'
a' {
	let x = 1;
	let s = as_str(&x);
}

// The desugared function that breaks its promise :(
fn as_str<'a>(data: &'a u32) -> &'a str {
    'b: {
        let s = format!("{}", data);
        return &'a s;
    }
}
```

This means we can't return a reference from `as_str`, it must be an **owned** string (allocated on heap) or a **literal** (allocated on stack).

``` rust
fn to_string(data: &u32) -> String {
    format!("{}", data)
}
```

### Example 2

This would be unsafe - returning a reference to local data in the function:

```rust
fn get_string() -> &str {
    let s = String::from("hello");
    &s  // ERROR: `s` dropped here while still borrowed
}  // `s` goes out of scope

```

The safe versions would be to return a reference where it has the same lifetime as the input, or to return an owned value:

``` rust
// Referent is in caller
fn get_prefix(text: &str) -> &str {
    &text[0..5]  // OK: reference has same lifetime as input
}

// Return owned value
fn get_prefix(text: &str) -> String {
    format!("{}", text[0..5])  // OK: reference has same lifetime as input
}
```

## Fault: Unconstrained Lifetime

This function declares lifetime 'a but has no input references, so it can be ANY lifetime:

```rust
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

**Key insight**: Lifetime parameters must be connected to input references or be `'static`, which means they live the length of the remainder of the program.
