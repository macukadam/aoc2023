#![allow(dead_code)]

mod q3;
mod q4;
mod q5;
mod q6;
mod q7;
mod q8;
mod read_input;

fn main() {
    let lines = read_input::read_input_lines("input8.txt");
    q8::part2(lines);
}

