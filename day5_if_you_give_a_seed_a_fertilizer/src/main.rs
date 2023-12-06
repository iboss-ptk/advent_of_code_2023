use btree_range_map::RangeMap;
use indicatif::ParallelProgressIterator;
use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, space1, u64},
    multi::{fold_many1, many1, separated_list1},
    sequence::{preceded, terminated, tuple},
    IResult, Parser,
};
use rayon::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Mapper {
    src_start: u64,
    dst_start: u64,
    range: u64,
}

impl Mapper {
    fn translate(&self, src: u64) -> u64 {
        assert!(src >= self.src_start);
        let res = self.dst_start + (src - self.src_start);
        assert!(res < self.dst_start + self.range);
        res
    }
}

fn seeds(input: &str) -> IResult<&str, Vec<u64>> {
    preceded(tag("seeds: "), separated_list1(space1, u64))(input)
}

fn mapping<'a>(label: &'a str) -> impl FnMut(&'a str) -> IResult<&'a str, RangeMap<u64, Mapper>> {
    preceded(
        many1(line_ending)
            .and(tag(label))
            .and(tag(" map:"))
            .and(many1(line_ending)),
        fold_many1(
            terminated(
                tuple((u64, preceded(space1, u64), preceded(space1, u64))),
                line_ending,
            ),
            RangeMap::new,
            |mut map: RangeMap<u64, Mapper>, (dst, src, range)| {
                map.insert(
                    src..src + range,
                    Mapper {
                        src_start: src,
                        dst_start: dst,
                        range,
                    },
                );

                map
            },
        ),
    )
}

fn main() {
    let input = include_str!("input.txt");
    let (_, (seeds, mappings)) = parse(input).unwrap();

    println!("Part 1: {:?}", find_lowest_location(&seeds, &mappings));

    // NOTE: Compute intensive solution, could be optimized further
    println!(
        "Part 2: {:?}",
        find_lowest_location(&seeds_from_range_pairs(seeds), &mappings)
    );
}

type Args = (Vec<u64>, Vec<RangeMap<u64, Mapper>>);

fn parse(input: &str) -> IResult<&str, Args> {
    let (rem, (seeds, mappings)) = tuple((
        seeds,
        tuple((
            mapping("seed-to-soil"),
            mapping("soil-to-fertilizer"),
            mapping("fertilizer-to-water"),
            mapping("water-to-light"),
            mapping("light-to-temperature"),
            mapping("temperature-to-humidity"),
            mapping("humidity-to-location"),
        )),
    ))(input)
    .unwrap();

    let mappings: Vec<RangeMap<u64, Mapper>> = vec![
        mappings.0, mappings.1, mappings.2, mappings.3, mappings.4, mappings.5, mappings.6,
    ];

    Ok((rem, (seeds, mappings)))
}

fn find_lowest_location(seeds: &[u64], mappings: &[RangeMap<u64, Mapper>]) -> Option<u64> {
    seeds
        .into_par_iter()
        .progress()
        .map(|seed| {
            let seed = *seed;
            mappings.iter().fold(seed, |src, mapping| {
                mapping.get(src).map(|m| m.translate(src)).unwrap_or(src)
            })
        })
        .min()
}

fn seeds_from_range_pairs(seeds: Vec<u64>) -> Vec<u64> {
    seeds
        .into_par_iter()
        .chunks(2)
        .progress()
        .flat_map(|chunk| {
            let start = chunk[0];
            let len = chunk[1];
            start..start + len
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("example.txt");
        let (_, (seeds, mappings)) = parse(input).unwrap();
        dbg!(&mappings);
        assert_eq!(find_lowest_location(&seeds, &mappings), Some(35));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("example.txt");
        let (_, (seeds, mappings)) = parse(input).unwrap();
        assert_eq!(
            find_lowest_location(&seeds_from_range_pairs(seeds), &mappings),
            Some(46)
        );
    }
}
