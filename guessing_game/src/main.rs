use std::io;
use rand::Rng;
use std::cmp::Ordering;

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new (value: i32) -> Guess{
        if value < 1 || value > 100 {
            panic!("Guess was out side range 1-100");
        }
        Guess {value}
    }

    // keeps value private and prevents bypassing validation by set explicitly
    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("Guess the number!");


    loop {
        println!("Input your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        println!("You guessed {}", guess);

        let guess: Guess = match guess.trim().parse() {
            Ok(num) => Guess::new(num),
            Err(_) => {
                println!("Please input a number");
                continue;
            },
        };

        match guess.value().cmp(&secret_number) {
            Ordering::Less => println!("Too low"),
            Ordering::Greater => println!("Too high"),
            Ordering::Equal => {
                println!("You win");
                break;
            },
        }
    }
}

