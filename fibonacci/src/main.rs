use std::io;

fn main() {
    println!("Input the nth fibonacci number you want to calculate:");

    let mut fib = String::new();

    match io::stdin().read_line(&mut fib) {
            Ok(_) => {},
            Err(_) => {
                println!("Error in input, unknown fault");
                return;
            }
        };
    
    let fib: u32 = match fib.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a positive number, negative fibonnaci not supported");
            return;
        }
    };
    println!("Fibonnaci number {} is {}", fib, fibonacci(fib, 0, 1))

}

fn fibonacci(num: u32, first: u32, second: u32) -> u32{
    match num {
        0 => {return 0},
        1 => {return second},
        _ => { return fibonacci(num - 1, second, first + second) }
    }
}
