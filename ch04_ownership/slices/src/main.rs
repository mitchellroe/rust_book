// fn main() {
//     let s = String::from("hello there world");
//     let first_space_index = first_word(&s);
//     println!("The location of the first space character is {first_space_index}");
//     println!("So the first word of {s} is {}", &s[0..first_space_index]);
// }
//
// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }
//     s.len()
// }

// // Another example
// fn main() {
//     let my_word = String::from("hello world");
//     let first_word = first_word(&my_word);
//     println!("The first word in {my_word} is {first_word}");
//     let second_word = second_word(&my_word);
//     println!("The second word in {my_word} is {second_word}");
// }
//
// fn first_word(s: &str) -> &str {
//     let bytes = s.as_bytes();
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }
//     &s[..]
// }
//
// fn second_word(s: &str) -> &str {
//     let first_word = first_word(&s);
//     return &s;
// }

fn main() {
    let my_string = String::from("hello world");
    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent to whole slices of
    // `String`s
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
