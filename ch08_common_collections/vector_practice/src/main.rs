fn main() {
    println!("# Creating a new vector"); // {{{
    {
        // Option 1
        let _v: Vec<i32> = Vec::new();
        // Option 2
        let _v = vec![1, 2, 3];
    } // }}}

    println!("# Updating a Vector"); // {{{
    {
        // In order to append values, it needs to be mutable.
        let mut v = Vec::new();
        v.push(5);
        v.push(6);
        v.push(7);
        v.push(8);
    } // }}}

    println!("# Reading Elements of Vectors"); // {{{
    {
        {
            let v = vec![1, 2, 3, 4, 5];
            let third: &i32 = &v[2];
            println!("The third element is {third}");

            let third: Option<&i32> = v.get(2);
            match third {
                Some(third) => println!("The third element is {third}"),
                None => println!("There is no third element"),
            }
        }

        {
            let v = vec![1, 2, 3, 4, 5];
            // // The following will fail
            // let does_not_exist = &v[100];
            let does_not_exist = v.get(100);
            dbg!(does_not_exist);
        }

        // {
        // // The following will fail to compile because we have a mutable reference to a vector,
        // // followed by an immutable reference. We cannot have both.
        // let mut v = vec![1, 2, 3, 4, 5];
        // let first = &v[0];
        // v.push(6);
        // println!("The first element is {first}");
        // }
    } // }}}

    println!("# Iterating Over the Values in a Vector"); // {{{
    {
        let v = vec![100, 32, 57];
        for i in &v {
            println!("{i}");
        }
    } // }}}
}

// vim:foldmethod=marker:
