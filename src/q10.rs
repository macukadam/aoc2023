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
    width: usize,
    height: usize,
}

impl Map {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
            main_loop: HashMap::new(),
            current_pipe: None,
            direction: Direction::N,
            width: 0,
            height: 0,
        }
    }

    fn add_start(&mut self, coordinates: (usize, usize), pipe: Pipe) {
        self.current_pipe = Some((coordinates, pipe));
        self.main_loop.insert(coordinates, pipe);
    }

    fn traverse(&mut self) -> usize {
        for direction in [Direction::N, Direction::E, Direction::S, Direction::W].iter() {
            if self.move_next(*direction) {
                break;
            }
        }

        let mut count = 1;
        while let Some((_, pipe)) = self.current_pipe {
            if let Pipe::Start = pipe {
                break;
            } else {
                self.move_next(self.direction);
                count += 1;
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

    fn load_map(lines: Lines<BufReader<File>>) -> Map {
        let mut map = Map::new();

        let mut height = 0;

        for (i, line) in lines.enumerate() {
            let line = line.unwrap();
            if i == 0 {
                map.width = line.len();
            }

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
            height += 1;
        }

        map.height = height;
        map
    }

    fn ray_beam_method(&mut self) -> usize {
        self.traverse();

        let mut crossed = 0;
        let mut count = 0;

        for i in 0..self.height {
            for j in 0..self.width {
                if self.main_loop.get(&(i, j)).is_some() {
                    continue;
                }

                // ray beams to the east
                for z in j..self.width {
                    if let Some(pipe) = self.main_loop.get(&(i, z)) {
                        match pipe {
                            Pipe::NS | Pipe::NW | Pipe::NE | Pipe::Start => {
                                crossed += 1;
                            }
                            _ => {}
                        }
                    }
                }

                if crossed % 2 == 1 {
                    count += 1;
                }
                crossed = 0;
            }
        }

        count
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
    let mut map = Map::load_map(lines);
    let count = map.traverse();
    println!("Part 1: {}", count / 2);
}

pub fn part2(lines: Lines<BufReader<File>>) {
    let mut map = Map::load_map(lines);
    let count = map.ray_beam_method();
    println!("Part 2: {}", count);
}
