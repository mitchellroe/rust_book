use std::{thread, time};

fn main() {
    //loop {
    //    println!("again!");
    //}

    //let mut counter = 0;
    //let result = loop {
    //    counter += 1;
    //    if counter == 10 {
    //        break counter * 2;
    //    }
    //};
    //println!("The result is {result}");

    //let mut count = 0;
    //'counting_up: loop {
    //    println!("count = {count}");
    //    let mut remaining = 10;
    //    loop {
    //        println!("remaining = {remaining}");
    //        if remaining == 9 {
    //            break;
    //        }
    //        if count == 2 {
    //            break 'counting_up;
    //        }
    //        remaining -= 1;
    //    }
    //    count += 1;
    //}
    //println!("End count = {count}");

    //let mut number = 3;
    //while number != 0 {
    //    println!("{number}");
    //    let one_sec = time::Duration::from_secs(1);
    //    thread::sleep(one_sec);
    //    number -= 1;
    //}
    //println!("LIFTOFF!!!");

    //let a = [10, 20, 30, 40, 50];
    //for element in a {
    //    println!("the value is: {element}");
    //}

    // a..b only works when a < b.
    // Be careful with this if you're using runtime values.
    for number in (1..4).rev() {
        println!("{number}");
        let one_sec = time::Duration::from_secs(1);
        thread::sleep(one_sec);
    }
    println!("LIFTOFF!!!");

}
