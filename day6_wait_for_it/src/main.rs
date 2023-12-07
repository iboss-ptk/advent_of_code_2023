use nom::{
    bytes::complete::tag,
    character::complete::{digit1, line_ending, space1, u64},
    combinator::map_res,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult, Parser,
};

#[derive(Debug, PartialEq, Clone, Copy, Default)]
struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn new(time: u64, distance: u64) -> Self {
        Self { time, distance }
    }

    /// Count all posible ways to hold in order for sure win
    /// constraint is:
    /// > (time - h) * h > distance; where `h` is hold time
    ///
    /// sovling this inequality with quadratic formula, we got:
    /// > (time +- sqrt(time^2 - 4 * distance)) / 2 = h
    /// as input are unsinged integers, the range of `h` is:
    /// > [h_lesser, h_greater]
    ///
    /// But it's a float number, so we need to round it to integer and count all posible ways.
    /// End result is:
    /// > floor(h_lesser) - ceil(h_greater) + 1
    fn possible_ways_to_win(&self) -> u64 {
        let time = self.time as f64;
        let distance = self.distance as f64;

        let sqrt_term = (time.powi(2) - 4.0 * distance).sqrt();
        let h_greater = floor_or_decrement((time + sqrt_term) / 2.0);
        let h_lesser = ceil_or_increment((time - sqrt_term) / 2.0);

        h_greater - h_lesser + 1
    }
}

fn floor_or_decrement(x: f64) -> u64 {
    (if x.fract() == 0.0 { x - 1.0 } else { x.floor() }) as u64
}

fn ceil_or_increment(x: f64) -> u64 {
    (if x.fract() == 0.0 { x + 1.0 } else { x.ceil() }) as u64
}

fn parse_races(input: &str) -> IResult<&str, Vec<Race>> {
    let (rem, (times, distances)) = separated_pair(
        preceded(tag("Time:").and(space1), separated_list1(space1, u64)),
        line_ending,
        preceded(tag("Distance:").and(space1), separated_list1(space1, u64)),
    )(input)?;

    let races = times
        .into_iter()
        .zip(distances.into_iter())
        .map(|(time, distance)| Race::new(time, distance))
        .collect();

    Ok((rem, races))
}

fn u64_ignore_spaces<'a>(input: &'a str) -> IResult<&'a str, u64> {
    map_res(separated_list1(space1, digit1), |nums: Vec<&str>| {
        nums.concat().parse::<u64>()
    })(input)
}

fn parse_race_ignore_spaces(input: &str) -> IResult<&str, Race> {
    let (rem, (times, distances)) = separated_pair(
        preceded(tag("Time:").and(space1), u64_ignore_spaces),
        line_ending,
        preceded(tag("Distance:").and(space1), u64_ignore_spaces),
    )(input)?;

    Ok((rem, Race::new(times, distances)))
}

fn main() {
    let input = include_str!("input.txt");

    // --- part 1 ---
    let (_, races) = parse_races(input).unwrap();
    let ways_product = races
        .iter()
        .map(|race| race.possible_ways_to_win())
        .product::<u64>();

    println!("Part 1: Product of possible ways to win: {}", ways_product);

    // --- part 2 ---
    let (_, race) = parse_race_ignore_spaces(input).unwrap();
    println!(
        "Part 2: possible ways to win {}",
        race.possible_ways_to_win()
    );
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn test_parse_races() {
        let input = include_str!("example.txt");
        let (_, races) = parse_races(input).unwrap();
        assert_eq!(
            races,
            vec![Race::new(7, 9), Race::new(15, 40), Race::new(30, 200)]
        );
    }

    #[test]
    fn test_parse_race_ignore_spaces() {
        let input = include_str!("example.txt");
        let (_, race) = parse_race_ignore_spaces(input).unwrap();
        assert_eq!(race, Race::new(71530, 940200));
    }

    #[rstest]
    #[case(Race::new(7, 9), 4)]
    #[case(Race::new(15, 40), 8)]
    #[case(Race::new(30, 200), 9)]
    fn test_possible_ways(#[case] race: Race, #[case] ways: u64) {
        assert_eq!(race.possible_ways_to_win(), ways);
    }
}
