use std::collections::BTreeMap;

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

#[derive(Debug)]
struct Schematic(BTreeMap<(usize, (usize, usize)), Value>);

impl Schematic {
    fn new() -> Self {
        Self(BTreeMap::new())
    }

    fn insert(&mut self, row: usize, span: (usize, usize), value: Value) {
        self.0.insert((row, span), value);
    }

    fn insert_row(&mut self, row: usize, input: &str) {
        let mut cursor = 0;
        let mut input = input.clone();

        while !input.is_empty() {
            let (rem, dist) = periods_count(input).unwrap();
            cursor += dist;

            input = rem;

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
}
fn main() {
    let mut schematic = Schematic::new();

    let input = include_str!("example.txt");
    input.lines().enumerate().for_each(|(row, line)| {
        schematic.insert_row(row, line);
    });

    dbg!(schematic);
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
        let mut schematic = Schematic::new();
        schematic.insert_row(row, input);
        assert_eq!(schematic.0.into_iter().collect::<Vec<_>>(), expected);
    }
}
