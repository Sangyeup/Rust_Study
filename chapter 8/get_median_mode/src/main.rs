use std::io;
use std::collections::HashMap;

fn main() {
    println!("This is average, median, mode computing program.");
    println!("Enter more than five numbers!, (delimiter=space)");
    println!("example) 1 5 4 12 10 12 3");
    
    let mut input_str = String::new();

    io::stdin().read_line(&mut input_str).expect("Failed to read line");
    
    //println!("{:?}", input_str);

    let input_str = input_str.trim().split(" ");
    let mut vec_int: Vec<i32> = input_str.map(|x| x.parse::<i32>().unwrap()).collect();

    //println!("{:?}", vec_str);

    println!("AVERAGE: {}", average(&vec_int));
    println!("MEDIAN: {}", median(&mut vec_int));
    println!("MODE: {}", mode(&vec_int));
}

fn average(numbers: &Vec<i32>) -> f32 {
    numbers.iter().sum::<i32>() as f32 / numbers.len() as f32
}

fn median(numbers: &mut Vec<i32>) -> i32 {
    numbers.sort();
    let mid = numbers.len() / 2;
    numbers[mid]
}

fn mode(numbers: &Vec<i32>) -> i32 {
    let mut occurr = HashMap::new();

    for &value in numbers {
        *occurr.entry(value).or_insert(0) += 1;
    }
    
    occurr
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(val, _)| val)
        .expect("Cannot compute the mode of zero numbers")
}