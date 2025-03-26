// TODO: Fix the compiler error in the function without adding any new line.
// 2. Pass mutable reference to a vector
fn fill_vec(vec: &mut Vec<i32>) -> Vec<i32> {
    // 3. Modify the vector
    vec.push(88);

    // 4. Return a copy of the vector
    vec.clone()
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_semantics3() {
        // 1. Create a mutable reference to a vector 
        let vec0 = &mut vec![22, 44, 66];
        let vec1 = fill_vec(vec0);
        assert_eq!(vec1, [22, 44, 66, 88]);
    }
}
