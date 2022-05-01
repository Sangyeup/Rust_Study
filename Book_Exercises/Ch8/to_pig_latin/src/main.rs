use std::io::{self, Write};

fn main() {
    println!("------------------------------");
    println!(" Convert strings to pig latin ");
    println!("------------------------------");
    println!("Input white space or just enter to terminate");

    loop {
        print!("String: ");
        io::stdout().flush().ok().expect("flush");

        let mut buf = String::new();
        io::stdin().read_line(&mut buf).expect("read_line");
    
        let mut chars = buf.trim().chars();
        match chars.next() {
            Some(c) => {
                print!("     -> ");
                if is_vowel(c) {
                    println!("{}{}-hay", c, chars.as_str());
                } else {
                    println!("{}-{}ay", chars.as_str(), c);
                }
            }
           None => {
                println!("Terminate");
                break;
            }
        }
    }
}

fn is_vowel(c: char) -> bool {
    if c == 'a' || c == 'A' || c == 'e' || c == 'E' || c == 'i' || c == 'I' || c == 'o' || c == 'O' || c == 'u' || c == 'U' 
    || c == 'Α' {
        true
    } else {
        false
    }
}