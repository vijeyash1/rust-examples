// vector is a collection of elements of the same type
// vectors allow you to store a collection of data in a single variable.
#[allow(unused_variables)]
fn main() {
    let mut w = vec![2, 3, 4, 5];
    w.push(6);
    println!("{:?}", w);

    // push() adds an element to the end of the vector
    // pop() removes an element from the end of the vector
    // Vec::new creates an empty vector

    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    v.pop();
    println!("{:?}", v);
}
