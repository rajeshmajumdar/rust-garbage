fn main() {
    simple_func();
    func_with_param(5);
    func_with_multiple_param(5, 'r');
    weird_notation();
    let x = func_with_return();
    println!("Returned value: {}", x);
}

fn simple_func() {
    println!("This is just a simple function.");
}

fn func_with_param(x: i32) {
    println!("The value of param is: {}", x);
}

fn func_with_multiple_param(x: i32, y: char) {
    println!("This is a multiple param: {}, {}", x, y);
}

fn weird_notation() {
    let y = {
        let x = 3;
        x + 1 // No semicolon here, since adding a semicolon would make it a statement, and statements doesn't return anything
    };

    println!("Weird function: {}", y);
}

fn func_with_return() -> i32 {
    5
}


