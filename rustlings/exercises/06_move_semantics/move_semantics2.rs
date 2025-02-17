fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    // Parameter 'vec' takes ownership of the passed vector
    // We shadow the immutable 'vec' with a mutable version
    let mut vec = vec;

    // Modify the vector we own
    vec.push(88);

    // Return ownership of the modified vector to the caller
    vec
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    // This test will fail because of ownership rules!
    #[test]
    fn move_semantics2() {
        // Create vec0 - we own it here
        let vec0 = vec![22, 44, 66];

        // vec0's ownership is moved to fill_vec's parameter
        // After this line, vec0 is no longer valid in this scope!
        let vec1 = fill_vec(vec0.clone());

        // This will fail - vec0's ownership was moved
        assert_eq!(vec0, [22, 44, 66]); // ❌ Error: vec0 was moved
                                        // This works - we own vec1 now
        assert_eq!(vec1, [22, 44, 66, 88]);
    }
}
