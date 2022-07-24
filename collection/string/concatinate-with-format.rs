// concatination of two strings using format! macro
fn main() {
    let s1 = "tic".to_string();
    let s2 = "tac".to_string();
    let s3 = "toe".to_string();
    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("{}", s);
    let t1 = "tic".to_string();
    let t2 = "tac".to_string();
    let t3 = "toe".to_string();
    let t = format!("{}-{}-{}", t1, t2, t3);

    println!("{}", t);
}
