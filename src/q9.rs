use std::{
    fs::File,
    io::{BufReader, Lines},
};

pub fn part1(lines: Lines<BufReader<File>>) {
    let mut sum = 0;
    for line in lines {
        let line = line.unwrap();

        let values = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let val = recurse_values(values);
        sum += val;
    }

    println!("Sum: {}", sum);
}

pub fn part2(lines: Lines<BufReader<File>>) {
    let mut sum = 0;
    for line in lines {
        let line = line.unwrap();

        let values = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let val = recurse_values_part2(values);
        sum += val;
    }

    println!("Sum: {}", sum);
}

pub fn recurse_values(values: Vec<i32>) -> i32 {
    if values.iter().all(|&x| x == 0) {
        return 0;
    }

    let mut new_values = Vec::new();
    for i in 1..values.len() {
        new_values.push(values[i] - values[i - 1]);
    }

    let val = recurse_values(new_values);
    return val + values.last().unwrap();
}

pub fn recurse_values_part2(values: Vec<i32>) -> i32 {
    if values.iter().all(|&x| x == 0) {
        return 0;
    }

    let mut new_values = Vec::new();
    for i in 1..values.len() {
        new_values.push(values[i] - values[i - 1]);
    }

    let val = recurse_values_part2(new_values);
    return values.first().unwrap() - val;
}
