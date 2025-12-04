use std::ops::Add;

type Grid = Vec<Vec<char>>;

#[derive(Debug)]
struct Coord {
    x: i32,
    y: i32,
}

impl Add for Coord {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// const UP: Coord = Coord { x: 0, y: -1 };
// const DOWN: Coord = Coord { x: 0, y: 1 };
// const LEFT: Coord = Coord { x: -1, y: 0 };
// const RIGHT: Coord = Coord { x: 1, y: 0 };
static DIRECTIONS: &[Coord] = &[
    Coord { x: 0, y: -1 },
    Coord { x: -1, y: -1 },
    Coord { x: 1, y: -1 },
    Coord { x: -1, y: 0 },
    Coord { x: 1, y: 0 },
    Coord { x: 0, y: 1 },
    Coord { x: -1, y: 1 },
    Coord { x: 1, y: 1 },
];

pub fn part1(grid: &Grid) -> String {
    let mut total = 0;
    let width = grid[0].len() as i32;
    let height = grid.len() as i32;

    for (y, row) in grid.iter().enumerate() {
        for (x, value) in row.iter().enumerate() {
            if *value != '@' {
                continue;
            }
            let coord = Coord {
                x: x as i32,
                y: y as i32,
            };
            let neighbours = DIRECTIONS.iter().map(|c| Coord {
                x: c.x + coord.x,
                y: c.y + coord.y,
            });
            let count = neighbours.fold(0, |acc, c| {
                if c.x < 0 || c.y < 0 || c.x >= width || c.y >= height {
                    return acc;
                }
                if grid[c.y as usize][c.x as usize] == '@' {
                    return acc + 1;
                }
                acc
            });
            if count < 4 {
                total += 1;
            }
        }
    }

    total.to_string()
}

pub fn part2(grid: &mut Grid) -> String {
    let mut total = 0;
    let width = grid[0].len() as i32;
    let height = grid.len() as i32;

    let mut current_loop_count = -1;
    while current_loop_count != 0 {
        let mut to_remove: Vec<Coord> = vec![];
        current_loop_count = 0;
        for (y, row) in grid.iter().enumerate() {
            for (x, value) in row.iter().enumerate() {
                if *value != '@' {
                    continue;
                }
                let coord = Coord {
                    x: x as i32,
                    y: y as i32,
                };
                let neighbours = DIRECTIONS.iter().map(|c| Coord {
                    x: c.x + coord.x,
                    y: c.y + coord.y,
                });
                let count = neighbours.fold(0, |acc, c| {
                    if c.x < 0 || c.y < 0 || c.x >= width || c.y >= height {
                        return acc;
                    }
                    if grid[c.y as usize][c.x as usize] == '@' {
                        return acc + 1;
                    }
                    acc
                });
                if count < 4 {
                    to_remove.push(coord);
                    current_loop_count += 1;
                }
            }
        }
        total += current_loop_count;
        for c in to_remove.iter() {
            grid[c.y as usize][c.x as usize] = '.';
        }
    }

    total.to_string()
}

pub fn input_to_grid(input: &str) -> Grid {
    let mut grid: Grid = vec![];

    for line in input.lines() {
        grid.push(line.chars().collect());
    }

    grid
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."#;

    struct InputTestCase<'a> {
        input: &'a str,
        expected: Grid,
    }

    #[test]
    fn test_input_to_grid() {
        let test_cases = vec![InputTestCase {
            input: r#"@.@
...
@@@"#,
            expected: vec![
                vec!['@', '.', '@'],
                vec!['.', '.', '.'],
                vec!['@', '@', '@'],
            ],
        }];
        for case in test_cases {
            let res = input_to_grid(case.input);
            assert_eq!(res, case.expected);
        }
    }

    #[test]
    fn part_1() {
        let grid = input_to_grid(EXAMPLE_INPUT);
        let result = part1(&grid);

        assert_eq!(result, "13");
    }

    #[test]
    fn part_2() {
        let mut grid = input_to_grid(EXAMPLE_INPUT);
        let result = part2(&mut grid);

        assert_eq!(result, "43");
    }
}
