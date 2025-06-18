// fn main() {
// let s1 = String::from("hello");
// let len = calculate_length(&s1);
//
// println!("The length of '{s1}' is {len}.");

// let x: u8 = b'*';
// println!("{x}");
// }

// fn calculate_length(s: &String) -> usize {
//     // s is a reference to a String
//     s.len()
// }
// Here, s goes out of scope. But because s does not have ownership of what it refers to, the value
// is not dropped.

// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }

// fn main() {
//     let mut s = String::from("hello");
//     println!("The initial value: {s}");
//     change(&mut s);
//     println!("And the new value: {s}");
// }
//
// fn change(some_string: &mut String) {
//     println!("We were given: {some_string}");
//     some_string.push_str(", world");
//     println!("And now it's: {some_string}");
// }

// // Will not compile. Cannot create multiple mutable references within the same scope.
// fn main() {
//     let mut s = String::from("hello");
//     let r1 = &mut s;
//     let r2 = &mut s;
//     println!("{} {}", r1, r2);
// }

// fn main() {
//     let mut s = String::from("hello");
//     {
//         let r1 = &mut s;
//     } // r1 goes out of scope here, so we can make a new reference with no problems
//     let r2 = &mut s;
// }

// fn main() {
//     // You can create multiple "normal" references, OR you can create ONE mutable reference. But
//     // you cannot create a mix of the two.
//     let mut s = String::from("hello");
//     let r1 = &s; // no problem
//     let r2 = &s; // no problem
//     let r3 = &mut s; // BIG PROBLEM
//     println!("{}, {}, and {}", r1, r2, r3);
// }

fn main() {
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{r1} and {r2}");
    // variables r1 and r2 will not be used after this point
    let r3 = &mut s;
    r3.push_str(", world");
    println!("{r3}");
}
