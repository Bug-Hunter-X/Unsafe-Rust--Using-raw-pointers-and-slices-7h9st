fn main() {
    let mut v = vec![1, 2, 3];
    // Instead of using raw pointers and unsafe code, we can use iterators to safely modify the vector.
    for i in v.iter_mut() {
        *i *= 2; 
    }
    println!("Modified vector: {:?}", v); 
} 