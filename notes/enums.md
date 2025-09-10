# Enumerations

#core-concepts #pattern-matching #enums

> Foundation for [[option]] and [[errors]] - enables powerful pattern matching

**Related Topics**: [[option]] | [[errors]] | [[control#Match]] | [[traits]]

Enums give you a way of saying a value is one of a possible set of values, for example, IPv4 and IPv6.
Because these are the only possibilities for an IP address that our program will come across, we can enumerate all possible **variants**, which is where enumeration gets its name.

```rust
enum IpAddrKind {
    V4,
    V6,
}
```

## Enums vs Structs

We can create instances of each of the two **variants** of `IpAddrKind`, and define a function that takes in any of that type:

```rust
fn route(ip_kind: IpAddrKind) {}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    // And we can call this function with either variant:
    route(four);
    route(six);
}
```

We can also use enums to store values, so we can associate the kind with the value, rather than an enum inside a struct, we can put data directly into each enum variant.
This new definition of the `IpAddr` enum says that both `V4` and `V6` variants will have associated `String` values:

```rust
enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));

let loopback = IpAddr::V6(String::from("::1"));
```

Another advantage to using an enum rather than a struct is that each variant can have different types and amounts of associated data. Version four IP addresses will always have four numeric components that will have values between 0 and 255. If we wanted to store V4 addresses as four u8 values but still express V6 addresses as one String value, we wouldn’t be able to with a struct. Enums handle this case with ease:

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);

let loopback = IpAddr::V6(String::from("::1"));
```

## Embedding Types in Enum Variants

For the most control over the enum values, we can compose structs with enums. This opens up more flexibility as we can attach methods to structs to support behavior like supporting different encodings.

This example embeds the address data inside the variants in the form of two different structs, which are defined differently for each variant:

```rust
struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
```

Furthermore, we do the same with using another enum as a value for an enum variant.

A `Message` enum with a wide variety of types embedded in its variants:

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

## Matching

The `match` expression allows you to compare a value against a series of patterns and then execute code based on which pattern matches composed of. It does this using **match arms** that consists of a pattern and some code.

Matching allows you to extract the inner `T` values from enum variants and compare the enum type variants exhaustively. The basic pattern is:

- match against an enum
- bind a variable to the data inside
- execute code based on it

```rust

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}
```

## Matching with `Option<T>`

> See [[option]] for detailed coverage of `Option<T>` patterns

You can unwrap the inner `T` values using matching. For example, if you only want to do an operation on the inner value if there is `Some` value, you can do so in the match arm:

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
```

## Matching is Exhaustive

If the match arms are not exhaustive, you can use a special pattern for a catch-all that won't bind to that value; `_`:

```rust
let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => reroll(),
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn reroll() {}
```

This tells Rust we aren’t going to use the value, so Rust won’t warn us about an unused variable.

Another example using the `_` match arm to return the empty tuple unit value `()`:

```rust
let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => (),
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
```

---

## See Also
- [[option]] - The most important enum in Rust
- [[errors]] - `Result<T,E>` enum for error handling
- [[control]] - Pattern matching with `match` and `if let`
- [[traits]] - Implementing traits for enums

**Practice**: `exercises/08_enums/` | **Review**: [[rust-review-guide#Core Concepts Phase]]
