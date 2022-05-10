use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input.");

    let number: u32 = input
        .trim()
        .parse()
        .expect("Input is not a number.");

    fibonacci(number);
}

fn fibonacci(mut x: u32) {
    let mut first: u32 = 0;
    let mut second: u32 = 1;
    println!("1");
    while x > 1 {
        let result = {
            first + second
        };
        first = second;
        second = result;
        println!("{}", result);
        x -= 1;
    }
}
        
