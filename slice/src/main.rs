fn main() {
    slices();

}



fn slices() {
    let s = String::from("Hello world");
    let first_word = &s[..5];
    let second_word = &s[6..11];
    println!("Juggling some words: {} {}", second_word, first_word);
}

// Well that's all about the slicing, we can use this to all types of collections
// of data, for e.g. arrays and all. 


        
