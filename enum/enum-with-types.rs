// enum varients can have differnt types

enum Ipaddr {
    V4(u8, u8, u8, u8),
    V6(String),
}
fn main() {
    let home = Ipaddr::V4(127, 0, 0, 1);
    let loopback = Ipaddr::V6(String::from("::1"));
}
