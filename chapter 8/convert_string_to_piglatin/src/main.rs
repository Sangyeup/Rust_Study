use std::io;

fn main() {
    println!("This is a English -> Pig Latin Translator.");
    println!("Enter a sentence.");
    println!("ex) Let's go to the movies");
    
    let mut input_sent = String::new();

    io::stdin().read_line(&mut input_sent).expect("Failed to read line");
    
    let input_sent = input_sent.trim().split(" ");
    
    // println!("{:?}", input_sent);

    let vowels = vec!['a', 'i', 'o', 'u', 'e'];

    for s in input_sent {
        let first_char = &s.chars().next().unwrap();
        if vowels.contains(first_char) {
            print!("{}", format!("{}-hay ", s));
        }
        else {
            let temp = &s[1..]; // is this code okay..?
            let new_word = format!("{}-{}ay", temp, first_char);
            print!("{} ", new_word);
        }
    }
}
