mod q3;
mod q4;
mod q5;
mod q6;
mod q7;
mod read_input;

fn main() {
    let lines = read_input::read_input_lines("input7.txt");
    q7::part1(lines);
}

