#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect = Rectangle {
        width: 10,
        height: 10,
    };

    //area(&rect);
    //println!("The area of the rect. is {}", area(&rect));
    println!("Rect: {:?}", rect);
    println!("Area from impl: {}", rect.area());
}

fn area(rect: &Rectangle) -> u32 {
    //println!("The area of the rect. is {}", rect.width * rect.height);
    rect.width * rect.height
}

// We are using {:?} to print out debug information in the println! macro, we can
// also use dbg! macro instead to acheive the same result, the only difference is
// in println! we are outputting it through stdout, while dbg! macro output it 
// through the stderr, which might come in handy somewhere.
