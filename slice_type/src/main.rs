fn main() {
    let hello = String::from("Hello world!");
    println!("first word is {}", first_word(&hello));
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[..i];
        }
    }
    return &s[..];
}