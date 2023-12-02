use std::cmp::max;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{space0, u64},
    combinator::{map, value},
    multi::separated_list1,
    sequence::{delimited, pair, preceded, tuple},
    IResult, Parser,
};

#[derive(Debug, PartialEq, Clone)]
pub enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Default, Debug, Clone)]
pub struct CubeSet {
    red: u64,
    green: u64,
    blue: u64,
}

impl From<Vec<(u64, Color)>> for CubeSet {
    fn from(count_color_pairs: Vec<(u64, Color)>) -> Self {
        let mut cube_set = CubeSet::default();
        for (count, color) in count_color_pairs {
            match color {
                Color::Red => cube_set.red += count,
                Color::Green => cube_set.green += count,
                Color::Blue => cube_set.blue += count,
            }
        }
        cube_set
    }
}

#[allow(dead_code)]
impl CubeSet {
    fn power(&self) -> u64 {
        self.red * self.green * self.blue
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Game {
    game_id: u64,
    rounds: Vec<CubeSet>,
}

impl From<(u64, Vec<CubeSet>)> for Game {
    fn from((game_id, rounds): (u64, Vec<CubeSet>)) -> Self {
        Self { game_id, rounds }
    }
}

const BAG: CubeSet = CubeSet {
    red: 12,
    green: 13,
    blue: 14,
};

#[allow(dead_code)]
impl Game {
    pub fn is_possible(&self) -> bool {
        self.rounds
            .iter()
            .all(|round| round.red <= BAG.red && round.green <= BAG.green && round.blue <= BAG.blue)
    }

    pub fn min_cube_set(&self) -> CubeSet {
        self.rounds
            .iter()
            .fold(CubeSet::default(), |acc, curr| CubeSet {
                red: max(acc.red, curr.red),
                green: max(acc.green, curr.green),
                blue: max(acc.blue, curr.blue),
            })
    }
}

fn parse_game_id(input: &str) -> IResult<&str, u64> {
    delimited(tag("Game "), u64, tag(": "))(input)
}

fn parse_round(input: &str) -> IResult<&str, CubeSet> {
    let (rem, out) = separated_list1(
        tag(",").and(space0),
        pair(
            u64,
            preceded(
                space0,
                alt((
                    value(Color::Red, tag("red")),
                    value(Color::Green, tag("green")),
                    value(Color::Blue, tag("blue")),
                )),
            ),
        ),
    )(input)?;

    Ok((rem, CubeSet::from(out)))
}

fn parse_rounds(input: &str) -> IResult<&str, Vec<CubeSet>> {
    separated_list1(tag(";").and(space0), parse_round)(input)
}

#[allow(dead_code)]
fn parse_game(input: &str) -> IResult<&str, Game> {
    map(tuple((parse_game_id, parse_rounds)), Game::from)(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", true, 48)]
    #[case(
        "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
        true,
        12
    )]
    #[case(
        "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
        false,
        1560
    )]
    #[case(
        "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
        false,
        630
    )]
    #[case("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", true, 36)]

    fn test_game_is_possible(
        #[case] input: &str,
        #[case] possible: bool,
        #[case] min_cube_set_power: u64,
    ) {
        let (_, game) = parse_game(input).unwrap();
        assert_eq!(game.is_possible(), possible);
        assert_eq!(game.min_cube_set().power(), min_cube_set_power);
    }

    #[test]
    fn test_part1() {
        let input = include_str!("input.txt");

        let result = input
            .lines()
            .map(|line| parse_game(line).unwrap().1)
            .filter(Game::is_possible)
            .map(|game| game.game_id)
            .sum::<u64>();

        assert_eq!(result, 2551)
    }

    #[test]
    fn test_part2() {
        let input = include_str!("input.txt");

        let result = input
            .lines()
            .map(|line| parse_game(line).unwrap().1.min_cube_set().power())
            .sum::<u64>();

        assert_eq!(result, 62811)
    }
}
