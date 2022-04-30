use std::io::{self, Write};
use std::collections::HashMap;

struct List {
    integers: Vec<i32>,
    is_sorted: bool,
    is_changed: bool,
    median: i32,
    mode: i32,
}

impl List {
    fn new() -> List {
        List {
            integers: Vec::new(),
            is_sorted: false,
            is_changed: true,
            median: 0,
            mode: 0,
        }
    }

    fn sort(&mut self) {
        self.integers.sort();
        self.is_sorted = true;
    }

    fn print(&self) {
        print!("    List:");
        if self.integers.len() == 0 {
            println!(" (empty)");
            return;
        }
        for int in &self.integers {
            print!(" {}", int);
        }
        println!();
    }


    fn find_median_mode(&mut self) -> Option<(i32, i32)> {
        let len = self.integers.len();
        if len == 0 {
            return None;
        }

        if self.is_changed {
            // get median
            let mut list = &self.integers;
            // if not sorted, make sorted clone
            let mut sorted_integers: Vec<i32>;
            if !self.is_sorted {
                sorted_integers = self.integers.clone();
                sorted_integers.sort();
                list = &sorted_integers;
            }
            self.median = list[(len-1)/2];

            // get mode
            let mut mode = self.integers[0];
            let mut max_counts = 1;
            let mut counts = HashMap::new();
            for int in &self.integers {
                let count = counts.entry(int).or_insert(0);
                *count += 1;

                if max_counts < *count {
                    max_counts = *count;
                    mode = *int;
                }
            }
            self.mode = mode;
            
            self.is_changed = false;
        }
        
        Some((self.median, self.mode))
    }


    fn add(&mut self, ints: Vec<i32>) {
        for int in ints {
            self.integers.push(int);
        }
        self.is_sorted = false;
        self.is_changed = true;
    }

    fn delete(&mut self, mut indices: Vec<i32>) {
        indices.sort();
        indices.reverse();
        for index in indices {
            match self.integers.get(index as usize) {
                Some(_) => self.integers.remove(index as usize),
                None => {
                    println!("    ! index {} is out of bound", index);
                    continue;
                }
            };
        }
    }
}

fn main() {
    println!("-------------------------------------------");
    println!(" Find median & mode from a list of integer ");
    println!("-------------------------------------------");
    println!("Type a command");
    print_help();

    let mut list = List::new();
    loop {
        let (command, args) = match get_command() {
            Some(tup) => tup,
            None => continue,
        };
        
        match &command[..] {
            "list" => {
                list.print();
            } 
            "sort" => {
                list.sort();
                list.print();
            }
            "run" => {
                match list.find_median_mode() {
                    Some((median, mode)) => {
                        println!("    median: {}", median);
                        println!("    mode: {}", mode);
                    }
                    None => println!("    ! list is empty"),
                }
            }
            "add" => {
                let args = match args {
                    Some(v) => v,
                    None => {
                        println!("    ! invalid command, type arguments");
                        continue;
                    }
                };
                list.add(args);
                list.print();
            }
            "delete" => {
                let args = match args {
                    Some(v) => v,
                    None => {
                        println!("    ! invalid command, type arguments");
                        continue;
                    }
                };
                list.delete(args);
                list.print();
            }
            "help" => print_help(),
            "exit" => {
                println!("    Terminate program");
                break;
            }
            _ => {
                println!("    ! invalid command: {}", command);
            }

        }

    }
}

fn print_help() {
    println!("    list: List current elements is a list");
    println!("    sort: Sort a list in ascending order");
    println!("    run: Find median and mode");
    println!("    add <integers>: Add integers separated by white spaces in a list");
    println!("    delete <indices>: Remove elements at indices separated by white spaces from a list");
    println!("    help: Print command list");
    println!("    exit: Terminate the program");
}

fn get_command() -> Option<(String, Option<Vec<i32>>)> {
    print!("Command: ");
    io::stdout().flush().expect("flush");
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("read_line");
    
    let mut iter = buf.split_whitespace();
    let command = String::from(iter.next()?);

    if command == "add" || command == "delete" {
        let mut args: Vec<i32> =  Vec::new();
        for arg in iter {
            match arg.parse() {
                Ok(i) => args.push(i),
                Err(_) => println!("    ! \"{}\" is not an integer", arg),
            };
        }
        
        Some((command, Some(args)))
    } else {
        Some((command, None))
    }
}
