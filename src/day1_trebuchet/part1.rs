#[allow(dead_code)]
pub fn extract_calibration_value(input: &str) -> u32 {
    let digits: Vec<u32> = input.chars().filter_map(|c| c.to_digit(10)).collect();

    digits.first().unwrap() * 10 + digits.last().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("1abc2", 12)]
    #[case("pqr3stu8vwx", 38)]
    #[case("a1b2c3d4e5f", 15)]
    #[case("treb7uchet", 77)]

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
