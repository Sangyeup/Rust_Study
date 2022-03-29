// Use cargo fmt to clean up code. Helps a lot!
use std::io::{self, Write};

const CONVERT: u8 = 1;
const FIB: u8 = 2;
const LYRICS: u8 = 3;

enum ErrorCode {
    NoError,
    User,
    WrongProg,
    WrongTemp,
    WrongFib,
    WrongCont,
    System,
}

fn main() {
    let exit_code = loop {
        let prog = get_prog();

        // Use matches!() for comparison between enums
        // or use #[derive(PartialEq)] above our defined enum.
        let prog = if matches!(prog.1, ErrorCode::NoError) {
            prog.0
        } else {
            break prog.1;
        };

        println!();
        match prog {
            CONVERT => {
                if !temp_conversion() {
                    break ErrorCode::WrongTemp;
                }
            }
            FIB => {
                if !get_fib() {
                    break ErrorCode::WrongFib;
                }
            }
            LYRICS => print_lyrics(),
            _ => break ErrorCode::WrongProg,
        };
    };

    println!();
    print!("[ExitCode: ");
    match exit_code {
        ErrorCode::User => println!("User terminated program]"),
        ErrorCode::System => println!("System Error]"),
        ErrorCode::WrongProg => println!("Wrong input for option number]"),
        ErrorCode::WrongTemp => println!("Wrong input for temperature conversion]"),
        ErrorCode::WrongFib => println!("Wrong input for Fibonacci]"),
        _ => println!("If this prints, there is something wrong with program]"),
    }
    println!();
}

fn get_prog() -> (u8, ErrorCode) {
    println!("┌─────────────────────────────────────────────────────────┐");
    println!("│ Which program do you want to run?                       │");
    println!("│ 1: Convert temperatures between Fahrenheit and Celsius  │");
    println!("│ 2: Generate the nth Fibonacci number                    │");
    println!("│ 3: Print lyrics to \"The Twelve Days of Christmas\"       │");
    println!("└─────────────────────────────────────────────────────────┘");
    print!("Input option number to execute program, or type quit to terminate: ");
    io::stdout().flush().ok().expect("Could not flush stdout"); // Flush needs to happen for stdin to work.

    let mut prog = String::new();

    match io::stdin().read_line(&mut prog) {
        Ok(_) => (),
        Err(_) => return (0, ErrorCode::System),
    };

    if prog.trim().eq("quit") {
        // trim() does not modify the string it is called on. It returns a new &string.
        return (0, ErrorCode::User);
    }

    let prog: u8 = match prog.trim().parse() {
        // So we have to call trim() here again.
        Ok(num) => num,
        Err(_) => return (0, ErrorCode::WrongProg),
    };

    (prog, ErrorCode::NoError)
}

fn temp_conversion() -> bool {
    let mut cont = true;
    while cont {
        println!("┌──────────────────────────────────────────────────────────────────────────────────────────┐");
        println!("│ Type desired temperature with a \"C\" or \"F\" suffix to auto convert it to its counterpart. │");
        println!("│ (e.g., 31C to convert 31 degrees Celsius to Fahrenheit.)                                 │");
        println!("└──────────────────────────────────────────────────────────────────────────────────────────┘");
        print!("Temperature to convert: ");
        io::stdout().flush().ok().expect("Could not flush stdout");

        let mut temp = String::new();
        io::stdin()
            .read_line(&mut temp)
            .expect("Could not read line.");
        temp = temp.trim().to_string(); // (sort-of) in-place trim

        let conv: char; // Apparently non-initialized, immutable variables can be given values later.

        // pop() method for string does exactly what we expect it to do.
        // But its return type is Option<char>, which can be either Some<char> or None.
        // It returns None when the string to be popped is empty.
        // So we have to take care of that.
        match temp.pop() {
            Some(c) => {
                if c == 'C' || c == 'F' {
                    conv = c;
                } else {
                    return false;
                }
            }
            None => return false,
        };

        let temp: f64 = match temp.parse() {
            Ok(num) => num,
            Err(_) => return false,
        };

        match conv {
            'C' => println!(
                "<{:.2} degrees Celsius is equal to {:.2} degrees Fahrenheit.>", // {:.2} defines the precision of the output.
                temp,
                temp * 9.0 / 5.0 + 32.0 // temp * 9 does not work. f64 variables have to be operated on with f64 literals.
            ),
            'F' => println!(
                "<{:.2} degrees Fahrenheit is equal to {:.2} degrees Celsius.>",
                temp,
                (temp - 32.0) * 5 as f64 / 9. // Can also use the "as" primitive conversion.
            ),
            _ => return false,
        };

        let cont_w_error = cont_run();
        cont = if matches!(cont_w_error.1, ErrorCode::NoError) {
            println!();
            cont_w_error.0
        } else {
            return false;
        };
    }
    true
}

