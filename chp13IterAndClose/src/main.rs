use std::thread;
use std::time::Duration;
use std::collections::HashMap;

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: HashMap<u32, u32>,
}

impl<T> Cacher<T> 
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T>{
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value.get(&arg) {
            Some(v) => *v,
            None => {
                let result = (self.calculation)(arg);
                self.value.insert(arg, result);
                result
            }
        }
    }

}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;
    generate_workout(simulated_user_specified_value, simulated_random_number);

    let v1 = vec![1, 2, 3];

    let v2 = v1.iter().map(|x| x+1);

    for v in v2{
        println!("{}", v)
    }

    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num: u32| { simulated_expensive_calculation(num) });
    if intensity < 25 {
        println!("Today, do {} pushups",
        expensive_result.value(intensity));

        println!("Next, do {} sit ups",
        expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break for today.");
        } else {
            println!("Today, run for {} minutes",
            expensive_result.value(intensity))
        }
    }
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter{
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item>{
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        }else {
            None
        }
    }
}