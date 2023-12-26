use std::{
    fs::File,
    io::{BufReader, Lines},
};

struct Map {
    map: Vec<Vec<Pipe>>,
    current_pipe: Option<Pipe>,
    direction: Direction,
}

impl Map {
    fn new() -> Self {
        Self {
            map: Vec::new(),
            current_pipe: None,
            direction: Direction::N,
        }
    }

    fn add_line(&mut self, line: Vec<Pipe>) {
        self.map.push(line);
    }

    fn add_start(&mut self, pipe: Option<Pipe>) {
        self.current_pipe = pipe;
    }

    fn set_next_pipe(&mut self, direction: Direction) -> bool {
        let (x, y) = match self.current_pipe.unwrap() {
            Pipe::NS(x, y) => (x, y),
            Pipe::EW(x, y) => (x, y),
            Pipe::NE(x, y) => (x, y),
            Pipe::NW(x, y) => (x, y),
            Pipe::SE(x, y) => (x, y),
            Pipe::SW(x, y) => (x, y),
            Pipe::Start(x, y) => (x, y),
            Pipe::None(x, y) => (x, y),
        };

        let next_pipe: Option<&Pipe>;
        match direction {
            Direction::N => {
                next_pipe = self.map.get(x - 1).and_then(|x| x.get(y));
                if let Some(next_pipe) = next_pipe {
                    match next_pipe {
                        Pipe::NS(_, _) => {
                            self.current_pipe = Some(*next_pipe);
                            self.direction = Direction::N;
                            return true;
                        }
                        Pipe::SE(_, _) => {
                            self.current_pipe = Some(*next_pipe);
                            self.direction = Direction::E;
                            return true;
                        }
                        Pipe::SW(_, _) => {
                            self.current_pipe = Some(*next_pipe);
                            self.direction = Direction::W;
                            return true;
                        }
                        Pipe::Start(_, _) => {
                            self.current_pipe = Some(*next_pipe);
                            self.direction = Direction::N;
                            return true;
                        }
                        _ => {
                            return false;
                        }
                    }
                }
            }
            Direction::E => {
                next_pipe = self.map.get(x).and_then(|x| x.get(y + 1));
                if let Some(next_pipe) = next_pipe {
                    match next_pipe {
                        Pipe::EW(_, _) => {
                            self.current_pipe = Some(*next_pipe);
                            self.direction = Direction::E;
                            return true;
                        }
                        Pipe::NW(_, _) => {
                            self.current_pipe = Some(*next_pipe);
                            self.direction = Direction::N;
                            return true;
                        }
                        Pipe::SW(_, _) => {
                            self.current_pipe = Some(*next_pipe);
                            self.direction = Direction::S;
                            return true;
                        }
                        Pipe::Start(_, _) => {
                            self.current_pipe = Some(*next_pipe);
                            self.direction = Direction::N;
                            return true;
                        }
                        _ => {
                            return false;
                        }
                    }
                }
            }
            Direction::S => {
                next_pipe = self.map.get(x + 1).and_then(|x| x.get(y));
                if let Some(next_pipe) = next_pipe {
                    match next_pipe {
                        Pipe::NS(_, _) => {
                            self.current_pipe = Some(*next_pipe);
                            self.direction = Direction::S;
                            return true;
                        }
                        Pipe::NE(_, _) => {
                            self.current_pipe = Some(*next_pipe);
                            self.direction = Direction::E;
                            return true;
                        }
                        Pipe::NW(_, _) => {
                            self.current_pipe = Some(*next_pipe);
                            self.direction = Direction::W;
                            return true;
                        }
                        Pipe::Start(_, _) => {
                            self.current_pipe = Some(*next_pipe);
                            self.direction = Direction::N;
                            return true;
                        }
                        _ => {
                            return false;
                        }
                    }
                }
            }
            Direction::W => {
                next_pipe = self.map.get(x).and_then(|x| x.get(y - 1));
                if let Some(next_pipe) = next_pipe {
                    match next_pipe {
                        Pipe::EW(_, _) => {
                            self.current_pipe = Some(*next_pipe);
                            self.direction = Direction::W;
                            return true;
                        }
                        Pipe::NE(_, _) => {
                            self.current_pipe = Some(*next_pipe);
                            self.direction = Direction::N;
                            return true;
                        }
                        Pipe::SE(_, _) => {
                            self.current_pipe = Some(*next_pipe);
                            self.direction = Direction::S;
                            return true;
                        }
                        Pipe::Start(_, _) => {
                            self.current_pipe = Some(*next_pipe);
                            self.direction = Direction::N;
                            return true;
                        }
                        _ => {
                            return false;
                        }
                    }
                }
            }
        }
        false
    }
}

#[derive(Debug, Clone, Copy)]
enum Pipe {
    NS(usize, usize),
    EW(usize, usize),
    NE(usize, usize),
    NW(usize, usize),
    SE(usize, usize),
    SW(usize, usize),
    Start(usize, usize),
    None(usize, usize),
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

        let mut pipes = Vec::new();

        for (j, char) in line.chars().enumerate() {
            let pipe = match char {
                'S' => {
                    let pipe = Pipe::Start(i, j);
                    map.add_start(Some(pipe));
                    pipe
                }
                '|' => Pipe::NS(i, j),
                '-' => Pipe::EW(i, j),
                'L' => Pipe::NE(i, j),
                'J' => Pipe::NW(i, j),
                '7' => Pipe::SW(i, j),
                'F' => Pipe::SE(i, j),
                '.' => Pipe::None(i, j),
                _ => {
                    panic!("Invalid char");
                }
            };

            pipes.push(pipe);
        }

        map.add_line(pipes);
    }

    let directions = [Direction::N, Direction::E, Direction::S, Direction::W];

    for direction in directions.iter() {
        let found = map.set_next_pipe(*direction);
        if found {
            break;
        }
    }

    let mut count = 0;
    for _ in 0..100000 {
        count += 1;
        match map.current_pipe.unwrap() {
            Pipe::Start(_, _) => {
                break;
            }
            _ => {
                map.set_next_pipe(map.direction);
            }
        }

    }

    println!("Count: {}", count / 2);
}
