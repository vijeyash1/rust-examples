fn main() {
    let mut w = Vec::new();
    w.push(1);
    w.push(2);
    w.push(3);
    w.push(4);
    let first = &w[0];
    let second = w.get(1);
    println!("{} {:?}", first, second);
}
