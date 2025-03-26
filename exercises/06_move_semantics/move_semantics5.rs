#![allow(clippy::ptr_arg)]

// TODO: Fix the compiler errors without changing anything except adding or
// removing references (the character `&`).

// Shouldn't take ownership
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();

    println!("{data}");
}

fn main() {
    let data = "Rust is great!".to_string();

    // 1. This passes a immutable reference, so won't take ownership
    get_char(&data);

    // 2. This will pass a mutable reference implicitly, since string doesn't implement Copy trait, so only allows for
    // "shallow copies".
    string_uppercase(data);
}
