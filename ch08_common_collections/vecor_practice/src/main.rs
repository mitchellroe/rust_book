fn main() {
    {
        let _v: Vec<i32> = Vec::new();
        let _v = vec![1, 2, 3];
    }

    {
        let mut v = Vec::new();
        v.push(5);
        v.push(6);
        v.push(7);
        v.push(8);
    }

    {
        let v = vec![1, 2, 3, 4, 5];
        let v_third: &i32 = &v[2];
        println!("The third element is {v_third}");

        let v_third: Option<&i32> = v.get(2);
        match v_third {
            Some(third) => println!("The third element is {third}"),
            None => println!("There is no third element"),
        }
    }

    let v5 = vec![1, 2, 3, 4, 5];
    // let _does_not_exist = &v5[100];
    let _does_not_exist = v5.get(100);
    dbg!(_does_not_exist);

    let v6 = vec![100, 32, 57];
    for i in &v6 {
        println!("{i}");
    }

    let mut v7 = vec![100, 32, 57];
    for i in &mut v7 {
        *i += 50;
    }
}
