use std::{
    fs::File,
    io::{BufReader, Lines},
};

pub fn part1(lines: Lines<BufReader<File>>) {
    let mut times: Vec<u64> = Vec::new();
    let mut distances: Vec<u64> = Vec::new();

    for (i, line) in lines.enumerate() {
        let line = line.unwrap();
        let second_half = line.split_once(':').unwrap().1;

        if i == 0 {
            times = second_half
                .split(' ')
                .filter_map(|x| x.parse::<u64>().ok())
                .collect::<Vec<u64>>();
        } else {
            distances = second_half
                .split(' ')
                .filter_map(|x| x.parse::<u64>().ok())
                .collect::<Vec<u64>>();
        }
    }

    product(times, distances);
}

pub fn part2(lines: Lines<BufReader<File>>) {
    let mut times: Vec<u64> = Vec::new();
    let mut distances: Vec<u64> = Vec::new();

    for (i, line) in lines.enumerate() {
        let line = line.unwrap();
        let second_half = line.split_once(':').unwrap().1.replace(' ', "").parse::<u64>().unwrap();
        println!("{}", second_half);

        if i == 0 {
            times = vec![second_half];
        } else {
            distances = vec![second_half];
        }
    }

    product(times, distances);
}

fn product(times: Vec<u64>, distances: Vec<u64>) {
    let mut vec = Vec::new();

    for (i, time) in times.into_iter().enumerate() {
        let mut move_sum = 0;
        for t in 1..time {
            let speed = t;
            let time = time - t;

            let move_dist = speed * time;

            if move_dist > distances[i] {
                move_sum += 1;
            }
        }

        vec.push(move_sum);
    }

    let mutliplied = vec.iter().product::<i32>();

    println!("Multiplied: {}", mutliplied);
}
