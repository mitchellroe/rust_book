fn main() {
    let mut s = String::from(" hello there with leading and trailing spaces ");
    println!("{}", s.trim());

    s = String::from("hello there     with inside spaces");
}
