fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&mut s[..]);

    let mut s1 = "world hello";
    let word1 = first_word(&mut s1);
    s.clear();
    println!("{},{}", word, word1);
}

fn first_word(s: &mut str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
