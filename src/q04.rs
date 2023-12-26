use std::{
    fs::File,
    io::{BufReader, Lines},
};

#[derive(Debug)]
struct Game<'a> {
    winning_numbers: &'a [u32],
    guesses: &'a [u32],
    found: u32,
}

impl<'a> Game<'a> {
    fn new(winning_numbers: &'a [u32], guesses: &'a [u32]) -> Self {
        Game {
            winning_numbers,
            guesses,
            found: {
                let mut found = 0;
                for guess in guesses {
                    if winning_numbers.contains(guess) {
                        found += 1;
                    }
                }

                found
            },
        }
    }

    fn calculate_points(&self) -> u32 {
        if self.found > 2 {
            return 2_u32.pow(self.found - 1);
        }

        self.found
    }
}

pub fn part1(lines: Lines<BufReader<File>>) {
    let mut sum = 0;
    for line in lines {
        let line = line.unwrap();
        let spl = line
            .split(' ')
            .skip(2)
            .filter_map(|x| x.parse::<u32>().ok())
            .collect::<Vec<u32>>();
        let (winning_numbers, guesses) = spl.split_at(10);
        let game = Game::new(winning_numbers, guesses);
        sum += game.calculate_points();
    }

    println!("Sum: {}", sum);
}

pub fn part2(lines: Lines<BufReader<File>>) {
    let mut founds: Vec<u32> = Vec::new();
    for line in lines {
        let line = line.unwrap();
        let spl = line
            .split(' ')
            .skip(2)
            .filter_map(|x| x.parse::<u32>().ok())
            .collect::<Vec<u32>>();
        let (wn, gs) = spl.split_at(10);
        let game = Game::new(wn, gs);
        let found = game.found;
        founds.push(found);
    }

    let mut amounts: Vec<u32> = vec![1; founds.len()];

    for i in 0..founds.len() {
        for j in 1..founds[i] + 1 {
            amounts[i + j as usize] += amounts[i];
        }
    }

    let sum = amounts.iter().sum::<u32>();
    println!("Sum: {}", sum);

}
