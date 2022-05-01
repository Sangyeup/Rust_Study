use std::io::{self, Write};
use std::collections::HashMap;

struct EmployeeList {
    company: HashMap<String, Vec<String>>,
    is_sorted: bool,
}

impl EmployeeList {
    fn new() -> EmployeeList {
        EmployeeList {
            company: HashMap::new(),
            is_sorted: false,
        }
    }

    fn list(&mut self) {
        if !self.is_sorted {
            self.sort();
            self.is_sorted = true;
        }

        let copy = self.company.clone();
        let mut sorted: Vec<_> = copy.iter().collect();
        sorted.sort();
        for (department, employee) in sorted {
            print!("    {}:", department);
            for name in employee {
                print!(" {}", name);
            }
            println!();
        }
    }

    fn list_department(&mut self, department: &str) {
        match self.company.get(department) {
            Some(employee) => {
                print!("    {}:", department);
                for name in employee {
                    print!(" {}", name);
                }
                println!();
            }
            None => println!("    ! cannot find given department"),
        }
    }

    fn sort(&mut self) {
        for (_, employee) in &mut self.company {
            employee.sort();
        }
    }

    fn add(&mut self, name: &str, department: &str) {
        let employee = self.company.entry(String::from(department)).or_insert(Vec::new());
        employee.push(String::from(name));

        self.is_sorted = false;
    }

    fn delete(&mut self, target: &str) -> bool {
        let mut res = false;

        for (_, employee) in &mut self.company {
            let mut idx = 0;
            for name in employee.clone() {
                if name == target {
                    employee.remove(idx);
                    res = true;
                    break;
                }
                idx = idx + 1;
            }
        }

        res
    }
}

fn main() {
    println!("------------------------------------------");
    println!(" Employee list for departments in company ");
    println!("------------------------------------------");
    print_help();

    let mut employee_list = EmployeeList::new();
    loop {
        let (command, args) = match get_command() {
            Some(tup) => tup,
            None => continue,
        };

        match &command[..] {
            "list" => {
                let args = args.expect("argument check");
                if args.len() == 0 {
                    employee_list.list();
                } else if args.len() == 1 {
                    employee_list.list_department(&args[0]);
                } else {
                    println!("    ! invalid command, 0 or 1 arguments needed");
                }
            }
            "add" => {
                let args = args.expect("argument check");
                if args.len() != 3 {
                    println!("    ! invalid command, 3 arguments needed");
                    continue;
                }

                if args[1] == "to" {
                    employee_list.add(&args[0], &args[2]);
                } else {
                    println!("    ! invalid command, wrong argument");
                }
            }
            "delete" => {
                let args = args.expect("argument check");
                if args.len() != 1 {
                    println!("    ! invalid command, 1 argument needed");
                    continue;
                }
                
                if !employee_list.delete(&args[0]) {
                    println!("    ! cannot find employee with given name");
                }
            }
            "help" => {
                print_help();
            }
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
    println!("Type a command");
    println!("    list: List all employee in company");
    println!("    list <department>: List all employee in given department");
    println!("    add <name> to <department>: Add employee with given name to given department");
    println!("    delete <name>: Remove employee with given name from company");
    println!("    help: Print command list");
    println!("    exit: Terminate the program");
}

fn get_command() -> Option<(String, Option<Vec<String>>)> {
    print!("Command: ");
    io::stdout().flush().ok().expect("flush");
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("read_line");
    
    let mut iter = buf.split_whitespace();
    let command = String::from(iter.next()?);

    if command == "list" || command == "add" || command == "delete" {
        let mut args: Vec<String> = Vec::new();
        for arg in iter {
            args.push(String::from(arg));
        }

        Some((command, Some(args)))
    } else {
        Some((command, None))
    }
}
