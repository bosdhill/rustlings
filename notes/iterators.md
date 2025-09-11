# Advanced Iterators & Functional Programming

#iterators #closures #functional-programming #iterator-trait
## Overview

Rust [iterators](https://doc.rust-lang.org/std/iter/) allow you to work with sequences of items, or [[collections]].

Rust iterators are lazy, so have no effect until you "consume" the iterator.

For example, this won't do any anything:
``` rust
let v1 = vec![1, 2, 3];
let v1_iter = v1.iter(); // no consumer, so does nothing
```

for loops implicitly create an iterator, so these are equivalent:
``` rust
// implicit iterator
let v1 = vec![1, 2, 3];
for val in v1 {
    println!("Got: {val}")
}

// explicit iterator
let v1_iter = v1.iter();
for val in v1_iter {
    println!("Got: {val}")
}
```

## Iterator Trait and `next` Method

All iterators implement the `Iterator` trait:
``` rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // methods with default implementations elided
}
```

- `type Item` and `Self::Item` define an associated type with the `Iterator` trait.
- Implementing the `Iterator` trait requires defining an `Item` type that is returned by the `next` method.
- Iteration ends when `None` is returned, otherwise it is expected to return `Some`.

## `next` Method and Mutability

In order to call the `next` method on an iterator, it needs to be declared *mutable* because each call advances its internal state.

``` rust
    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];

        let mut v1_iter = v1.iter();

		// This code consumes aka uses up the iterator.
		// Each call "eats up" an item from the iterator.
        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }
```

When used in a for loop, the loop takes ownership of the iterator and makes it implicitly _mutable_.

> **Note:** v1.iter() yields _immutable references_ (&i32) to the items in the vector, not owned values.
## Forms of Iteration

There are three common methods for creating iterators from [[collections]]:
- The `iter` method iterates over *immutable references* (`&T`).
- The `iter_mut` method iterates over *mutable references* (`&mut T` ).
-  The `into_iter` method iterates over *owned values* ( `T`).

| method      | iteration type         | ownership     | example                                                             |
| ----------- | ---------------------- | ------------- | ------------------------------------------------------------------- |
| `iter`      | iterates over `&T`     | immutable ref | <code class="language-rust">let v1_iter = v1.iter();</code>         |
| `iter_mut`  | iterates over `&mut T` | mutable ref   | <code class="language-rust">let mut v1_iter = v1.iter_mut();</code> |
| `into_iter` | iterates over `T`      | owned value   | <code class="language-rust">let v1_iter = v1.into_iter();</code>    |

> **Note**: `into_iter` takes ownership of the collection. A clearer name might have been `iter_owned`, but Rust conventionally uses `into_` to mean _by-value_.

See [[ownership#Concept Reference Table|Ownership Reference Table]].

## Consuming Adapters

Methods that call `next` are called *consuming adapters*, because calling those methods "uses up" the iterator.

One example is the [`sum` method](https://doc.rust-lang.org/std/iter/trait.Sum.html#required-methods).

``` rust
#[cfg(test)]
mod tests {
    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];

        // create immutable references with iter
        let v1_iter = v1.iter();

        // sum "consumes" the iterator here
        let total: i32 = v1_iter.sum();

        assert_eq!(total, 6);
    }
}
```
