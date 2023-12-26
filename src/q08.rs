use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Lines},
};

enum Direction {
    Left,
    Right,
}

impl Direction {
    fn from_char(c: char) -> Direction {
        match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Invalid direction"),
        }
    }
}

#[derive(Debug)]
struct Map {
    map: HashMap<String, (String, String)>,
}

impl Map {
    fn new() -> Map {
        Map {
            map: HashMap::new(),
        }
    }

    fn add(&mut self, start: String, left: String, right: String) {
        self.map.insert(start, (left, right));
    }
}

pub fn part1(lines: Lines<BufReader<File>>) {
    let mut directions: Vec<Direction> = Vec::new();
    let mut map = Map::new();

    for (i, line) in lines.enumerate() {
        let line = line.unwrap();

        if i == 0 {
            for c in line.chars() {
                directions.push(Direction::from_char(c));
            }
            continue;
        }

        if line.is_empty() {
            continue;
        }

        let splits: Vec<&str> = line
            .split_terminator(&['=', ',', '(', ')', ' '][..])
            .filter(|x| !x.is_empty())
            .collect();

        map.add(
            splits[0].to_string(),
            splits[1].to_string(),
            splits[2].to_string(),
        );
    }

    let mut start_node = map.map.get("AAA").unwrap();
    let mut steps = 0;
    let mut step_out = false;

    while !step_out {
        for direction in &directions {
            steps += 1;
            match direction {
                Direction::Left => {
                    if start_node.0 == "ZZZ" {
                        step_out = true;
                        break;
                    }
                    start_node = map.map.get(&start_node.0).unwrap();
                }
                Direction::Right => {
                    if start_node.1 == "ZZZ" {
                        step_out = true;
                        break;
                    }
                    start_node = map.map.get(&start_node.1).unwrap();
                }
            }
        }
    }

    println!("Steps: {}", steps);
}

pub fn part2(lines: Lines<BufReader<File>>) {
    let mut directions: Vec<Direction> = Vec::new();
    let mut map = Map::new();
    let mut starting_points: Vec<String> = Vec::new();

    for (i, line) in lines.enumerate() {
        let line = line.unwrap();

        if i == 0 {
            for c in line.chars() {
                directions.push(Direction::from_char(c));
            }
            continue;
        }

        if line.is_empty() {
            continue;
        }

        let splits: Vec<&str> = line
            .split_terminator(&['=', ',', '(', ')', ' '][..])
            .filter(|x| !x.is_empty())
            .collect();

        if splits[0].to_string().ends_with('A') {
            starting_points.push(splits[0].to_string());
        }

        map.add(
            splits[0].to_string(),
            splits[1].to_string(),
            splits[2].to_string(),
        );
    }

    let mut counts: Vec<u128> = vec![0; starting_points.len()];

    for (i, starting_point) in starting_points.iter().enumerate() {
        let mut step_out = false;
        let mut current_point = starting_point.clone();

        while !step_out {
            for direction in &directions {
                counts[i] += 1;
                match direction {
                    Direction::Left => {
                        current_point = map.map.get(&current_point).unwrap().0.clone();
                        if current_point.ends_with('Z') {
                            step_out = true;
                            break;
                        }
                    }
                    Direction::Right => {
                        current_point = map.map.get(&current_point).unwrap().1.clone();
                        if current_point.ends_with('Z') {
                            step_out = true;
                            break;
                        }
                    }
                }
            }
        }
    }

    println!("{}", lcm(&counts));
}

fn lcm(v: &Vec<u128>) -> u128 {
    v.iter().fold(1, |a, &b| num::integer::lcm(a, b))
}
