fn main() {
    strings();
    string_copy();
    func_ownership();
}

fn strings() {
    let mut s1 = String::from("Hello");
    s1.push_str(", world!");
    println!("{}", s1);
}

fn string_copy() {
    let s1 = String::from("Hello");
    let s2 = s1;
    // Here by rust design, s1 is already out of scope, now if we
    // try to use s1 now, the compiler would throw an error.
    // Because String::from() method returns a pointer to the memory
    // containing the string, and when we copied it to s2, we copied
    // the address of the string in the heap.
    // To prevent double-free bug from happening, s1 goes out of scope
    // Kinda like s2=s1, is moving the value instead of copying, like in
    // other languages.
    //println!("{}", s1);
    println!("{}", s2); // only s2 is valid now.
    let s4 = String::from("Second");
    let s3 = s4.clone();
    // To copy it, we use the clone() method.
    println!("{} {}", s3, s4);
}

fn func_ownership() {
    let s = String::from("Took ownership");
    take_ownership(s);
    
    // variable 's' is out of scope now since s is moved to the function
    // we shouldn't be able to use it anymore.
    let x = 5;
    make_copy(x);

    // but here, we are copying the value of x and moving it to another function
    // so we should still be able to use it after.
    println!("{} used once.", x);
}

fn take_ownership(s: String) {
    println!("{}", s);
}

fn make_copy(x: i32) {
    println!("{} just copied from above.", x);
}


