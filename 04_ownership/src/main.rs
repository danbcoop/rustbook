/*
* Rules of ownership
* - Each value in Rust has an owner
* - There can only be one owner at a time 
* - When the owner goes out of scope, the value will be dropped
*/

fn main() {
    let mut s1 = String::from("hello");  // create String from string literal
    
    s1.push_str(", world!");             // String type can be mutated

    // Move s1 into s2. This invalidates s1. (shallow copy and invalidation)
    let mut s2 = s1;

    // The clone() method allows for deep copying (expensive)
    let s1 = s2.clone();

    // New assignments will free previously owned memory via drop()
    s2 = String::from("ahoy");
    
    // Copy trait annotates types stored on the stack
    let x = 1;
    let y = x;
    println!("{x} = {y}");

    /* 
    * References
    * - At any given time, you can have either immutable references of exactly one mutable
    * reference
    * - References must be valid
    */

    // Using a reference, we can access s1 without taking ownership
    let _len =  calculate_length(&s1);
    // Mutable references (&mut foo) can have no other references to that value!
    println!("{s1} and {s2}!");

    let reference_to_nothing = dangle();
}

// Borrowing s
fn calculate_length(s: &String) -> usize {
    s.len()
}

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}

// Resturn String slice of the first word
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
