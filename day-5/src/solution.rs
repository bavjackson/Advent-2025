use std::{
    cmp::{Reverse, max},
    ops::Range,
};

pub fn part1(ranges: &Vec<Range<u64>>, ids: Vec<u64>) -> String {
    let mut count = 0;
    for id in ids {
        let range = ranges.iter().find(|r| r.start <= id && r.end >= id);
        if let Some(_r) = range {
            count += 1
        }
    }

    count.to_string()
}

pub fn part2(ranges: &mut Vec<Range<u64>>) -> String {
    let mut total = 0;
    ranges.sort_unstable_by_key(|r| Reverse(r.start));

    let mut merged_ranges: Vec<Range<u64>> = vec![];
    while let Some(range) = ranges.pop() {
        if merged_ranges.is_empty() {
            merged_ranges.push(range);
        } else {
            let end_index = merged_ranges.len() - 1;
            if range.start <= merged_ranges[end_index].end {
                merged_ranges[end_index] =
                    merged_ranges[end_index].start..max(range.end, merged_ranges[end_index].end);
            } else {
                merged_ranges.push(range);
            }
        }
    }

    for range in merged_ranges {
        total += range.end - range.start + 1;
    }

    total.to_string()
}

pub fn parse_input(input: &str) -> (Vec<Range<u64>>, Vec<u64>) {
    let mut ranges = vec![];
    let mut ids = vec![];

    let mut on_ids = false;

    for line in input.lines() {
        if line.is_empty() {
            on_ids = true;
            continue;
        }

        if !on_ids {
            let split = line.split_once("-");
            let (left, right) = match split {
                Some(res) => res,
                None => panic!("Could not split {:?}", line),
            };

            let lower_convert = left.parse::<u64>();
            let upper_convert = right.parse::<u64>();
            let lower = match lower_convert {
                Ok(int) => int,
                Err(error) => panic!("Problem converting {:?} to int: {:?}", left, error),
            };
            let upper = match upper_convert {
                Ok(int) => int,
                Err(error) => panic!("Problem converting {:?} to int: {:?}", right, error),
            };

            ranges.push(lower..upper);
        } else {
            let id_convert = line.parse::<u64>();
            match id_convert {
                Ok(int) => ids.push(int),
                Err(error) => panic!("Could not convert {:?} to int: {:?}", line, error),
            }
        }
    }

    (ranges, ids)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"3-5
10-14
16-20
12-18

1
5
8
11
17
32"#;
    const EXAMPLE_INPUT_2: &str = r#"10-14
14-20

1
5
8
11
17
32"#;

    #[test]
    fn test_parse_input() {
        let (ranges, ids) = parse_input(EXAMPLE_INPUT);
        assert_eq!(ranges, vec![3..5, 10..14, 16..20, 12..18,]);
        assert_eq!(ids, vec![1, 5, 8, 11, 17, 32]);
    }

    #[test]
    fn part_1() {
        let (ranges, ids) = parse_input(EXAMPLE_INPUT);
        let result = part1(&ranges, ids);
        assert_eq!(result, "3")
    }

    #[test]
    fn part_2() {
        let (mut ranges, _ids) = parse_input(EXAMPLE_INPUT);
        let result = part2(&mut ranges);
        assert_eq!(result, "14")
    }
    #[test]
    fn part_2_edge_case() {
        let (mut ranges, _ids) = parse_input(EXAMPLE_INPUT_2);
        let result = part2(&mut ranges);
        assert_eq!(result, "11")
    }
}
