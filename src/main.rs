#![allow(dead_code)]

mod q3;
mod q4;
mod q5;
mod q6;
mod q7;
mod q8;
mod q9;
mod read_input;

fn main() {
    let lines = read_input::read_input_lines("input9.txt");
    q9::part2(lines);
}

