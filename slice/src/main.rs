fn main() {
    let s = String::from("hello world");
    let word = first_word(&s[..]);

    let s1 = "world hello";
    let word1 = first_word(&s1);
    println!("{},{}", word, word1);
    // s.clear();   
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
