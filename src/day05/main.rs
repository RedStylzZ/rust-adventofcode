#![feature(iter_advance_by)]
#![feature(iter_array_chunks)]

use std::ops::Range;

use std::cmp::Ordering::*;

fn main() {
    let x = std::fs::read_to_string("input/day05_test.txt").unwrap();

    let solution = get_part_1(x.clone());

    println!("part 1: {}", solution);

    let sol2 = get_part_2(x);

    println!("part 2: {}", sol2);
}

fn get_part_1(input: String) -> i128 {
    let (first_line, rest) = input.split_once("\n\n").unwrap();

    let mut ids = first_line.split(" ");

    ids.advance_by(1).unwrap();

    let mut seeds = ids
        .map(|x| x.parse::<i128>().unwrap())
        .collect::<Vec<i128>>();

    for block in rest.split("\n\n") {
        let ranges = parse_block(block);

        for sed in &mut seeds {
            for range in &ranges {
                if range.source.contains(sed) {
                    *sed = range.dest.start + (*sed - range.source.start);
                    break;
                }
            }
        }
    }

    *seeds.iter().min().unwrap()
}

fn get_part_2(input: String) -> i128 {
    let (first_line, rest) = input.split_once("\n\n").unwrap();

    let mut ids = first_line.split(" ");

    ids.advance_by(1).unwrap();

    let mut seeds = ids
        .array_chunks::<2>()
        .map(|x| {
            let start = x[0].parse::<i128>().unwrap();
            let end_after = x[1].parse::<i128>().unwrap();
            start..start + end_after
        })
        .collect::<Vec<Range<i128>>>();

    let blocks = rest.split("\n\n");

    for block in blocks {
        let ranges = parse_block(block);
        seeds = get_schnitt(seeds, ranges);
    }
    seeds
        .iter()
        .min_by(|a, b| a.start.cmp(&b.start))
        .unwrap()
        .start
}

fn get_schnitt(mut source: Vec<Range<i128>>, map: Vec<MapSingle>) -> Vec<Range<i128>> {
    let mut i = 0;

    let mut result = Vec::new();

    while i < source.len() {
        let mut found_range = false;

        for map_single in &map {
            let cmp = source[i].compare(&map_single.source);

            let found = match cmp.get_matching_part() {
                Some(x) => x,
                None => continue,
            };

            let space_front = found.start - map_single.source.start;
            let length = found.end - found.start;

            let found =
                map_single.dest.start + space_front..map_single.dest.start + space_front + length;

            found_range = true;
            result.push(found);

            let left = cmp.get_original_not_matching_parts();
            source.extend(left);
        }

        if !found_range {
            result.push(source[i].clone());
        }
        i = i + 1;
    }

    result
}

trait RangeExt {
    fn compare(&self, other: &Range<i128>) -> RangeCmpResult;

    fn shift(&self, shift: i128) -> Range<i128>;
}

impl RangeExt for Range<i128> {
    fn compare(&self, other: &Range<i128>) -> RangeCmpResult {
        if self.is_empty() || other.is_empty() {
            // when empty always not included
            return RangeCmpResult::NotIncluded;
        }

        match (
            self.start.cmp(&other.start),
            self.end.cmp(&other.end),
            self.start.cmp(&other.end),
            self.end.cmp(&other.start),
        ) {
            (Equal, Equal, _, _) => RangeCmpResult::CompletelyTheSame,
            // Greater or Equal because the range is exclusive above
            (_, _, Greater | Equal, _) => RangeCmpResult::NotIncluded,
            (_, _, _, Less | Equal) => RangeCmpResult::NotIncluded,
            (Less, Less, _, _) => RangeCmpResult::EndIndluded {
                // + 1 because the range is inclusive at the beginning
                other_after: self.end + 1..other.end,
                original_part_which_is_not_included: self.start..other.start,
                original_included_part: other.start..self.end,
            },
            (Greater, Greater, _, _) => RangeCmpResult::StartIncluded {
                other_before: other.start..self.start,
                // + 1 because the range is inclusive at the beginning
                original_part_which_is_not_included: other.end + 1..self.end,
                original_included_part: self.start..other.end,
            },
            (Less, Greater, _, _) => RangeCmpResult::MiddleIncluded {
                original_included_part: other.start..other.end,
                original_before_not_included: self.start..other.start,
                // +1 because the range is inclusive at the beginning
                original_after_not_included: other.end + 1..self.end,
            },
            (Greater, Less, _, _) => RangeCmpResult::CompletelyIncluded {
                other_before: other.start..self.start,
                other_after: self.end + 1..other.end,
                original_included_part: self.start..self.end,
            },
            (Equal, Less, _, _) => RangeCmpResult::SameStartOriginalShorter {
                original_included_part: self.start..self.end,
                other_after_not_included: self.end + 1..other.end,
            },
            (Equal, Greater, _, _) => RangeCmpResult::SameStartOtherShorter {
                original_included_part: other.start..other.end,
                original_after_not_included: other.end + 1..self.end,
            },
            (Less, Equal, _, _) => RangeCmpResult::SameEndOtherShorter {
                original_included_part: other.start..other.end,
                original_before_not_included: self.start..other.start,
            },
            (Greater, Equal, _, _) => RangeCmpResult::SameEndOriginalShorter {
                original_included_part: self.start..self.end,
                other_before_not_included: other.start..self.start,
            },
        }
    }
    fn shift(&self, shift: i128) -> Range<i128> {
        self.start + shift..self.end + shift
    }
}

