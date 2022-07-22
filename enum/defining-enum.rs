#[allow(dead_code)]
#[derive(Debug)]
enum Ipaddrkind {
    V4,
    V6,
}

struct Ipaddr {
    kind: Ipaddrkind,
    address: String,
}

impl Ipaddr {
    fn new(kind: Ipaddrkind, address: &str) -> Ipaddr {
        Ipaddr {
            kind: kind,
            address: address.to_string(),
        }
    }
}

fn main() {
    let home = Ipaddr::new(Ipaddrkind::V4, "127.0.0.1");
    let work = Ipaddr::new(Ipaddrkind::V6, "::1");

    println!("Home address: {}", home.address);
    println!("work address: {}", work.address);
    println!("home Ipaddr kind: {:#?}", home.kind);
    println!("work Ipaddr kind: {:#?}", work.kind);
}
