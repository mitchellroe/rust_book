// Tuple Structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit-like structs
struct AlwaysEqual;

fn main() {
    // Tuple structs
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // Unit-like structs
    let subject = AlwaysEqual;
}
