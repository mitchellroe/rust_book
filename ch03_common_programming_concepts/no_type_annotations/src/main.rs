fn main() {
    let x = 3;
    println!("x is set to {x}");

    let x = x + 4;
    println!("x is set to {x}");

    let x = x + 4;
    println!("x is set to {x}");

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 99.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = 5 / 5;
    println!("truncated is {truncated}");

    // tuples (fixed size, multiple data types)
    let my_tup: (i32, f64, u8) = (500, 6.4, 1);
    // tuple deconstruction
    let (x, y, z) = my_tup;
    // You can also use dot notation
    let six_point_four = my_tup.1;

    // arrays
    //   fixed size, but must be the same data type
    let a = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April", "May", "June", "July", "August",
    "September", "October", "November", "December"];
    // specifying a data type and size
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    // auto-generating an array with a certain value
    let a = [3; 5]; // Creates [3, 3, 3, 3, 3]
    // Accessing array elements
    let first = a[0];
    let second = a[1];
}
