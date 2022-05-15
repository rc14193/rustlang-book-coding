use std::collections::HashMap;

fn main() {
    let numbers = vec![1,1,2,3,4,4,4,5,6,0];
    let avg = mean(&numbers);
    let med = median(&numbers);
    let mode = mode(&numbers);
    println!("avg is {}", avg);
    println!("med is {}", med);
    println!("mode is {}", mode);
    let basic_str = "Then again free pen".to_string();
    let converted = convert_pig_latin(basic_str);
    println!("The converted sentence is {}", converted);
}

fn mean(v: &[i32]) -> f64 {
    let v_sum: i32 = v.iter().sum();
    v_sum as f64 / v.len() as f64
}

fn median(v: &[i32]) -> f64 {
    let mut sorted_nums = v.to_vec();
    sorted_nums.sort_unstable();
    println!("sorted vector is {:?}", sorted_nums);
    let middle: usize = sorted_nums.len() / 2;
    match sorted_nums.len() % 2 {
        0 => {
            0.5*(sorted_nums[middle] + sorted_nums[middle - 1]) as f64
        }
        _ => {
            sorted_nums[middle] as f64
        }
    }
}

fn mode(v: &Vec<i32>) -> i32{
    let mut map: HashMap<i32,i32> = HashMap::new();
    for num in v {
        let num_count = map.entry(*num).or_insert(0);
        *num_count += 1;
    }
    let max_value: i32 = *match map.values().max() {
        Some(val) => val,
        _ => &0,
    };
    match map.iter().find(|(_, &value)| value == max_value) {
        Some((_, val)) => *val,
        _ => 0
    }
}

fn convert_pig_latin(s: String) -> String {
    let lowered = s.to_lowercase();
    let words = lowered.split_whitespace();
    fn convert_word(s: &str) -> String {
        let first_letter = match s.chars().next() {
            Some(ltr) => ltr,
            _ => ' ',
        };
        let vowels = vec!['a', 'e', 'i', 'o', 'u'];
        match vowels.contains(&first_letter) {
            true => format!("{}-hay", s),
            false => {
                let mut chars = s.chars();
                chars.next();
                format!("{}-{}ay", chars.as_str(), first_letter)
            }
        }
    }
    let conversion = &words.map(|w| {convert_word(w)}).collect::<Vec<String>>().join(" ");
    conversion.to_string()
}