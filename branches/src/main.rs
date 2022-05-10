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
    
    loops();
    returning_loop();
    conditional_loop();
    for_loop();
}


fn loops() {
    let mut count: u32 = 0;
    'counting_up: loop {
        println!("count= {}", count);
        let mut remaining: u32 = 10;

        loop {
            println!("remaining= {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count= {}", count);
}

fn conditional_loop() {
    let mut counter: u32 = 3;
    while counter != 0 {
        println!("{}", counter);
        counter -= 1;
    }
    println!("BOOM!");
}

fn returning_loop() {
    let mut count: u32 = 0;
    let res = loop {
        count += 1;
        if count == 10 {
            break count * 2;
        }
    };
    println!("The result is {}", res);
}

fn for_loop() {
    let a: [u32; 5] = [1, 2, 3, 4, 5];
    for element in a {
        println!("Element is {}", element);
    }
}
