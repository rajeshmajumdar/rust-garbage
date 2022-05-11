fn main() {

    references();
    mut_refs();
    multiple_mut();
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

fn multiple_mut() {
    let mut s = String::from("Hemlo");
    //let r1 = &mut s;
    //let r1 = &s1;
    //let r2 = &mut s;
    // this code should throw us an error since we are creating multiple mutable refs of the same
    // piece of data. In rust, we are only allowed to make one mutable reference to particular
    // piece of data at a time.
    // Second piece of code would also fail compiling since, here we are first creating a immutable
    // refs and later we are creating a mutable refs, and rust doesn't allow that

    //println!("{} {}", r1, r2);
    
    let r1 = &s;
    let r2 = &s;
    println!("We are using the immutable refs here.: {} {}", r1, r2);

    let r3 = &mut s;
    println!("Now we should be able to create a mutable refs.: {}", r3);

}
