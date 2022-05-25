use std::io;
fn main() {
    
    println!("Type desired temperature with a suffix to convert it in to its counterpart: ");
    let mut temp = String::new();
    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line");
    temp = temp.trim().to_string();

    let conv: char;
    
    match temp.pop() {
        Some(c) => {
            if c == 'C' || c == 'F' {
                conv = c;
            } else {
                return ();
            }
        }
        None => return (),
    };
    
    let temp: f64 = match temp.parse() {
        Ok(num) => num,
        Err(_) => return (),
    };

    match conv {
        'C' => println!(
            "<{:.2} degrees Celsius is equal to {:.2} degrees Fahrenheit.>",
            temp,
            temp * 9.0 / 5.0 + 32.0
        ),
        'F' => println!(
            "<{:.2} degrees Fahrenheit is equal to {:.2} degrees Celsius.>",
            temp,
            (temp - 32.0) * 5 as f64 / 9.
        ),
        _ => return (),
    };
}