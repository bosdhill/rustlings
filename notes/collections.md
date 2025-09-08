# Collections

## Iterators

1. For loop + iterators

```rust
// this won't actually iterate from 0 to n-1, it will just print
// [0..1] and [0..n-1]
// not sure why
for i in [0..n] {
    print!(i)
}
```

## iter().map(|v| => {}).collect()

This is reminiscent of Map => Reduce.

## Vectors

## Strings

## HashMaps

1. Create a new hashmap

    ```rust
    // HashMap needs to be included from the stdlib before it can be used
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    ```

2. Getting values

    ```rust
    let team_name = String::from("Blue");
    // handles the Option by calling copied to get an Option<i32> rather than an Option<&i32>, then unwrap_or to set score to zero if scores doesn’t have an entry for the key
    let score = scores.get(&team_name).copied().unwrap_or(0);
    ```

    The value is returned as an `Option<V>` to pattern match `Some(V)` or `None`.

3. Iterating

    ```rust
    for (key, value) in &scores {
        println!("{key}: {value}");
    }
    ```

4. Overwriting a value

    ```rust
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{scores:?}");
    ```

5. Adding key if not present

    Hash maps have a special API for this called entry that takes the key you want to check as a parameter. The return value of the entry method is an enum called Entry that represents a value that might or might not exist.

    ```rust
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    // The or_insert method on Entry is defined to return a mutable reference to the value for the corresponding Entry key if that key exists.
    //
    // If not, it inserts the parameter as the new value for this key and returns a mutable reference to the new value.
    //
    // This technique is much cleaner than writing the logic ourselves and, in addition, plays more nicely with the borrow checker.
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{scores:?}");
    ```

6. Updating value based on old value

    ```rust
    use std::collections::HashMap;

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    // If it hasn't been seen, it's zero
    // `entry(K)` returns a mutable reference `&mut V` that
    // can be safely dereferenced and modified.
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");
    ```

7. Swapping out Hashing function with `Hasher` trait

    If you profile your code and find that the default hash function is too slow for your purposes,
    you can switch to another function by specifying a different hasher.

    A hasher is a type that implements the BuildHasher trait.

### Ownership

Primitive types or types that implement the `Copy` trait will have their _values copied_ into the HashMap, while `Drop`
trait types will have their _ownership moved_ by default to the HashMap:

```rust
use std::collections::HashMap;

let field_name = String::from("Favorite color");
let field_value = String::from("Blue");

let mut map = HashMap::new();
map.insert(field_name, field_value);
// field_name and field_value are invalid at this point, try using them and
// see what compiler error you get!
```

If the references are used, then the references must be valid for the
_lifetime_ of the HashMap.

This would be a problem if the referenced value gets dropped when going
out of scope.
