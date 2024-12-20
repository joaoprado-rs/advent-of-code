use std::{fs, usize};

fn main() {
    let contents = fs::read_to_string("./src/input.txt").unwrap();
    let sum = cauculate_distance(contents);
    println!("sum: {}", sum);
}

fn find_lowest_number_and_remove(vector: &mut Vec<(usize, &str)>) -> usize {
    let (min_index, _) = vector
        .iter()
        .enumerate()
        .min_by_key(|(_, (_, s))| s.parse::<usize>().unwrap())
        .unwrap();
    let (_, number) = vector.remove(min_index);
    let lowest = number.parse::<usize>().unwrap();
    lowest
}

fn cauculate_distance(file: String) -> isize {
    let mut locations_first: Vec<(usize, &str)> = file
        .split_whitespace()
        .enumerate()
        .filter(|x| x.0 % 2 == 0)
        .collect();

    let mut locations_second: Vec<(usize, &str)> = file
        .split_whitespace()
        .enumerate()
        .filter(|x| x.0 % 2 != 0)
        .collect();

    let mut sum = 0;
    let mut lowest_first;
    let mut lowest_second;
    while !locations_first.is_empty() && !locations_second.is_empty() {
        lowest_first = find_lowest_number_and_remove(&mut locations_first);
        lowest_second = find_lowest_number_and_remove(&mut locations_second);
        sum = sum + absolute((lowest_first as isize) - (lowest_second as isize));
    }
    sum
}

fn absolute(number: isize) -> isize {
    if number < 0 {
        -number
    } else {
        number
    }
}
