use nom::{error, Finish};

use crate::cube_set::CubeSet;
use std::cmp::max;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Game {
    pub game_id: u64,
    pub rounds: Vec<CubeSet>,
}

impl From<(u64, Vec<CubeSet>)> for Game {
    fn from((game_id, rounds): (u64, Vec<CubeSet>)) -> Self {
        Self { game_id, rounds }
    }
}

#[allow(dead_code)]
impl Game {
    pub fn try_from_str(input: &str) -> Result<Game, error::Error<&str>> {
        let (rem, game) = parser::game(input).finish()?;
        assert!(rem.is_empty());
        Ok(game)
    }

    pub fn is_possible(&self, bag: &CubeSet) -> bool {
        self.rounds
            .iter()
            .all(|round| round.red <= bag.red && round.green <= bag.green && round.blue <= bag.blue)
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

mod parser {
    use super::*;
    use crate::cube_set::Color;
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{space0, u64},
        combinator::{map, value},
        multi::separated_list1,
        sequence::{delimited, pair, preceded, tuple},
        IResult, Parser,
    };

    pub fn game(input: &str) -> IResult<&str, Game> {
        map(tuple((game_id, rounds)), Game::from)(input)
    }

    fn game_id(input: &str) -> IResult<&str, u64> {
        delimited(tag("Game "), u64, tag(": "))(input)
    }

    fn round(input: &str) -> IResult<&str, CubeSet> {
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

    fn rounds(input: &str) -> IResult<&str, Vec<CubeSet>> {
        separated_list1(tag(";").and(space0), round)(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cube_set::BAG;
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
        let game = Game::try_from_str(input).unwrap();
        assert_eq!(game.is_possible(&BAG), possible);
        assert_eq!(game.min_cube_set().power(), min_cube_set_power);
    }
}
