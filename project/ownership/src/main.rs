fn main() {
    let s = String::from("Hello World");
    let word = first_word(&s[..]);
    println!("The first word is '{}'.", word);

    let s_literal = "hello world";
    let word = first_word(&s_literal[..]);
    println!("The first word_literal is '{}'.", word);

    let word = first_word(s_literal);
    println!("The first word_direct is '{}'.", word);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}