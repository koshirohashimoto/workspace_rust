fn main() {
    let mut s = String::from("foo");
    s.push_str("bar");

    let s1 = String::from("Hello");
    let s2 = String::from(" World");
    let s3 = String::from("!");
    let s  = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);
}
