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

#[derive(Default, Debug)]
pub struct Round {
    red: u64,
    green: u64,
    blue: u64,
}

impl From<Vec<(u64, Color)>> for Round {
    fn from(count_color_pairs: Vec<(u64, Color)>) -> Self {
        let mut round = Round::default();
        for (count, color) in count_color_pairs {
            match color {
                Color::Red => round.red += count,
                Color::Green => round.green += count,
                Color::Blue => round.blue += count,
            }
        }
        round
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Game {
    game_id: u64,
    rounds: Vec<Round>,
}

impl From<(u64, Vec<Round>)> for Game {
    fn from((game_id, rounds): (u64, Vec<Round>)) -> Self {
        Self { game_id, rounds }
    }
}

const MAX_ROUND: Round = Round {
    red: 12,
    green: 13,
    blue: 14,
};

impl Game {
    #[allow(dead_code)]
    pub fn is_possible(&self) -> bool {
        self.rounds.iter().all(|round| {
            round.red <= MAX_ROUND.red
                && round.green <= MAX_ROUND.green
                && round.blue <= MAX_ROUND.blue
        })
    }
}

fn parse_game_id(input: &str) -> IResult<&str, u64> {
    delimited(tag("Game "), u64, tag(": "))(input)
}

fn parse_round(input: &str) -> IResult<&str, Round> {
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

    Ok((rem, Round::from(out)))
}

fn parse_rounds(input: &str) -> IResult<&str, Vec<Round>> {
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
    #[case("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", true)]
    #[case(
        "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
        true
    )]
    #[case(
        "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
        false
    )]
    #[case(
        "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
        false
    )]
    #[case("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", true)]

    fn test_game_is_possible(#[case] input: &str, #[case] expected: bool) {
        let (_, game) = parse_game(input).unwrap();
        assert_eq!(game.is_possible(), expected);
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
}
