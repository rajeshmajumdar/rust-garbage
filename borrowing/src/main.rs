fn main() {

    references();
    mut_refs();
}

fn references() {
    let s1 = String::from("Hemlo");
    let len = calculate_len(&s1);
    // here since we have sent a pointer to s1, we can use s1 later in the program.
    println!("String: {}, len: {}", s1, len);
}

fn calculate_len(s: &String) -> usize {
    s.len()
        // We aren't using ; at the end, because we want this value to be returned
}

//fn mut_refs() {
//    let s = String::from("Hello");
//    change(&s);

//}

//fn change(s: &String) {
//    s.push_str(", world!");
//}

// if we run the above func, it should throw us an error because we are getting an
// reference and then we are modifying, and just like variables are immutable by
// default in rust, references are also immutable. So for it to work we need to make
// it mutable.

fn mut_refs() {
    let mut s = String::from("Hello");
    change(&mut s);
}

fn change(s: &mut String) {
    s.push_str(", world!");
    println!("{}", s);
}

