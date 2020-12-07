#[allow(dead_code)]
mod days;

fn main() {
    let input = include_str!("input.txt");
    days::day7::run(input);
}