fn get_fib() -> bool {
    let mut cont = true;
    while cont {
        print!("Type desired location of Fibonacci number: ");
        io::stdout().flush().ok().expect("Could not flush stdout");

        let mut n = String::new();
        io::stdin().read_line(&mut n).expect("Could not read line.");

        // Indexing has to be done with usize,
        // and we're going to use n for indexing later.
        // So we have to parse n into the usize type.
        let n: usize = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => return false,
        };

        if n == 1 || n == 2 {
            println!("<The {}th Fibonacci number is 1.>", n);
        } else {
            let mut fib = vec![1u64; 2]; // The vec![] macro creates a vector like an array.
            for i in 1..n - 1 {
                // This is why we needed to parse n into usize.
                fib.push(fib[i - 1] + fib[i]);
            }

            // Same with String.pop(), Vec.pop() also returns Option<?>.
            match fib.pop() {
                Some(num) => println!("<The {}th Fibonacci number is {}.>", n, num),
                None => return false,
            }
        }

        let cont_w_error = cont_run();
        cont = if matches!(cont_w_error.1, ErrorCode::NoError) {
            println!();
            cont_w_error.0
        } else {
            return false;
        };
    }
    true
}

fn print_lyrics() {
    let nth = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];
    let gifts = [
        "│ A partridge in a pear tree.    │",
        "│ Two turtle doves, and          │",
        "│ Three French hens,             │",
        "│ Four colly birds,              │",
        "│ Five gold rings,               │",
        "│ Six geese a laying,            │",
        "│ Seven swans a swimming,        │",
        "│ Eight maids a milking,         │",
        "│ Nine drummers drumming,        │",
        "│ Ten pipers piping,             │",
        "│ Eleven ladies dancing,         │",
        "│ Twelve lords a leaping,        │",
    ];

    println!("┌────────────────────────────────┐");

    // Similar to Python, enumerate() creates an iterator of tuples(position, value).
    for (pos, n) in nth.iter().enumerate() {
        // This part is used to make a placeholder string that creates a proper box.
        let mut end_string = String::new();
        // .chars().count() returns the length of the string.
        // Fun fact: this method runs in O(N) because Unicode is complicated.
        for _i in 0..9 - n.chars().count() {
            end_string.push(' ');
        }
        end_string.push('│');

        println!("│ The {} day of Christmas,{}", n, end_string);
        println!("│ My true love sent to me        │");
        for i in (0..=pos).rev() {
            println!("{}", gifts[i]); // println!(gifts[i]) does not work. Need to use the curly brackets.
        }

        if pos != 11 {
            println!("│                                │");
        }
    }
    println!("└────────────────────────────────┘");
    println!();
}

fn cont_run() -> (bool, ErrorCode) {
    println!();
    print!("Continue running this program? (y/n) ");
    io::stdout().flush().ok().expect("Could not flush stdout");

    let mut cont = String::new();
    match io::stdin().read_line(&mut cont) {
        Ok(_) => (),
        Err(_) => return (true, ErrorCode::WrongCont),
    }

    match cont.trim() {
        "y" | "Y" | "yes" | "Yes" => return (true, ErrorCode::NoError), // You can use | to group multiple patterns.
        "n" | "N" | "no" | "No" => return (false, ErrorCode::NoError),
        _ => return (true, ErrorCode::WrongCont),
    }
}
