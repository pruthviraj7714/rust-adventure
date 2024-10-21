fn main() {
    //ownership
    let s = String::from("Hello, Rust!"); // `s` owns the String.
    println!("{}", s); // `s` is still valid here.

    //borrowing
    let s1 = String::from("Hello");
    let s2 = s1;

    // println!("{}", s1); // This would cause a compile error since `s1` is no longer valid.
    println!("{}",s2);


} // `s` goes out of scope here, and the memory is automatically freed.

//Borrowing 

fn main() {
    let s = String::from("Borrow me!");
    let len = calculate_length(&s); // Pass `s` by reference (borrowed).
    println!("Length of '{}': {}", s, len); // `s` is still usable since ownership wasn't moved.
}

fn calculate_length(s: &String) -> usize { // Takes a reference.
    s.len() // Accessing the length of the string.
}

//mutable borrowing
fn main() {
    let mut s = String::from("Change me!");
    change(&mut s); // Mutable borrow.
    println!("{}", s); // The String is now changed.
}

fn change(s: &mut String) {
    s.push_str(" Modified!"); // Modify the string.
}


S1 after the move
fn main() {
    let s1 = String::from("Ownership moved");
    let r = &s1; // Borrow `s1`
    
    let s2 = s1; // Ownership of `s1` moves to `s2`
    
    println!("{}", r); // Compile error: `s1` no longer valid, so `r` is also invalid.
    println!("{}", s2); // `s2` owns the value now.
}



