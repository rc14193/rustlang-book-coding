use std::collections::HashMap;

fn main() {
    //explicit
    let v: Vec<i32> = Vec::new();

    //infered
    let v = vec![2,3,4,5,6];

    let mut v_mut = Vec::new();
    v_mut.push(5);
    v_mut.push(88);

    let data = "init contents";

    let s = data.to_string();

    let s = "init contents".to_string();

    let mut hash = HashMap::new();

    hash.insert("Blue", 10);

    let text = "This is a sentence of text that shows the ability to update a value in a hash map based on what was there.";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

}
