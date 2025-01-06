fn main() {
    
    let s1 = String::from("hello"); // `s1` owns the string.
    let mut  s2 = s1; // Ownership is moved to `s2`.
    // println!("{}", s1); // Error: `s1` is no longer valid.

    let mut s3 = s2.clone(); // Explicitly clone the string.
    println!("{}", s3); // `s3` owns a copy.

    s2 = "hello1".to_string();
    borrow_example(&mut s3); // Borrow the string (does not take ownership).
    println!("{}", s3); // `s3` is still valid here.
    println!("{}",s2)
}

fn borrow_example(s: &mut String) {
    println!("Borrowed string: {}", s);
    s.push_str(", world")
}
