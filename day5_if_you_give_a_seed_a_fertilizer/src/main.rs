use btree_range_map::RangeMap;
use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, space1, u64},
    multi::{fold_many1, many1, separated_list1},
    sequence::{preceded, terminated, tuple},
    IResult, Parser,
};

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

    let (_, (seeds, mappings)) = tuple((
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

    let lowest_location = seeds
        .into_iter()
        .map(|seed| {
            mappings.iter().fold(seed, |src, mapping| {
                mapping.get(src).map(|m| m.translate(src)).unwrap_or(src)
            })
        })
        .min();

    println!("Part 1: Lowest location number: {:?}", lowest_location);
}
