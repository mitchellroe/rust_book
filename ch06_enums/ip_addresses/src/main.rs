// enum IPAddrKind {
//     V4,
//     V6,
// }
//
// struct IPAddr {
//     kind: IPAddrKind,
//     address: String,
// }
//
// fn main() {
//     let four = IPAddrKind::V4;
//     let six = IPAddrKind::V6;
//
//     route(four);
//     route(six);
//     route(IPAddrKind::V4);
//
//     let home = IPAddr {
//         kind: IPAddrKind::V4,
//         address: String::from("127.0.0.1"),
//     };
//
//     let loopback = IPAddr {
//         kind: IPAddrKind::V6,
//         address: String::from("::1"),
//     };
// }
//
// fn route(_ip_kind: IPAddrKind) {}

#[derive(Debug)]
enum IPAddr {
    V4(String),
    V6(String),
}

fn main() {
    let home = IPAddr::V4(String::from("127.0.0.1"));
    let loopback = IPAddr::V6(String::from("::1"));
    dbg!(home);
    dbg!(loopback);
}
