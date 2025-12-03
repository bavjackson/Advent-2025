pub fn part1(input: &str) -> String {
    let mut total = 0;

    for line in input.lines() {
        total += get_joltage_pt1(line);
    }

    total.to_string()
}

fn get_joltage_pt1(batteries: &str) -> u64 {
    let mut max1 = '0';
    let mut max1_index = 0;
    let mut max2 = '0';

    for (index, battery) in batteries[..batteries.len() - 1].chars().enumerate() {
        // compare the chars as the byte values are also comparable
        if battery > max1 {
            max1 = battery;
            max1_index = index;
        }
    }

    for battery in batteries[(max1_index + 1)..].chars() {
        if battery > max2 {
            max2 = battery;
        }
    }

    let number: String = String::from_iter([max1, max2]);
    let convert_result = number.parse::<u64>();

    match convert_result {
        Ok(int) => int,
        Err(error) => panic!("Problem converting {} to int {:?}", number, error),
    }
}

pub fn part2(input: &str) -> String {
    let mut total = 0;

    for line in input.lines() {
        total += get_joltage_pt2(line, 12);
    }

    total.to_string()
}

fn get_joltage_pt2(batteries: &str, battery_count: usize) -> u64 {
    let length = batteries.len();
    let mut previous_index = 0;
    let mut digits = vec!['0'; battery_count];

    (0..battery_count).for_each(|i| {
        let range = length - battery_count + i;

        let mut battery = '0';
        let previous = previous_index;

        for (index, bat) in batteries[previous_index..=(range)].chars().enumerate() {
            if bat > battery {
                battery = bat;
                previous_index = previous + index + 1;
            }
        }

        digits[i] = battery;
    });

    let number: String = String::from_iter(digits);
    let convert_result = number.parse::<u64>();

    match convert_result {
        Ok(int) => int,
        Err(error) => panic!("Problem converting {} to int {:?}", number, error),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase<'a> {
        input: &'a str,
        count: usize,
        expected: u64,
    }

    #[test]
    fn test_get_joltage_pt1() {
        let test_cases = vec![
            TestCase {
                input: "987654321111111",
                count: 2,
                expected: 98,
            },
            TestCase {
                input: "811111111111119",
                count: 2,
                expected: 89,
            },
        ];

        for case in test_cases {
            let res = get_joltage_pt1(case.input);
            assert_eq!(res, case.expected)
        }
    }

    #[test]
    fn test_get_joltage_pt2() {
        let test_cases = vec![
            TestCase {
                input: "987654321111111",
                count: 12,
                expected: 987654321111,
            },
            TestCase {
                input: "811111111111119",
                count: 12,
                expected: 811111111119,
            },
        ];

        for case in test_cases {
            let res = get_joltage_pt2(case.input, case.count);
            assert_eq!(res, case.expected)
        }
    }

    const EXAMPLE_INPUT: &str = r#"987654321111111
811111111111119
234234234234278
818181911112111"#;

    #[test]
    fn part_1() {
        let result = part1(EXAMPLE_INPUT);

        assert_eq!(result, "357");
    }

    #[test]
    fn part_2() {
        let result = part2(EXAMPLE_INPUT);

        assert_eq!(result, "3121910778619");
    }
}
