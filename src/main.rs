#![allow(dead_code)]

mod q03;
mod q04;
mod q05;
mod q06;
mod q07;
mod q08;
mod q09;
mod q10;
mod q11;
mod q12;
mod read_input;

fn main() {
    let lines = read_input::read_input_lines("input12.txt");
    q12::part1(lines);
}

