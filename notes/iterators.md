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

> **Note:** v1.iter() yields _immutable references_ (`&i32`) to the items in the vector, not owned values.
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

## Iterator Methods

### Iterator Consumers

Methods that call `next` are called *consuming adapters*, because calling those methods "use up" the iterator.

Calling a *consuming adapter* method may take ownership of the iterator.

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

The call to `sum` takes ownership of the iterator, so it can't be used after.

### Iterator Adapters

Iterator adapters are defined on the `Iterator` trait and don't consume the iterator, which means they don't call the `next` method.

Instead, they produce different iterators by changing something of the original one, or *adapting* it.

One example is `map` that takes a closure to call on each item as the items are iterated:
``` rust
    let v1: Vec<i32> = vec![1, 2, 3];
    v1.iter().map(|x| x + 1); // prints a warning since it isn't consumed, just defined
```

It doesn't consume the values, changes *how* the iterator operates on the values. `map` will return a new iterator that will produce the modified items.

We can consume the iterator using the `collect` method that collects the resulting values into a collection data type:
``` rust
    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);
```

Closures allow you to customize the behavior while reusing the iteration behavior of the `Iterator` trait, and you can chain multiple iterators. Since they are lazy, you just need to call one of the consuming iterator adapters.

This would be useful for data processing on the fly.

## Closures

Many iterator adapters take closures as arguments, and commonly the closures we’ll specify as arguments to iterator adapters will be *closures that capture their environment*.

For example, `filter` takes a closure.

We can use the closure to get an item and return a `bool`, `true` to include it in the iteration, and `false` to ignore it.

This example filters a vector of `Shoe`s and filters them by those that are equal to a `shoe_size`.

``` rust
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
	// collect and return owned values with into_iter
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}
```
