use std::io;

fn main() {
    
    println!("Enter a number");
    let mut number = String::new();
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read input");

    let number: u32 = number
        .trim()
        .parse()
        .expect("Input was not a number.");

    if number < 5 {
        println!("{} is smaller than 5.", number);
    } else if number == 5 {
        println!("Number is 5");
    } else {
        println!("{} is greater than 5.", number);
    }

}


