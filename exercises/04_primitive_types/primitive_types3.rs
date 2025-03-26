fn main() {
    // TODO: Create an array called `a` with at least 100 elements in it.
    let a = [100; 100];

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed");
    }
    let a = [1..10];
    for i in 0..a.len() {
        println!("Value is {}", i)
    }
}
