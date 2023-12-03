use std::collections::{BTreeMap, HashSet};

use nom::{
    bytes::complete::tag,
    character::complete::{none_of, u64},
    multi::many0,
    IResult,
};

fn num(input: &str) -> IResult<&str, Value> {
    let (rem, n) = u64(input)?;
    Ok((rem, Value::Num(n)))
}

fn periods_count(input: &str) -> IResult<&str, usize> {
    let (rem, periods) = many0(tag("."))(input)?;
    Ok((rem, periods.len()))
}

fn symbol(input: &str) -> IResult<&str, Value> {
    let (rem, _) = none_of("0123456789.")(input)?;
    Ok((rem, Value::Symbol))
}

#[derive(Debug, PartialEq, Eq)]
enum Value {
    Num(u64),
    Symbol,
}

impl Value {
    fn is_symbol(&self) -> bool {
        self == &Value::Symbol
    }
}

type SchematicIndex = (usize, (usize, usize));
#[derive(Debug)]
struct Schematic(BTreeMap<SchematicIndex, Value>);

impl Schematic {
    fn empty() -> Self {
        Self(BTreeMap::new())
    }

    fn new(input: &str) -> Self {
        let mut schematic = Self::empty();
        input.lines().enumerate().for_each(|(row, line)| {
            schematic.insert_row(row, line);
        });
        schematic
    }

    fn insert(&mut self, row: usize, span: (usize, usize), value: Value) {
        self.0.insert((row, span), value);
    }

    fn insert_row(&mut self, row: usize, input: &str) {
        let mut cursor = 0;
        let mut input = input;

        while !input.is_empty() {
            if let Ok((rem, dist)) = periods_count(input) {
                cursor += dist;
                input = rem;
            }

            if let Ok((rem, n)) = num(input) {
                let dist = input.len() - rem.len();
                self.insert(row, (cursor, cursor + dist), n);
                cursor += dist;
                input = rem;
            }

            if let Ok((rem, s)) = symbol(input) {
                let dist = input.len() - rem.len();
                self.insert(row, (cursor, cursor + dist), s);
                cursor += dist;
                input = rem;
            }
        }
    }

    fn row_iter(&self, row: usize) -> impl Iterator<Item = (&(usize, (usize, usize)), &Value)> {
        self.0.range((row, (0, 0))..(row + 1, (0, 0))).into_iter()
    }

    fn any_symbol_in_span(&self, row: usize, (start, end): (usize, usize)) -> bool {
        let first_pos = (start, start + 1);
        let last_pos = (end - 1, end);
        self.0
            .range((row, first_pos)..=(row, last_pos))
            .any(|(_, v)| v.is_symbol())
    }

    fn get_number(&self, idx: SchematicIndex) -> Option<u64> {
        let v = self.0.get(&idx)?;
        match v {
            Value::Num(n) => Some(*n),
            _ => None,
        }
    }

    fn get_eligible_number(&self, idx: SchematicIndex) -> Option<u64> {
        let (row, (start, end)) = idx;

        let mut rows = row.saturating_sub(1)..=row.saturating_add(1);
        let is_adjecent_to_symbol = rows.any(|row| {
            self.any_symbol_in_span(row, (start.saturating_sub(1), end.saturating_add(1)))
        });

        if is_adjecent_to_symbol {
            self.get_number(idx)
        } else {
            None
        }
    }

    fn eligible_numbers_by_row(&self, row: usize) -> Vec<u64> {
        self.row_iter(row)
            .filter_map(|(idx, _)| self.get_eligible_number(*idx))
            .collect()
    }

    fn sum_eligible_numbers(&self) -> u64 {
        self.0
            .keys()
            .map(|(row, _)| row)
            .collect::<HashSet<_>>()
            .into_iter()
            .fold(0, |acc, row| {
                acc + self.eligible_numbers_by_row(*row).iter().sum::<u64>()
            })
    }
}
fn main() {
    let schematic = Schematic::new(include_str!("input.txt"));

    println!("part 1: {}", schematic.sum_eligible_numbers());
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("467..114..", 0, vec![((0, (0, 3)), Value::Num(467)), ((0, (5, 8)), Value::Num(114))])]
    #[case("...*......", 1, vec![((1, (3, 4)), Value::Symbol)])]
    #[case("..35..633.", 2, vec![((2, (2, 4)), Value::Num(35)), ((2, (6, 9)), Value::Num(633))])]
    #[case("......#...", 3, vec![((3, (6, 7)), Value::Symbol)])]
    #[case("617*......", 4, vec![((4, (0, 3)), Value::Num(617)), ((4, (3, 4)), Value::Symbol)])]
    #[case(".....+.58.", 5, vec![((5, (5, 6)), Value::Symbol), ((5, (7, 9)), Value::Num(58))])]
    #[case("..592.....", 6, vec![((6, (2, 5)), Value::Num(592))])]
    #[case("......755.", 7, vec![((7, (6, 9)), Value::Num(755))])]
    #[case("...$.*....", 8, vec![((8, (3, 4)), Value::Symbol), ((8, (5, 6)), Value::Symbol)])]
    #[case(".664.598.." , 9, vec![((9, (1, 4)), Value::Num(664)), ((9, (5, 8)), Value::Num(598))])]
    fn test_insert_rows(
        #[case] input: &str,
        #[case] row: usize,
        #[case] expected: Vec<((usize, (usize, usize)), Value)>,
    ) {
        let mut schematic = Schematic::empty();
        schematic.insert_row(row, input);
        assert_eq!(schematic.0.into_iter().collect::<Vec<_>>(), expected);
    }

    #[rstest]
    #[case(0 ,vec![467])]
    #[case(1 ,vec![])]
    #[case(2 ,vec![35, 633])]
    #[case(3 ,vec![])]
    #[case(4 ,vec![617])]
    #[case(5 ,vec![])]
    #[case(6 ,vec![592])]
    #[case(7 ,vec![755])]
    #[case(8 ,vec![])]
    #[case(9 ,vec![664, 598])]
    fn test_symbol_adjecent_number_for_row(#[case] row: usize, #[case] expected: Vec<u64>) {
        let schematic = Schematic::new(include_str!("example.txt"));
        assert_eq!(schematic.eligible_numbers_by_row(row), expected);
    }

    #[test]
    fn test_sum_eligible_numbers() {
        let schematic = Schematic::new(include_str!("example.txt"));
        assert_eq!(schematic.sum_eligible_numbers(), 4361);
    }
}
