use day1_trebuchet::solution::{part1, part2};

fn main() {
    let input = include_str!("input.txt");
    let solve = |extract: &dyn (Fn(&str) -> u32)| input.lines().map(extract).sum::<u32>();

    println!("part 1: {}", solve(&part1::extract_calibration_value));
    println!("part 2: {}", solve(&part2::extract_calibration_value));
}
