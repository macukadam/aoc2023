use std::{
    fmt::Display,
    fs::File,
    io::{BufReader, Lines},
};

#[derive(Clone)]
struct Springs {
    state: Vec<Vec<State>>,
    broken: Vec<Vec<usize>>,
}

impl Springs {
    fn permute_by_index(self, index: usize) -> Vec<Vec<State>> {
        let mut new_states = vec![self.state[index].clone()];
        let mut new_states_out: Vec<Vec<State>> = vec![];

        while new_states
            .iter()
            .any(|s| s.iter().any(|s| *s == State::QuestionMark))
        {
            if let Some(mut new_state) = new_states.pop() {
                if new_state.iter().all(|s| *s != State::QuestionMark) {
                    new_states_out.push(new_state.clone());
                    continue;
                }

                for i in 0..new_state.len() {
                    if new_state[i] == State::QuestionMark {
                        new_state[i] = State::Dot;
                        new_states.push(new_state.clone());
                        new_state[i] = State::Pound;
                        new_states.push(new_state.clone());
                        break;
                    }
                }
            }
        }

        new_states_out.append(&mut new_states);

        new_states_out
    }

    fn count(&self, index: usize) -> usize {
        let permutated_states = self.clone().permute_by_index(index);
        let mut total_count = 0;

        for states in permutated_states {
            let mut counts = Vec::new();
            let mut count = 0;

            for state in &states {
                match state {
                    State::Pound => count += 1,
                    State::Dot => {
                        if count > 0 {
                            counts.push(count);
                        }
                        count = 0;
                    }
                    _ => {
                        panic!("There should be no question marks");
                    }
                }
            }

            if count > 0 {
                counts.push(count);
            }

            if counts == self.broken[index] {
                total_count += 1;
            }
        }

        total_count
    }
}

impl Display for Springs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.state.len() {
            for j in 0..self.state[i].len() {
                write!(f, "{}", self.state[i][j])?;
            }

            for j in 0..self.broken[i].len() {
                write!(f, " {}", self.broken[i][j])?;
            }

            writeln!(f).unwrap();
        }

        writeln!(f)
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum State {
    QuestionMark,
    Dot,
    Pound,
}

impl State {
    fn from_char(c: char) -> Self {
        match c {
            '?' => State::QuestionMark,
            '.' => State::Dot,
            '#' => State::Pound,
            _ => panic!("Unknown state"),
        }
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            State::QuestionMark => '?',
            State::Dot => '.',
            State::Pound => '#',
        };
        write!(f, "{}", c)
    }
}

impl Springs {
    fn new() -> Self {
        Springs {
            state: Vec::new(),
            broken: Vec::new(),
        }
    }

    fn add_line(&mut self, states: Vec<State>, broken: Vec<usize>) {
        self.state.push(states);
        self.broken.push(broken);
    }
}

pub fn part1(lines: Lines<BufReader<File>>) {
    let mut springs = Springs::new();
    for line in lines {
        let line = line.unwrap();
        let (first, second) = line.split_once(' ').unwrap();
        let states = first.chars().map(State::from_char).collect();
        let broken: Vec<usize> = second.split(',').map(|s| s.parse().unwrap()).collect();
        springs.add_line(states, broken);
    }

    let mut sum = 0;

    for i in 0..springs.state.len() {
        sum += springs.count(i);
    }
    println!("sum: {}", sum);


}
