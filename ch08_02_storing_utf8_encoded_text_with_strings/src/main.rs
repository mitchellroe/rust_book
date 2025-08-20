#![allow(unused)]

fn main() {
    ch8_2();
}

fn ch8_2() {
    println!("## 8.2 Storing UTF-8 Encoded Texts with Strings"); // {{{

    println!("### What Is a String?"); // {{{
    // }}}

    println!("### Creating a New String"); // {{{
    {
        let mut s = String::new();
    }
    {
        let data = "initial contents";
        let s = data.to_string();

        // The method also works on a literal directly:
        let s = "initial contents".to_string();
    }
    {
        let s = String::from("initial contents");
    }
    {
        // Strings are UTF-8 encoded, so they can contain all sorts of characters.
        let hello = String::from("السلام عليكم");
        let hello = String::from("Dobrý den");
        let hello = String::from("Hello");
        let hello = String::from("שלום");
        let hello = String::from("नमस्ते");
        let hello = String::from("こんにちは");
        let hello = String::from("안녕하세요");
        let hello = String::from("你好");
        let hello = String::from("Olá");
        let hello = String::from("Здравствуйте");
        let hello = String::from("Hola");
    }
    // }}}

    println!("### Updating a String"); // {{{
    println!("#### Appending to a String with push_str and push"); // {{{
    {
        let mut s = String::from("foo");
        s.push_str("bar");
        dbg!(&s);
    }
    {
        // The `push_str` method appends a string slice
        let mut s1 = String::from("foo");
        dbg!(&s1);
        let s2 = "bar";
        dbg!(&s2);
        s1.push_str(s2);
        dbg!(&s1);
        // ... but it doesn't take ownership of it.
        dbg!(&s2);
    }
    {
        // The `push` method appends a single character.
        let mut s = String::from("lo");
        s.push('l');
        dbg!(&s);
    }
    // }}}

    println!("#### Concatenation with the + Operator or the format! Macro"); // {{{
    {
        let s1 = String::from("Hello, ");
        let s2 = String::from("world!");
        let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
        dbg!(s3);
    }

    {
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");
        // This is efficient, but unwieldy. It also takes ownership of s1, which may not be what
        // you want to do.
        let s = s1 + "-" + &s2 + "-" + &s3;
    }
    {
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");
        // The format! macro is much easier to read, and it doesn't take ownership of any of the
        // parameters along the way.
        let s = format!("{s1}-{s2}-{s3}");
    }
    // }}}
    // }}}

    println!("### Indexing into Strings"); // {{{

    // You cannot access individual characters of a String by their index like you can in other
    // programming languages.
    // The following is an error:
    // let s1 = String::from("hi");
    // let h = s1[0];

    println!("#### Internal Representation"); // {{{
    {
        // The following takes 4 bytes, one byte for each character.
        let hello = String::from("Hola");
        // The following is 24 bytes, 2 bytes for each character.
        let hello = String::from("Здравствуйте");

        // UTF-8 takes two bytes for each character. This is why trying to pull a character out of
        // the String via index is not valid: &hello[0] represents the 0th BYTE, not the 0th CHAR.
        // Thus, to avoid the confusion, Rust just refuses to compile this code and instead has you
        // use other methods to retrieve characters from Strings.
    }
    // }}}

    println!("#### Bytes and Scalar Values and Grapheme Clusters! Oh My!"); // {{{
    // Grapheme clusters are the closest thing to what we would call "letters". For example, there
    // are, in ascending order of abstraction:
    //   1. u8 values.
    //      [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]
    //   2. Characters, which might be "letters" or diacritics that get combined later.
    //      ['न', 'म', 'स', '्', 'त', 'े']
    //   3. Grapheme clusters, which are the final "letters" that might be recognized by a human
    //      reader.
    //      ["न", "म", "स्", "ते"]
    // }}}

    // }}}

    println!("### Slicing Strings"); // {{{
    {
        let hello = "Здравствуйте";
        let s = &hello[0..4];
        dbg!(s);
        // The following will result in a runtime error, since З is 2 bytes, and we're only
        // asking for the first byte.
        // let s = &hello[0..1];
    }
    // }}}

    println!("### Methods for Iterating Over Strings"); // {{{
    for c in "Зд".chars() {
        println!("{c}");
    }
    for b in "Зд".bytes() {
        println!("{b}");
    }
    // }}}

    println!("### Strings Are Not So Simple"); // {{{
    // }}}

    // }}}
}
