use std::cmp::{max, min};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Coord {
    x: u64,
    y: u64,
}

impl Coord {
    fn area_between(self, other: Self) -> u64 {
        (self.x.abs_diff(other.x) + 1) * (self.y.abs_diff(other.y) + 1)
    }
}

pub fn part1(input: &str) -> String {
    let coords: Vec<Coord> = input
        .lines()
        .map(|l| {
            let split = l.split_once(',');
            Coord {
                x: split.unwrap().0.parse::<u64>().unwrap(),
                y: split.unwrap().1.parse::<u64>().unwrap(),
            }
        })
        .collect();

    let length = coords.len();
    let mut max_area = 0;

    for i in 0..length {
        for j in (i + 1)..length {
            let area = coords[i].area_between(coords[j]);
            if area > max_area {
                max_area = area;
            }
        }
    }

    max_area.to_string()
}

pub fn part2(input: &str) -> String {
    let red_tiles: Vec<Coord> = input
        .lines()
        .map(|l| {
            let split = l.split_once(',');
            Coord {
                x: split.unwrap().0.parse::<u64>().unwrap(),
                y: split.unwrap().1.parse::<u64>().unwrap(),
            }
        })
        .collect();

    let mut lines = vec![];

    let length = red_tiles.len();

    for i in 0..length {
        let j = if i + 1 < length { i + 1 } else { 0 };

        lines.push((red_tiles[i], red_tiles[j]))
    }

    let mut max_area = 0;

    for i in 0..length {
        for j in (i + 1)..length {
            let coord_1 = red_tiles[i];
            let coord_2 = red_tiles[j];

            let mins = Coord {
                x: min(coord_1.x, coord_2.x),
                y: min(coord_1.y, coord_2.y),
            };
            let maxes = Coord {
                x: max(coord_1.x, coord_2.x),
                y: max(coord_1.y, coord_2.y),
            };

            if lines.iter().any(|l| {
                let (x1, y1) = (min(l.0.x, l.1.x), min(l.0.y, l.1.y));
                let (x2, y2) = (max(l.0.x, l.1.x), max(l.0.y, l.1.y));

                ((x1 == x2)
                    && (x1 > mins.x && x1 < maxes.x && !((y2 <= mins.y) || (y1 >= maxes.y))))
                    || ((y1 == y2)
                        && (y1 > mins.y && y1 < maxes.y && !((x2 <= mins.x) || (x1 >= maxes.x))))
            }) {
                continue;
            }

            let area = coord_1.area_between(coord_2);
            if area > max_area {
                max_area = area;
            }
        }
    }

    max_area.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3"#;

    #[test]
    fn part_1() {
        let res = part1(EXAMPLE_INPUT);
        assert_eq!(res, "50");
    }

    #[test]
    fn part_2() {
        let res = part2(EXAMPLE_INPUT);
        assert_eq!(res, "24");
        // panic!("fail to see print output");
    }
}
