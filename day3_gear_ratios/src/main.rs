use std::collections::BTreeMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{none_of, u64},
    combinator::value,
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
    alt((
        value(Value::gear_symbol(), tag("*")),
        value(Value::non_gear_symbol(), none_of("0123456789.*")),
    ))(input)
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum SymbolType {
    Gear,
    NonGear,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Value {
    Num(u64),
    Symbol(SymbolType),
}

impl Value {
    fn is_symbol(&self) -> bool {
        match self {
            Value::Symbol(_) => true,
            _ => false,
        }
    }

    fn is_gear(&self) -> bool {
        match self {
            Value::Symbol(SymbolType::Gear) => true,
            _ => false,
        }
    }

    fn gear_symbol() -> Self {
        Value::Symbol(SymbolType::Gear)
    }

    fn non_gear_symbol() -> Self {
        Value::Symbol(SymbolType::NonGear)
    }

    fn get_num(&self) -> Option<u64> {
        match self {
            Value::Num(n) => Some(*n),
            _ => None,
        }
    }
}

type SchematicIndex = (usize, (usize, usize));
#[derive(Debug)]
struct Schematic {
    value_map: BTreeMap<SchematicIndex, Value>,
    max_row: usize,
    gear_indices: Vec<SchematicIndex>,
}

impl Schematic {
    fn empty() -> Self {
        Self {
            value_map: BTreeMap::new(),
            max_row: 0,
            gear_indices: Vec::new(),
        }
    }

    fn new(input: &str) -> Self {
        let mut schematic = Self::empty();
        input.lines().enumerate().for_each(|(row, line)| {
            schematic.insert_row(row, line);
        });
        schematic
    }

    fn insert(&mut self, row: usize, span: (usize, usize), value: Value) {
        self.value_map.insert((row, span), value);
    }

    fn insert_row(&mut self, row: usize, input: &str) {
        let mut cursor = 0;
        let mut input = input;

        self.max_row = self.max_row.max(row);

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

                // index gear symbols
                if s.is_gear() {
                    self.gear_indices.push((row, (cursor, cursor + dist)));
                }

                self.insert(row, (cursor, cursor + dist), s);

                cursor += dist;
                input = rem;
            }
        }
    }

    fn row_iter(&self, row: usize) -> impl Iterator<Item = (&(usize, (usize, usize)), &Value)> {
        self.value_map
            .range((row, (0, 0))..(row + 1, (0, 0)))
            .into_iter()
    }

    fn any_symbol_in_span(&self, row: usize, (start, end): (usize, usize)) -> bool {
        let first_pos = (start, start + 1);
        let last_pos = (end - 1, end);
        self.value_map
            .range((row, first_pos)..=(row, last_pos))
            .any(|(_, v)| v.is_symbol())
    }

    fn nums_intersect_box_around(&self, idx: SchematicIndex) -> Vec<u64> {
        let (row, (kernel_start, kernel_end)) = idx;

        let intersection_by_row = |row: usize| {
            let span_to_kernel_start = (row, (0, kernel_start));
            let span_from_kernel_end = (row, (kernel_end, usize::MAX));

            self.value_map
                .range(span_to_kernel_start..=span_from_kernel_end)
                .filter_map(|((_, (num_start, num_end)), v)| {
                    let span_to_kernel = num_start < &kernel_start && num_end >= &kernel_start;
                    let span_from_kernel = num_start >= &kernel_start && num_start <= &kernel_end;
                    let intersecting = span_to_kernel || span_from_kernel;

                    v.get_num().filter(|_| intersecting)
                })
        };

        with_adjecents(row).flat_map(intersection_by_row).collect()
    }

    fn gear_ratio(&self) -> u64 {
        self.gear_indices
            .iter()
            .filter_map(|idx| {
                let nums = self.nums_intersect_box_around(*idx);
                if nums.len() == 2 {
                    Some(nums[0] * nums[1])
                } else {
                    None
                }
            })
            .sum()
    }

    fn get_num(&self, idx: SchematicIndex) -> Option<u64> {
        let v = self.value_map.get(&idx)?;
        v.get_num()
    }

    fn get_eligible_number(&self, idx: SchematicIndex) -> Option<u64> {
        let (row, (start, end)) = idx;

        let is_adjecent_to_symbol = with_adjecents(row).any(|row| {
            self.any_symbol_in_span(row, (start.saturating_sub(1), end.saturating_add(1)))
        });

        if is_adjecent_to_symbol {
            self.get_num(idx)
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
        (0..=self.max_row).into_iter().fold(0, |acc, row| {
            acc + self.eligible_numbers_by_row(row).iter().sum::<u64>()
        })
    }
}

fn with_adjecents(row: usize) -> impl Iterator<Item = usize> {
    row.saturating_sub(1)..=row.saturating_add(1)
}

fn main() {
    let schematic = Schematic::new(include_str!("input.txt"));

    println!("part 1: {}", schematic.sum_eligible_numbers());
    println!("part 2: {}", schematic.gear_ratio());
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("467..114..", 0, vec![((0, (0, 3)), Value::Num(467)), ((0, (5, 8)), Value::Num(114))])]
    #[case("...*......", 1, vec![((1, (3, 4)), Value::gear_symbol())])]
    #[case("..35..633.", 2, vec![((2, (2, 4)), Value::Num(35)), ((2, (6, 9)), Value::Num(633))])]
    #[case("......#...", 3, vec![((3, (6, 7)), Value::non_gear_symbol())])]
    #[case("617*......", 4, vec![((4, (0, 3)), Value::Num(617)), ((4, (3, 4)), Value::gear_symbol())])]
    #[case(".....+.58.", 5, vec![((5, (5, 6)), Value::non_gear_symbol()), ((5, (7, 9)), Value::Num(58))])]
    #[case("..592.....", 6, vec![((6, (2, 5)), Value::Num(592))])]
    #[case("......755.", 7, vec![((7, (6, 9)), Value::Num(755))])]
    #[case("...$.*....", 8, vec![((8, (3, 4)), Value::non_gear_symbol()), ((8, (5, 6)), Value::gear_symbol())])]
    #[case(".664.598.." , 9, vec![((9, (1, 4)), Value::Num(664)), ((9, (5, 8)), Value::Num(598))])]
    fn test_insert_rows(
        #[case] input: &str,
        #[case] row: usize,
        #[case] expected: Vec<((usize, (usize, usize)), Value)>,
    ) {
        let mut schematic = Schematic::empty();
        schematic.insert_row(row, input);
        assert_eq!(
            schematic.value_map.into_iter().collect::<Vec<_>>(),
            expected
        );
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

    #[test]
    fn test_gears_indices() {
        let schematic = Schematic::new(include_str!("example.txt"));
        assert_eq!(
            schematic.gear_indices,
            vec![(1, (3, 4)), (4, (3, 4)), (8, (5, 6)),]
        );
    }

    #[rstest]
    #[case((1, (3, 4)), vec![467, 35])]
    #[case((4, (3, 4)), vec![617])]
    #[case((8, (5, 6)), vec![755, 598])]
    fn test_nums_around(#[case] idx: SchematicIndex, #[case] expected: Vec<u64>) {
        let schematic = Schematic::new(include_str!("example.txt"));
        assert_eq!(schematic.nums_intersect_box_around(idx), expected);
    }

    #[test]
    fn test_gear_ratio() {
        let schematic = Schematic::new(include_str!("example.txt"));
        assert_eq!(schematic.gear_ratio(), 467835);
    }
}
