mod parser {
    use super::*;
    use nom::bytes::complete::tag;
    use nom::character::complete::{line_ending, space0, space1, u64};
    use nom::multi::{fold_many1, separated_list1};
    use nom::sequence::{delimited, preceded, separated_pair, terminated, tuple};
    use nom::{IResult, Parser};
    use std::collections::HashSet;

    pub fn scratchcards(input: &str) -> IResult<&str, Vec<ScratchCard>> {
        separated_list1(line_ending, scratchcard)(input)
    }

    pub fn scratchcard(input: &str) -> IResult<&str, ScratchCard> {
        let header = terminated(preceded(tag("Card").and(space1), u64), tag(":").and(space1));
        let nums_set_pair = separated_pair(nums_set, delimited(space1, tag("|"), space1), nums_set);

        let (rem, (id, (owned, winning))) = tuple((header, nums_set_pair))(input)?;

        Ok((rem, ScratchCard { id, owned, winning }))
    }

    fn nums_set(input: &str) -> IResult<&str, HashSet<u64>> {
        fold_many1(
            preceded(space0, u64),
            || HashSet::new(),
            |mut set: HashSet<u64>, num| {
                set.insert(num);
                set
            },
        )(input)
    }
}

use std::collections::{BTreeMap, HashSet};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ScratchCard {
    id: u64,
    owned: HashSet<u64>,
    winning: HashSet<u64>,
}

impl ScratchCard {
    fn points(&self) -> u64 {
        match (self.matches_count() as u32).checked_sub(1) {
            Some(exp) => 2u64.pow(exp),
            None => 0,
        }
    }

    fn matches(&self) -> HashSet<&u64> {
        self.owned.intersection(&self.winning).collect()
    }

    fn matches_count(&self) -> usize {
        self.matches().len()
    }
}

fn total_points(cards: &[ScratchCard]) -> u64 {
    cards.iter().map(ScratchCard::points).sum()
}

fn total_copies(cards: &[ScratchCard]) -> u64 {
    let mut total = 0;
    let mut cards_count: BTreeMap<usize, usize> =
        BTreeMap::from_iter(cards.iter().map(|c| (c.id as usize, 1)));

    for card in cards.iter() {
        let copies = card.matches_count();
        let current_card_count = cards_count[&(card.id as usize)].to_owned();

        total += current_card_count;

        let copy_range = (card.id + 1)..(card.id + 1 + copies as u64);
        for id in copy_range {
            cards_count
                .entry(id as usize)
                .and_modify(|card_count| *card_count += current_card_count);
        }
    }

    total as u64
}

fn main() {
    let input = include_str!("input.txt");
    let (_, cards) = parser::scratchcards(input).unwrap();

    println!("Part 1: total points = {}", total_points(&cards));
    println!("Part 2: total cards = {}", total_copies(&cards));
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53", 8)]
    #[case("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19", 2)]
    #[case("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1", 2)]
    #[case("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83", 1)]
    #[case("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36", 0)]
    #[case("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11", 0)]
    fn test_insert_rows(#[case] input: &str, #[case] expected: u64) {
        let (_, card) = parser::scratchcard(input).unwrap();

        assert_eq!(card.points(), expected);
    }

    #[test]
    fn test_total_points() {
        let input = include_str!("example.txt");
        let (_, cards) = parser::scratchcards(input).unwrap();
        assert_eq!(total_points(&cards), 13);
    }

    #[test]
    fn test_with_copies() {
        let input = include_str!("example.txt");
        let (_, cards) = parser::scratchcards(input).unwrap();

        assert_eq!(total_copies(&cards), 30);
    }
}
