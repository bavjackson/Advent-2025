pub struct Column {
    numbers: Vec<u64>,
    operation: char,
}

type Grid = Vec<Vec<char>>;

pub fn part1(input: &str) -> String {
    let mut total = 0;
    let mut end_line = false;

    let mut cols: Vec<Column> = vec![];

    for (row, line) in input.lines().enumerate() {
        let first_char = &line.chars().next().unwrap();
        if ['*', '+'].contains(first_char) {
            end_line = true;
        }
        if !end_line {
            for (index, num) in line.split_whitespace().enumerate() {
                if row == 0 {
                    cols.push(Column {
                        numbers: vec![num.parse::<u64>().unwrap()],
                        operation: '_',
                    })
                } else {
                    cols[index].numbers.push(num.parse::<u64>().unwrap())
                }
            }
        } else {
            for (index, item) in line.split_whitespace().enumerate() {
                cols[index].operation = item.chars().next().unwrap();
            }
        }
    }

    for col in cols {
        let mut running_total = 0;
        if col.operation == '*' {
            running_total = 1;
        }

        for num in col.numbers {
            if col.operation == '*' {
                running_total *= num
            } else {
                running_total += num
            }
        }

        total += running_total;
    }

    total.to_string()
}

pub fn part2(input: &str) -> String {
    let mut total: u64 = 0;

    let mut grid: Grid = vec![];

    for line in input.lines() {
        let mut grid_line = vec![];
        for char in line.chars() {
            grid_line.push(char);
        }

        grid.push(grid_line);
    }

    let height = grid.len();
    let width = grid[0].len();

    let mut working_numbers: Vec<u64> = vec![];

    for i in (0..width).rev() {
        let mut digits = vec![];
        for j in 0..(height - 1) {
            let ch = grid[j][i];
            if ch != ' ' {
                digits.push(ch)
            }
        }
        if digits.is_empty() {
            continue;
        }

        let num = String::from_iter(digits.iter()).parse::<u64>().unwrap();
        working_numbers.push(num);

        let operation = grid[height - 1][i];
        if operation == '*' {
            total += working_numbers.iter().fold(1, |acc, c| acc * c) as u64;
            working_numbers = vec![];
        } else if operation == '+' {
            total += working_numbers.iter().fold(0, |acc, c| acc + c) as u64;
            working_numbers = vec![];
        }
    }

    total.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  "#;

    #[test]
    fn part_1() {
        let res = part1(EXAMPLE_INPUT);
        assert_eq!(res, "4277556");
    }

    #[test]
    fn part_2() {
        let res = part2(EXAMPLE_INPUT);
        assert_eq!(res, "3263827");
    }
}
