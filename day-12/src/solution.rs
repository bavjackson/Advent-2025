#[derive(Debug, Clone)]
struct Shape {
    shape: Vec<Vec<char>>,
    area: u32,
}

#[derive(Debug, Clone)]
struct Area {
    x: u32,
    y: u32,
    presents: Vec<u32>,
}

pub fn part1(input: &str) -> String {
    let mut count = 0;
    let (shapes, spaces) = parse_input(input);

    for space in spaces {
        let present_area = space
            .presents
            .iter()
            .enumerate()
            .fold(0, |acc, (i, p)| acc + p * shapes[i].area);

        if space.x * space.y >= present_area {
            count += 1;
        }
    }

    count.to_string()
}

fn parse_input(input: &str) -> (Vec<Shape>, Vec<Area>) {
    let mut shapes = vec![];
    let mut spaces = vec![];

    let mut is_shape = false;

    let mut shape: Shape = Shape {
        shape: vec![vec![]],
        area: 0,
    };

    for line in input.lines() {
        if line.is_empty() {
            shapes.push(shape);
            shape = Shape {
                shape: vec![vec![]],
                area: 0,
            };
            is_shape = false
        } else if line.as_bytes()[0].is_ascii_digit() && line.as_bytes()[1] == b':' {
            is_shape = true;
        } else if is_shape {
            shape.shape.push(line.chars().collect());
            shape.area += line
                .chars()
                .fold(0, |acc, c| acc + if c == '#' { 1 } else { 0 });
        } else {
            let mut area = Area {
                x: 0,
                y: 0,
                presents: vec![],
            };
            let split = line.split_once(": ");

            if let Some((l, r)) = split {
                let (x, y) = l.split_once("x").unwrap();
                area.x = x.parse::<u32>().unwrap();
                area.y = y.parse::<u32>().unwrap();

                area.presents = r
                    .split(" ")
                    .map(|num| num.parse::<u32>().unwrap())
                    .collect();
            }

            spaces.push(area);
        }
    }

    (shapes, spaces)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2"#;

    // #[test]
    // fn part_1() {
    //     let res = part1(EXAMPLE_INPUT);
    //     assert_eq!(res, "2");
    // }
}
