fn main() {

    references();

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
    
