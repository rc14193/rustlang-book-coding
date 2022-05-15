fn main() {
    let mut s = String::from("hello");
    s.push_str(", world");
    let length = calculate_length(&s);
    println!("{} and length is {}",s, length);

}

fn calculate_length(s: &String) -> usize {
    s.len()
}
