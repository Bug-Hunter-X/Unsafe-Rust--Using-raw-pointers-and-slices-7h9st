fn main() {
    let mut v = vec![1, 2, 3];
    let ptr = v.as_mut_ptr();
    // ... some code that modifies v ...
    unsafe {
        let slice = std::slice::from_raw_parts_mut(ptr, v.len());
        // ... use the slice ...
    }
}