#[allow(unused)]

fn main() {
    println!("# 8.3 Storing Keys with Associated Values in Hash Maps");

    println!("## Creating a New Hash Map");
    {
        use std::collections::HashMap;
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);
    }

    println!("## Accessing Values in a Hash Map");
    {
        use std::collections::HashMap;

        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);

        // Just practicing with Option<V>
        {
            // unwrap_or(V) provides a value if the Option<T> is None.
            let score = scores.get("Yellow").copied().unwrap_or(0);
            dbg!(score);
        }

        {
            let score = match scores.get("Yellow").copied() {
                Some(score) => score,
                None => 0,
            };
            dbg!(score);
        }

        {
            if let Some(score) = scores.get("foo").copied() {
                println!("Got a value");
                dbg!(score);
            } else {
                println!("Failed to get a value");
            }
        }
    }

    {}
}
