// // This won't compile because we're trying to return a reference to a variable which is scoped
// // within the `dangle` function. We can't return a reference to a variable which has gone out of
// // scope.
// fn main() {
//     let reference_to_nothing = dangle();
// }
//
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }

// Instead, we should just return a value, not a reference
fn main() {
    let reference_to_something = no_dangle();
    println!("{reference_to_something}");
}

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}
