use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Lines},
};

struct Map {
    map: HashMap<(usize, usize), Pipe>,
    main_loop: HashMap<(usize, usize), Pipe>,
    current_pipe: Option<((usize, usize), Pipe)>,
    direction: Direction,
}

impl Map {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
            main_loop: HashMap::new(),
            current_pipe: None,
            direction: Direction::N,
        }
    }

    fn add_start(&mut self, coordinates: (usize, usize), pipe: Pipe) {
        self.current_pipe = Some((coordinates, pipe));
        self.main_loop.insert(coordinates, pipe);
    }

    fn traverse(&mut self) -> usize {
        let directions = [Direction::N, Direction::E, Direction::S, Direction::W];

        for direction in directions.iter() {
            let found = self.move_next(*direction);
            if found {
                break;
            }
        }

        let mut count = 1;
        while let Some((_, pipe)) = self.current_pipe {
            match pipe {
                Pipe::Start => {
                    break;
                }
                _ => {
                    self.move_next(self.direction);
                    count += 1;
                }
            }
        }

        count
    }

    fn move_next(&mut self, direction: Direction) -> bool {
        let (x, y) = self.current_pipe.unwrap().0;
        let next_pipe: Option<&Pipe>;
        let coordinates: (usize, usize);

        match direction {
            Direction::N => {
                coordinates = (x - 1, y);
                next_pipe = self.map.get(&coordinates);
            }
            Direction::E => {
                coordinates = (x, y + 1);
                next_pipe = self.map.get(&coordinates);
            }
            Direction::S => {
                coordinates = (x + 1, y);
                next_pipe = self.map.get(&coordinates);
            }
            Direction::W => {
                coordinates = (x, y - 1);
                next_pipe = self.map.get(&coordinates);
            }
        }

        if let Some(next_pipe) = next_pipe {
            match next_pipe {
                Pipe::None => {
                    return false;
                }
                _ => {
                    self.current_pipe = Some((coordinates, *next_pipe));
                    self.main_loop.insert(coordinates, *next_pipe);
                    self.direction = match (*next_pipe, direction) {
                        (Pipe::NS, Direction::N) => Direction::N,
                        (Pipe::NS, Direction::S) => Direction::S,
                        (Pipe::SE, Direction::N) => Direction::E,
                        (Pipe::SE, Direction::W) => Direction::S,
                        (Pipe::SW, Direction::N) => Direction::W,
                        (Pipe::SW, Direction::E) => Direction::S,
                        (Pipe::EW, Direction::E) => Direction::E,
                        (Pipe::EW, Direction::W) => Direction::W,
                        (Pipe::NE, Direction::S) => Direction::E,
                        (Pipe::NE, Direction::W) => Direction::N,
                        (Pipe::NW, Direction::S) => Direction::W,
                        (Pipe::NW, Direction::E) => Direction::N,
                        (Pipe::Start, _) => Direction::N,
                        _ => panic!("Invalid pipe"),
                    };

                    return true;
                }
            }
        }
        false
    }
}

#[derive(Debug, Clone, Copy)]
enum Pipe {
    NS,
    EW,
    NE,
    NW,
    SE,
    SW,
    Start,
    None,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    N,
    E,
    S,
    W,
}

pub fn part1(lines: Lines<BufReader<File>>) {
    let mut map = Map::new();

    for (i, line) in lines.enumerate() {
        let line = line.unwrap();

        for (j, char) in line.chars().enumerate() {
            let (coordinates, pipe) = match char {
                'S' => {
                    let pipe = Pipe::Start;
                    map.add_start((i, j), pipe);
                    ((i, j), pipe)
                }
                '|' => ((i, j), Pipe::NS),
                '-' => ((i, j), Pipe::EW),
                'L' => ((i, j), Pipe::NE),
                'J' => ((i, j), Pipe::NW),
                '7' => ((i, j), Pipe::SW),
                'F' => ((i, j), Pipe::SE),
                '.' => ((i, j), Pipe::None),
                _ => {
                    panic!("Invalid char");
                }
            };

            map.map.insert(coordinates, pipe);
        }
    }

    let count = map.traverse();
    println!("Part 1: {}", count / 2);
}

