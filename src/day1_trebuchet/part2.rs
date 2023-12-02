use std::collections::HashMap;

#[allow(dead_code)]
pub fn extract_calibration_value(input: &str) -> u32 {
    let mut digits: Vec<u32> = vec![];

    digits.first().unwrap() * 10 + digits.last().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("two1nine", 29)]
    #[case("eightwothree", 83)]
    #[case("abcone2threexyz", 13)]
    #[case("xtwone3four", 24)]
    #[case("4nineeightseven2", 42)]
    #[case("zoneight234", 14)]
    #[case("7pqrstsixteen", 76)]

    fn test_extract_calibration_value(#[case] input: &str, #[case] expected: u32) {
        assert_eq!(extract_calibration_value(input), expected);
    }

    #[test]
    fn solve_part1() {
        let input = include_str!("input.txt");
        let calibration_value = input.lines().map(extract_calibration_value).sum::<u32>();
        println!("Calibration value: {}", calibration_value);
    }
}
