#[allow(unused)]

fn main() {
    ch8_3();
}

fn ch8_3() {
    println!("## 8.3 Storing Keys with Associated Values in Hash Maps"); // {{{

    println!("### Creating a New Hash Map"); // {{{
    {
        use std::collections::HashMap;
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);
    }
    // }}}

    println!("### Accessing Values in a Hash Map"); // {{{
    {
        use std::collections::HashMap;
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);
        let team_name = String::from("Blue");
        let score = scores.get(&team_name).copied().unwrap_or(0);
        dbg!(team_name);
        dbg!(score);
    }
    // }}}

    // }}}
}
