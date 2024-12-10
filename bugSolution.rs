fn main() {
    let mut v = vec![1, 2, 3];
    // Instead of using raw pointers, use safe methods to modify the vector.
    v[0] = 10; // This is the safe and recommended way to modify the vector
    println!("The first element is: {}", v[0]);
}