fn main() {
    // let mut s = String::from("hello");
    // s.push_str(", world!"); // push_str() appends a literal to a String
    // println!("{s}");

    // // Assigning two pointers to the same heap object. The original pointer gets deleted.
    // let s1 = String::from("hello");
    // let s2 = s1;
    // println!("{s1}, world!");

    // // Shadowing does the opposite. The original data in the heap gets deleted.
    // let mut s = String::from("hello");
    // s = String::from("ahoy");
    // println!("{s}, world!");

    // // Cloning
    // let s1 = String::from("hello");
    // let s2 = s1.clone();
    // println!("s1 = {s1}, s2 = {s2}");

    // s comes into scope
    let s = String::from("hello");
    // s's value moves into the function
    takes_ownership(s);
    // ... and so is no longer valid here

    // x comes into scope
    let x = 5;
    // because i32 implements the Copy trait,
    // x does NOT move into the function,
    makes_copy(x);
    // so it's okay to use x afterward
    println!("{x}");
}
// Here, x goes out of scope, then s. But because s's value was moved, nothing special happens.

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{some_string}");
}
// Here, some_string goes out of scope and 'drop' is called. The backing memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{some_integer}");
}
// Here, some_integer goes out of scope. Nothing special happens.
