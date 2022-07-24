fn main() {
    let mut w = vec![1, 4, 7, 9, 10];
    for i in &mut w {
        i *= 10;
    }
    println!("{:?}", w);
}
