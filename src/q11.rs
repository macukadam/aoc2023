use core::fmt;
use std::{
    fmt::{Display, Formatter},
    fs::File,
    io::{BufReader, Lines},
};

struct Universe {
    grid: Vec<Vec<char>>,
    galaxies: Vec<(usize, usize)>,
}

impl Display for Universe {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for row in self.grid.iter() {
            for c in row.iter() {
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Universe {
    fn new() -> Self {
        Self {
            grid: Vec::new(),
            galaxies: Vec::new(),
        }
    }

    fn add_row(&mut self, row: Vec<char>) {
        self.grid.push(row);
    }

    fn add_col(&mut self, col: Vec<char>) {
        for (i, c) in col.iter().enumerate() {
            self.grid[i].push(*c);
        }
    }

    fn expand_universe(&mut self, times: usize) {
        let mut index = 0;
        while index < self.grid.len() {
            if self.grid[index].iter().all(|c| *c == '.') {
                for _ in 0..times {
                    self.grid.insert(index, vec!['.'; self.grid[0].len()]);
                    index += 1;
                }
            }

            index += 1;
        }

        index = 0;

        while index < self.grid[0].len() {
            if self.grid.iter().all(|row| row[index] == '.') {
                for _ in 0..times {
                    self.grid.iter_mut().for_each(|row| row.insert(index, '.'));
                    index += 1;
                }
            }

            index += 1;
        }
    }

    fn insert_galaxies(&mut self) {
        self.galaxies.clear();
        for i in 0..self.grid.len() {
            for j in 0..self.grid[0].len() {
                if self.grid[i][j] == '#' {
                    self.galaxies.push((i, j));
                }
            }
        }
    }

    fn distance_between_galaxies(&self, g1: usize, g2: usize) -> usize {
        let (x1, y1) = self.galaxies[g1];
        let (x2, y2) = self.galaxies[g2];
        x1.abs_diff(x2) + y1.abs_diff(y2)
    }
}

pub fn part1(lines: Lines<BufReader<File>>) {
    let mut universe = Universe::new();

    for line in lines {
        let line = line.unwrap();
        let mut row = Vec::new();
        line.chars().for_each(|c| row.push(c));
        universe.add_row(row);
    }

    universe.expand_universe(0);
    universe.insert_galaxies();

    let mut total_distance = 0;

    for i in 0..universe.galaxies.len() {
        for j in i + 1..universe.galaxies.len() {
            total_distance += universe.distance_between_galaxies(i, j);
        }
    }

    println!("Total distance: {}", total_distance);
}

pub fn part2(lines: Lines<BufReader<File>>) {
    let mut universe = Universe::new();

    for line in lines {
        let line = line.unwrap();
        let mut row = Vec::new();
        line.chars().for_each(|c| row.push(c));
        universe.add_row(row);
    }

    universe.expand_universe(0);
    universe.insert_galaxies();

    let mut first_iter = 0;

    for i in 0..universe.galaxies.len() {
        for j in i + 1..universe.galaxies.len() {
            first_iter += universe.distance_between_galaxies(i, j);
        }
    }

    universe.expand_universe(1);
    universe.insert_galaxies();

    let mut second_iter = 0;

    for i in 0..universe.galaxies.len() {
        for j in i + 1..universe.galaxies.len() {
            second_iter += universe.distance_between_galaxies(i, j);
        }
    }

    let diff =  second_iter - first_iter;

    let total_distance = 999_999 * diff + first_iter;
    println!("Total distance: {}", total_distance);
}
