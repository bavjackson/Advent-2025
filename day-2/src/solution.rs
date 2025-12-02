use std::sync::{Arc, Mutex};
use std::thread;

pub fn part1(input: &str) -> String {
    let total = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for range in input.split(",") {
        let counter = Arc::clone(&total);
        let current_range = String::from(range);
        let handle = thread::spawn(move || {
            let (bottom, top) = match current_range.split_once("-") {
                Some((left, right)) => (left.trim(), right.trim()),
                None => {
                    panic!("Error spliting range {}", current_range)
                }
            };

            let convert_lower = bottom.parse::<u64>();
            let convert_upper = top.parse::<u64>();

            let lower = match convert_lower {
                Ok(int) => int,
                Err(error) => panic!("Problem converting {} to int {:?}", bottom, error),
            };
            let upper = match convert_upper {
                Ok(int) => int,
                Err(error) => panic!("Problem converting {} to int {:?}", top, error),
            };

            for i in lower..=upper {
                if is_invalid_pt1(&i.to_string()) {
                    let num_result = counter.lock();
                    match num_result {
                        Ok(mut num) => *num += i,
                        Err(_error) => panic!("Could not get lock on counter"),
                    }
                }
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        let join_res = handle.join();
        match join_res {
            Ok(()) => {}
            Err(_error) => panic!("Error joining handle"),
        }
    }
    let total_res = total.lock();
    match total_res {
        Ok(res) => res.to_string(),
        Err(_error) => panic!("Unable to get lock on total"),
    }
}

pub fn part2(input: &str) -> String {
    let total = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for range in input.split(",") {
        let counter = Arc::clone(&total);
        let current_range = String::from(range);
        let handle = thread::spawn(move || {
            let (bottom, top) = match current_range.split_once("-") {
                Some((left, right)) => (left.trim(), right.trim()),
                None => {
                    panic!("Error spliting range {}", current_range)
                }
            };

            let convert_lower = bottom.parse::<u64>();
            let convert_upper = top.parse::<u64>();

            let lower = match convert_lower {
                Ok(int) => int,
                Err(error) => panic!("Problem converting {} to int {:?}", bottom, error),
            };
            let upper = match convert_upper {
                Ok(int) => int,
                Err(error) => panic!("Problem converting {} to int {:?}", top, error),
            };

            for i in lower..=upper {
                if is_invalid_pt2(&i.to_string()) {
                    let num_result = counter.lock();
                    match num_result {
                        Ok(mut num) => *num += i,
                        Err(_error) => panic!("Could not get lock on counter"),
                    }
                }
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        let join_res = handle.join();
        match join_res {
            Ok(()) => {}
            Err(_error) => panic!("Error joining handle"),
        }
    }
    let total_res = total.lock();
    match total_res {
        Ok(res) => res.to_string(),
        Err(_error) => panic!("Unable to get lock on total"),
    }
}

fn is_invalid_pt1(num: &str) -> bool {
    let length = num.len();
    if !length.is_multiple_of(2) {
        return false;
    }
    let upper = length / 2;

    num[..upper] == num[upper..]
}
fn is_invalid_pt2(num: &str) -> bool {
    // format!("{}{}", &num[1..], &num[..num.len() - 1]).contains(num)

    // the following code is faster than the above, possibly as it's able to break out of invalid
    // cases a bit earlier than the algorithm contains uses
    let length = num.len();
    'outer: for i in 2..=length {
        if !length.is_multiple_of(i) {
            continue;
        }
        let split = length / i;

        for j in 1..=(i - 1) {
            if num[split * (j - 1)..split * (j)] != num[split * j..split * (j + 1)] {
                continue 'outer;
            }
        }

        return true;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase<'a> {
        input: &'a str,
        expected: bool,
    }

    #[test]
    fn test_is_invalid_pt1() {
        let test_cases = vec![
            TestCase {
                input: "11",
                expected: true,
            },
            TestCase {
                input: "1234",
                expected: false,
            },
            TestCase {
                input: "12312",
                expected: false,
            },
        ];

        for case in test_cases {
            let result = is_invalid_pt1(case.input);
            assert_eq!(result, case.expected)
        }
    }

    #[test]
    fn test_is_invalid_pt2() {
        let test_cases = vec![
            TestCase {
                input: "11",
                expected: true,
            },
            TestCase {
                input: "121212",
                expected: true,
            },
            TestCase {
                input: "1212121212",
                expected: true,
            },
            TestCase {
                input: "123123123",
                expected: true,
            },
            TestCase {
                input: "1111111",
                expected: true,
            },
            TestCase {
                input: "1234",
                expected: false,
            },
            TestCase {
                input: "12312",
                expected: false,
            },
        ];

        for case in test_cases {
            let result = is_invalid_pt2(case.input);
            assert_eq!(result, case.expected)
        }
    }

    const EXAMPLE_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn part_1() {
        let result = part1(EXAMPLE_INPUT);

        assert_eq!(result, "1227775554");
    }

    #[test]
    fn part_2() {
        let result = part2(EXAMPLE_INPUT);

        assert_eq!(result, "4174379265");
    }
}
