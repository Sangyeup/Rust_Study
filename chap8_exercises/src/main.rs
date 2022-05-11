use std::collections::HashMap;
use std::io;
fn main() {
    let mut emp;
    let mut employees: HashMap<String, Vec<String>> = HashMap::new();
    let mut words = Vec::new();
    let mut depts = Vec::new();

    loop {
        emp = String::new();
        match io::stdin().read_line(&mut emp) {
            Ok(_) => (),
            Err(_) => return,
        };
        emp += " ";
        words.clear();
        let mut word = String::new();
        for char in emp.chars() {
            if char != ' ' {
                word += char.to_string().as_str();
            } else {
                words.push(word.trim().to_string());
                word = String::new();
            }
        }
        if words[0] == "1" {
            print_dept(&employees, &words[1]);
        } else if words[0] == "2" {
            let mut depts_clone = depts.clone();
            depts_clone.sort();
            depts_clone.dedup();
            for dept in depts_clone {
                print_dept(&employees, &dept)
            }
        } else if words[0] == "exit" {
            break;
        } else {
            let dept_vec = employees.entry(words[3].clone()).or_insert(Vec::new());
            dept_vec.push(words[1].clone());
            depts.push(words[3].clone());
        }
    };
}

fn print_dept(employees: &HashMap<String, Vec<String>>, dept: &String) {
    print!("Dept: {} - ", dept);
    match employees.get(dept) {
        Some(vec) => {
            let mut vec_clone = vec.clone();
            vec_clone.sort();
            println!("{:?}", vec_clone);
        }
        None => return
    }
}

// use std::collections::HashMap;
// fn main() {
//     let list = [3, 3, 5, 5, 5, 5, 4, 4, 8, 9, 10];
//     let mut v: Vec<i32> = Vec::from(list);
//     v.sort();
//     println!("{}", v[(v.len() - 1) / 2]);

//     let mut modes = HashMap::new();

//     for num in v {
//         let count = modes.entry(num).or_insert(0);
//         *count += 1;
//     }

//     println!("{}", modes.iter().max_by_key(|entry | entry.1).unwrap().0);
// }


// fn main() {
//     let word = "apple";
//     let mut pig = String::new();
//     let mut end = String::new();
//     let mut first_char = true;
//     for c in word.chars() {
//         if first_char {
//             if is_vowel(c) {
//                 end += "-hay";
//                 pig += c.to_string().as_str();
//             } else {
//                 end += "-";
//                 end += c.to_string().as_str();
//                 end += "ay";
//             }
//             first_char = false;
//         } else {
//             pig += c.to_string().as_str();
//         }
//     }
//     println!("{}", pig + end.as_str());
// }

// fn is_vowel(c: char) -> bool {
//     c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u'
// }