#[derive(Debug, Clone)]
enum RangeCmpResult {
    CompletelyTheSame,
    NotIncluded,
    CompletelyIncluded {
        other_before: Range<i128>,
        other_after: Range<i128>,
        original_included_part: Range<i128>,
    },
    EndIndluded {
        // The "rest" from the other range which is not included on the original one
        other_after: Range<i128>,
        original_part_which_is_not_included: Range<i128>,
        original_included_part: Range<i128>,
    },
    StartIncluded {
        other_before: Range<i128>,
        original_part_which_is_not_included: Range<i128>,
        original_included_part: Range<i128>,
    },
    MiddleIncluded {
        original_included_part: Range<i128>,
        original_before_not_included: Range<i128>,
        original_after_not_included: Range<i128>,
    },
    SameStartOriginalShorter {
        original_included_part: Range<i128>,
        other_after_not_included: Range<i128>,
    },
    SameStartOtherShorter {
        original_included_part: Range<i128>,
        original_after_not_included: Range<i128>,
    },
    SameEndOriginalShorter {
        original_included_part: Range<i128>,
        other_before_not_included: Range<i128>,
    },
    SameEndOtherShorter {
        original_included_part: Range<i128>,
        original_before_not_included: Range<i128>,
    },
}

impl RangeCmpResult {
    fn get_matching_part(&self) -> Option<Range<i128>> {
        match self {
            RangeCmpResult::CompletelyTheSame => None,
            RangeCmpResult::NotIncluded => None,
            RangeCmpResult::CompletelyIncluded {
                other_before: _,
                other_after: _,
                original_included_part,
            } => Some(original_included_part.clone()),
            RangeCmpResult::EndIndluded {
                // The "rest" from the other range which is not included on the original one
                other_after: _,
                original_part_which_is_not_included: _,
                original_included_part,
            } => Some(original_included_part.clone()),
            RangeCmpResult::StartIncluded {
                other_before: _,
                original_part_which_is_not_included: _,
                original_included_part,
            } => Some(original_included_part.clone()),
            RangeCmpResult::MiddleIncluded {
                original_included_part,
                original_before_not_included: _,
                original_after_not_included: _,
            } => Some(original_included_part.clone()),
            RangeCmpResult::SameStartOriginalShorter {
                original_included_part,
                other_after_not_included: _,
            } => Some(original_included_part.clone()),
            RangeCmpResult::SameStartOtherShorter {
                original_included_part,
                original_after_not_included: _,
            } => Some(original_included_part.clone()),
            RangeCmpResult::SameEndOriginalShorter {
                original_included_part,
                other_before_not_included: _,
            } => Some(original_included_part.clone()),
            RangeCmpResult::SameEndOtherShorter {
                original_included_part,
                original_before_not_included: _,
            } => Some(original_included_part.clone()),
        }
    }

    fn get_original_not_matching_parts(&self) -> Vec<Range<i128>> {
        match self {
            RangeCmpResult::CompletelyTheSame => vec![],
            RangeCmpResult::NotIncluded => vec![],
            RangeCmpResult::CompletelyIncluded {
                other_before: _,
                other_after: _,
                original_included_part: _,
            } => vec![],
            RangeCmpResult::EndIndluded {
                // The "rest" from the other range which is not included on the original one
                other_after: _,
                original_part_which_is_not_included,
                original_included_part: _,
            } => vec![original_part_which_is_not_included.clone()],
            RangeCmpResult::StartIncluded {
                other_before: _,
                original_part_which_is_not_included,
                original_included_part: _,
            } => vec![original_part_which_is_not_included.clone()],
            RangeCmpResult::MiddleIncluded {
                original_included_part: _,
                original_before_not_included,
                original_after_not_included,
            } => vec![
                original_before_not_included.clone(),
                original_after_not_included.clone(),
            ],
            RangeCmpResult::SameStartOriginalShorter {
                original_included_part: _,
                other_after_not_included,
            } => vec![],
            RangeCmpResult::SameStartOtherShorter {
                original_included_part: _,
                original_after_not_included,
            } => vec![original_after_not_included.clone()],
            RangeCmpResult::SameEndOriginalShorter {
                original_included_part: _,
                other_before_not_included,
            } => vec![],
            RangeCmpResult::SameEndOtherShorter {
                original_included_part: _,
                original_before_not_included,
            } => vec![original_before_not_included.clone()],
        }
    }
}

struct MapSingle {
    source: Range<i128>,
    dest: Range<i128>,
}

fn parse_block(block: &str) -> Vec<MapSingle> {
    let mut lines = block.lines();

    lines.advance_by(1).unwrap();

    lines
        .map(|x| {
            let x = x.split(" ");
            let mut x = x.map(|x| x.parse::<i128>().unwrap());

            let dest = x.next().unwrap();
            let source = x.next().unwrap();
            let length = x.next().unwrap();

            let source = source..source + length;
            let dest = dest..dest + length;
            MapSingle { source, dest }
        })
        .collect()
}
