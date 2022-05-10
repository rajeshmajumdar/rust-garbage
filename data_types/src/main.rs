use std::io;

fn main() {
    //let tup: (i32, f64, u8) = (500, 6.4, 1);
    //let five_hundred: i32 = tup.0;
    //let six_point_four: f64 = tup.1;
    //let one: u8 = tup.2;

    //println!("Values are {}, {}, {}", five_hundred, six_point_four, one);
    
    //let a : [i32; 5] = [1,2,3,4,5];
    //let array_with_initialized_val = [3; 5];
    tuples();
    arrays();
    ask_input();

}

fn tuples() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let fh = tup.0;
    let sf = tup.1;
    let on = tup.2;

    println!("Values are {}, {}, {}", fh, sf, on);
}

fn arrays() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let array_with_initialized_value = [3; 5]; // Size = 5 and initialized value = 3
    println!("Second value of a: {}\nSecond value of initialized array: {}", a[1], array_with_initialized_value[1]);
}

fn ask_input() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Enter a index: ");
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read input.");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];
    println!("The value of element at {} is {}", index, element);
}

