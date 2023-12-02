use crate::conversion_trie;

#[allow(dead_code)]
pub fn extract_calibration_value(input: &str) -> u32 {
    let tree = conversion_trie! {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9
    };

    let mut digits: Vec<u32> = vec![];

    let mut rem_input = input;

    while !rem_input.is_empty() {
        let mut chars_iter = rem_input.chars();
        if let Some(digit) = chars_iter.next().expect("non empty string").to_digit(10) {
            digits.push(digit);
        } else if let Some((value, _)) = tree.convert_head(rem_input) {
            digits.push(value);
        }
        rem_input = chars_iter.as_str();
    }
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
    #[case("sevenxx", 77)]
    #[case("xxfivexx", 55)]
    #[case("six7sixqrdfive3twonehsk", 61)]

    fn test_extract_calibration_value(#[case] input: &str, #[case] expected: u32) {
        assert_eq!(extract_calibration_value(input), expected);
    }

    #[test]
    fn solve_part2() {
        let input = include_str!("input.txt");
        let calibration_value = input.lines().map(extract_calibration_value).sum::<u32>();
        assert_eq!(calibration_value, 53312);
    }
}
