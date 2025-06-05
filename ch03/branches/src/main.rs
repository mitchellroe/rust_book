//use std::io;

fn main() {
    //let number = 7;
    //if number < 5 {
    //    println!("condition was true");
    //} else {
    //    println!("condition was false");
    //}

    ////let number = 6;
    //let mut number = String::new();
    //println!("Please enter a number");
    //io::stdin().read_line(&mut number).expect("Failed to read line");
    //let number: i32 = number.trim().parse().expect("Failed to convert to a number!");
    //if number % 4 == 0 {
    //    println!("number is divisible by 4");
    //} else if number % 3 == 0 {
    //    println!("number is divisble by 3");
    //} else if number % 2 == 0 {
    //    println!("number is divislbe by 2");
    //} else {
    //    println!("number is not divisible by 4, 3, or 2");
    //}

    //let condition = true;
    //let number = if condition { 5 } else { 6 };
    //println!("The value of number is: {number}");

    let condition = true;
    let number = if condition { 5 } else { "six" };
    println!("The value of number is: {number}");
}
