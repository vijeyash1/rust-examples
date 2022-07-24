fn main() {
    // creating string from literal
    let mut s = String::from("Hello ");
    // creating string with to_string() method.
    let mut t = "Hello ".to_string();

    // both styles can be used to create string.

    // appending string
    s.push_str("world");
    t.push_str("world");

    // push method takes a single character as argument.
    s.push('!');
    t.push('!');
    println!("{}", s);
    println!("{}", t);
}
