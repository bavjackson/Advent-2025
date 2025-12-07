use std::collections::{HashMap, HashSet};

type GridLine = Vec<char>;
type Grid = Vec<GridLine>;

type Coords = Vec<Coord>;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Coord {
    x: usize,
    y: usize,
}

pub fn part1(input: &str) -> String {
    let mut total = 0;

    let (grid, start) = input_to_grid(input);

    let mut path_coords: Coords = vec![start];

    for (y, line) in grid.iter().enumerate() {
        if y == 0 {
            continue;
        }
        let (count, new_coords) = get_splits_for_line(line, path_coords);

        total += count;
        path_coords = new_coords;
    }

    total.to_string()
}

pub fn part2(input: &str) -> String {
    let (grid, start) = input_to_grid(input);

    let mut cache: HashMap<Coord, u64> = HashMap::new();

    let total = get_timelines(&grid, start, &mut cache);

    total.to_string()
}

fn get_splits_for_line(gridline: &GridLine, coords: Coords) -> (u32, Coords) {
    let mut split_count = 0;
    let mut split_set = HashSet::new();

    for coord in coords.iter() {
        if gridline[coord.x] == '^' {
            split_count += 1;
            if coord.x + 1 < gridline.len() {
                split_set.insert(Coord {
                    x: coord.x + 1,
                    y: coord.y,
                });
            }
            if coord.x > 0 {
                split_set.insert(Coord {
                    x: coord.x - 1,
                    y: coord.y,
                });
            }
        } else {
            split_set.insert(coord.clone());
        }
    }

    (split_count, split_set.iter().cloned().collect())
}

fn get_timelines(grid: &Grid, coord: Coord, cache: &mut HashMap<Coord, u64>) -> u64 {
    if cache.contains_key(&coord) {
        return *cache.get(&coord).unwrap();
    }
    if coord.y >= grid.len() {
        return 1;
    }

    let mut total = 0;

    let ch = grid[coord.y][coord.x];

    if ch == '^' {
        total += get_timelines(
            grid,
            Coord {
                x: coord.x - 1,
                y: coord.y + 2,
            },
            cache,
        );
        total += get_timelines(
            grid,
            Coord {
                x: coord.x + 1,
                y: coord.y + 2,
            },
            cache,
        );
    } else {
        total += get_timelines(
            grid,
            Coord {
                x: coord.x,
                y: coord.y + 2,
            },
            cache,
        )
    }

    cache.insert(coord, total);
    total
}

fn input_to_grid(input: &str) -> (Grid, Coord) {
    let mut grid: Grid = vec![];
    let mut start = Coord { x: 0, y: 0 };

    for (y, line) in input.lines().enumerate() {
        let mut grid_line = vec![];
        for (x, ch) in line.chars().enumerate() {
            grid_line.push(ch);
            if ch == 'S' {
                start.x = x;
                start.y = y;
            }
        }

        grid.push(grid_line);
    }

    (grid, start)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."#;

    const EXAMPLE_INPUT_2: &str = r#"...S...
.......
...^...
.......
..^.^..
.......
...^...
......."#;

    #[test]
    fn part_1() {
        let res = part1(EXAMPLE_INPUT);
        assert_eq!(res, "21");
    }

    #[test]
    fn part_2() {
        let res = part2(EXAMPLE_INPUT);
        assert_eq!(res, "40");
        let res = part2(EXAMPLE_INPUT_2);
        assert_eq!(res, "6");
    }
}
